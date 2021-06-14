// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Testing utils used by the RPC tests.

use futures::{executor, FutureExt, future::BoxFuture};
use sp_core::traits::SpawnNamed;
use substrate_test_runtime_client::{prelude::*, runtime::{H256, Block, Header}};
use sp_rpc::{list::ListOrValue, number::NumberOrHex};

type SignedBlock = sp_runtime::generic::SignedBlock<Block>;

// Executor shared by all tests.
//
// This shared executor is used to prevent `Too many open files` errors
// on systems with a lot of cores.
lazy_static::lazy_static! {
	static ref EXECUTOR: executor::ThreadPool = executor::ThreadPool::new()
		.expect("Failed to create thread pool executor for tests");
}

/// Executor for use in testing
#[derive(Clone, Debug)]
pub struct TaskExecutor;

impl TaskExecutor {
	pub fn execute(fut: BoxFuture<'static, ()>) -> std::result::Result<(), ()> {
		EXECUTOR.spawn_ok(fut);
		Ok(())
	}
}

impl SpawnNamed for TaskExecutor {
	fn spawn_blocking(&self, _: &'static str, _: BoxFuture<'static, ()>) {
		todo!()
	}

	fn spawn(&self, _name: &'static str, fut: BoxFuture<'static, ()>) {
		TaskExecutor::execute(fut).expect("future must succeed in tests")
	}
}

jsonrpsee::proc_macros::rpc_client_api! {
	pub(crate) Chain {
		#[rpc(method = "chain_getHeader", positional_params)]
		fn header(hash: Option<H256>) -> Header;
		#[rpc(method = "chain_getBlock", positional_params)]
		fn block(hash: Option<H256>) -> SignedBlock;
		#[rpc(method = "chain_getBlockHash", positional_params)]
		fn block_hash(hash: Option<ListOrValue<NumberOrHex>>) -> ListOrValue<Option<H256>>;
		#[rpc(method = "chain_getFinalizedHead")]
		fn finalized_head() -> H256;
	}
}
