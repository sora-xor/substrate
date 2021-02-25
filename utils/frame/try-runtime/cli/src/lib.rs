// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! `Structopt`-ready struct for `try-runtime`.

use parity_scale_codec::{Decode, Encode};
use std::{fmt::Debug, str::FromStr};
use sc_service::Configuration;
use sc_cli::{CliConfiguration, ExecutionStrategy, WasmExecutionMethod};
use sc_executor::NativeExecutor;
use sc_service::NativeExecutionDispatch;
use sp_state_machine::{StateMachine, Backend};
use sp_externalities::Extensions;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sp_core::storage::{StorageData, StorageKey, well_known_keys};
use remote_externalities::{Builder, Mode, CacheConfig, OfflineConfig, OnlineConfig, TestExternalities};

/// Various commands to try out the new runtime, over configurable states.
///
/// For now this only assumes running the `on_runtime_upgrade` hooks.
#[derive(Debug, structopt::StructOpt)]
pub struct TryRuntimeCmd {
	/// The shared parameters
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: sc_cli::SharedParams,

	/// The state to use to run the migration. Should be a valid FILE or HTTP URI.
	#[structopt(short, long, default_value = "http://localhost:9933")]
	pub state: State,

	#[structopt(short, long, default_value = "OnRuntimeUpgrade")]
	pub api_command: ApiCommands,

	/// The execution strategy that should be used for benchmarks
	#[structopt(
		long = "execution",
		value_name = "STRATEGY",
		possible_values = &ExecutionStrategy::variants(),
		case_insensitive = true,
		default_value = "Native",
	)]
	pub execution: ExecutionStrategy,

	/// Method for executing Wasm runtime code.
	#[structopt(
		long = "wasm-execution",
		value_name = "METHOD",
		possible_values = &WasmExecutionMethod::enabled_variants(),
		case_insensitive = true,
		default_value = "Interpreted"
	)]
	pub wasm_method: WasmExecutionMethod,
}

/// The state to use for a migration dry-run.
#[derive(Debug)]
pub enum State {
	/// A snapshot. Inner value is a file path.
	Snap(String),

	/// A live chain. Inner value is the HTTP uri.
	Live(String),
}

impl FromStr for State {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.get(..7) {
			// could use Url crate as well, but lets keep it simple for now.
			Some("http://") => Ok(State::Live(s.to_string())),
			Some("file://") => s
				.split("//")
				.collect::<Vec<_>>()
				.get(1)
				.map(|s| State::Snap(s.to_string()))
				.ok_or("invalid file URI"),
			_ => Err("invalid format. Must be a valid HTTP or File URI"),
		}
	}
}

/// The `try-runtime` api method to call into.
#[derive(Debug)]
pub enum ApiCommands {
	/// Call into the on-runtime-upgrade api.
	OnRuntimeUpgrade,
	/// Call into the on-initialize api for the given number of blocks. The number can be set via a
	/// dash following the name, e.g. `InitializeBlock-100`. No dash postfix is treated as
	/// `InitializeBlock-1`
	///
	/// Note that the first one might trigger runtime upgrades as well.
	InitializeBlock(u64),
}

impl FromStr for ApiCommands {
	type Err = &'static str;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "OnRuntimeUpgrade" {
			Ok(ApiCommands::OnRuntimeUpgrade)
		} else if s == "InitializeBlock" {
			Ok(ApiCommands::InitializeBlock(1))
		} else if s.starts_with("InitializeBlock-") {
			let number = s
				.split("-")
				.collect::<Vec<_>>()
				.get(1)
				.expect("input is know to have one '-'; length will be at least 2; qed")
				.parse::<u64>()
				.map_err(|_| "failed to parse: {:?}")?;
			Ok(ApiCommands::InitializeBlock(number))
		} else {
			Err("Unknown command")
		}
	}
}

impl TryRuntimeCmd {
	pub async fn run<B, ExecDispatch>(&self, config: Configuration) -> sc_cli::Result<()>
	where
		B: BlockT,
		ExecDispatch: NativeExecutionDispatch + 'static,
	{
		match self.api_command {
			ApiCommands::InitializeBlock(count) => {
				self.initialize_block::<B, ExecDispatch>(config, count).await
			}
			ApiCommands::OnRuntimeUpgrade => {
				self.on_runtime_upgrade::<B, ExecDispatch>(config).await
			}
		}
	}
}

impl TryRuntimeCmd {
	/// Try and read the block number from the given externalities. The result type is SCALE
	/// encoded.
	///
	/// # Warning
	///
	/// This function assumes a frame-based storage layout.
	fn get_block_number(ext: &TestExternalities) -> sc_cli::Result<Vec<u8>> {
		let mut block_number_key = [0u8; 32];
		let prefix = sp_io::hashing::twox_128(b"System");
		let postfix = sp_io::hashing::twox_128(b"Number");
		block_number_key[..16].copy_from_slice(&prefix);
		block_number_key[16..].copy_from_slice(&postfix);

		ext.backend
			.storage(&block_number_key)
			.and_then(|maybe_bytes| {
				maybe_bytes.ok_or("no value found for block number!".to_string())
			})
			.map_err(|e| {
				format!(
					"failed to read block number from state. This is probably not a FRAME based \
					 chain {:?}",
					e
				)
				.into()
			})
	}

	/// Common setup operations for all commands.
	///
	/// Builds remote-externalities, injects the code of the current binary into it, and return it
	/// with a executor instance that can be used with state machine.
	async fn setup<ExecDispatch>(
		&self,
		config: Configuration,
	) -> sc_cli::Result<(TestExternalities, NativeExecutor<ExecDispatch>)>
	where
		ExecDispatch: NativeExecutionDispatch + 'static,
	{
		let spec = config.chain_spec;
		let genesis_storage = spec.build_storage()?;

		let code = StorageData(
			genesis_storage
				.top
				.get(well_known_keys::CODE)
				.expect("code key must exist in genesis storage; qed")
				.to_vec(),
		);
		let code_key = StorageKey(well_known_keys::CODE.to_vec());

		let wasm_method = self.wasm_method;
		// don't really care about these -- use the default values.
		let max_runtime_instances = config.max_runtime_instances;
		let heap_pages = config.default_heap_pages;
		let executor = NativeExecutor::<ExecDispatch>::new(
			wasm_method.into(),
			heap_pages,
			max_runtime_instances,
		);

		let ext = {
			let builder = match &self.state {
				State::Snap(file_path) => Builder::new().mode(Mode::Offline(OfflineConfig {
					cache: CacheConfig { name: file_path.into(), ..Default::default() },
				})),
				State::Live(http_uri) => Builder::new().mode(Mode::Online(OnlineConfig {
					uri: http_uri.into(),
					..Default::default()
				})),
			};

			// inject the code into this ext.
			builder.inject(&[(code_key, code)]).build().await
		};

		Ok((ext, executor))
	}
}

impl TryRuntimeCmd {
	async fn on_runtime_upgrade<B, ExecDispatch>(&self, config: Configuration) -> sc_cli::Result<()>
	where
		B: BlockT,
		ExecDispatch: NativeExecutionDispatch + 'static,
	{
		let mut changes = Default::default();
		let execution = self.execution;
		let (ext, executor) = self.setup::<ExecDispatch>(config).await?;

		let encoded_result = StateMachine::<_, _, NumberFor<B>, _>::new(
			&ext.backend,
			None,
			&mut changes,
			&executor,
			"TryRuntime_on_runtime_upgrade",
			&[],
			ext.extensions,
				&sp_state_machine::backend::BackendRuntimeCode::new(&ext.backend)
			.runtime_code()?,
			sp_core::testing::TaskExecutor::new(),
		)
		.execute(execution.into())
		.map_err(|e| format!("failed to execute 'TryRuntime_on_runtime_upgrade' due to {:?}", e))?;

		let (weight, total_weight) = <(u64, u64) as Decode>::decode(&mut &*encoded_result)
			.map_err(|e| format!("failed to decode output due to {:?}", e))?;
		log::info!(
			"try-runtime executed without errors. Consumed weight = {}, total weight = {} ({})",
			weight,
			total_weight,
			weight as f64 / total_weight as f64
		);

		Ok(())
	}

	async fn initialize_block<B, ExecDispatch>(
		&self,
		config: Configuration,
		count: u64,
	) -> sc_cli::Result<()>
	where
		B: BlockT,
		ExecDispatch: NativeExecutionDispatch + 'static,
	{
		let mut changes = Default::default();
		let execution = self.execution;
		let (mut ext, executor) = self.setup::<ExecDispatch>(config).await?;

		let block_number_bytes = Self::get_block_number(&ext)?;
		let mut now = <NumberFor<B> as Decode>::decode(&mut &*block_number_bytes)
			.map_err(|_| "failed to decode block number.")?;

		for _ in 0..count {
			let encoded_result = StateMachine::<_, _, NumberFor<B>, _>::new(
				&ext.backend,
				None,
				&mut changes,
				&executor,
				"TryRuntime_initialize_block",
				&(now + 1u32.into()).encode(),
				Extensions::default(),
				&sp_state_machine::backend::BackendRuntimeCode::new(&ext.backend).runtime_code()?,
				sp_core::testing::TaskExecutor::new(),
			)
			.execute(execution.into())
			.map_err(|e| {
				format!("failed to execute 'TryRuntime_initialize_block' due to {:?}", e)
			})?;

			let (weight, total_weight) = <(u64, u64) as Decode>::decode(&mut &*encoded_result)
				.map_err(|e| format!("failed to decode output due to {:?}", e))?;
			log::info!(
				"TryRuntime_initialize_block executed without errors. Consumed weight = {}, total \
				 weight = {} ({})",
				weight,
				total_weight,
				weight as f64 / total_weight as f64
			);

			now += 1u32.into();
			ext.commit_all().unwrap();
		}
		Ok(())
	}
}

impl CliConfiguration for TryRuntimeCmd {
	fn shared_params(&self) -> &sc_cli::SharedParams {
		&self.shared_params
	}

	fn chain_id(&self, _is_dev: bool) -> sc_cli::Result<String> {
		Ok(match self.shared_params.chain {
			Some(ref chain) => chain.clone(),
			None => "dev".into(),
		})
	}
}
