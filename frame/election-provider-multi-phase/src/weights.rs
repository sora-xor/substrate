// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_election_provider_multi_phase
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-20, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_election_provider_multi_phase
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/election-provider-multi-phase/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_election_provider_multi_phase.
pub trait WeightInfo {
	fn on_initialize_nothing() -> Weight;
	fn on_initialize_open_signed() -> Weight;
	fn on_initialize_open_unsigned_with_snapshot() -> Weight;
	fn finalize_signed_phase_accept_solution() -> Weight;
	fn finalize_signed_phase_reject_solution() -> Weight;
	fn on_initialize_open_unsigned_without_snapshot() -> Weight;
	fn elect_queued(v: u32, t: u32, a: u32, d: u32) -> Weight;
	fn submit(c: u32, ) -> Weight;
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32) -> Weight;
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32) -> Weight;
}

/// Weights for pallet_election_provider_multi_phase using the Substrate node and recommended
/// hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn on_initialize_nothing() -> Weight {
		(33_392_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
	}
	fn on_initialize_open_signed() -> Weight {
		(115_659_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_open_unsigned_with_snapshot() -> Weight {
		(114_970_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn finalize_signed_phase_accept_solution() -> Weight {
		(51_442_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn finalize_signed_phase_reject_solution() -> Weight {
		(23_160_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_initialize_open_unsigned_without_snapshot() -> Weight {
		(24_101_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn elect_queued(_v: u32, _t: u32, _a: u32, _d: u32) -> Weight {
		(6_038_989_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn submit(c: u32, ) -> Weight {
		(78_972_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((308_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 12_000
			.saturating_add((3_572_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 42_000
			.saturating_add((23_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 12_000
			.saturating_add((11_529_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 63_000
			.saturating_add((3_333_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 7_000
			.saturating_add((3_647_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 23_000
			.saturating_add((390_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 7_000
			.saturating_add((9_614_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 35_000
			.saturating_add((3_405_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize_nothing() -> Weight {
		(33_392_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
	}
	fn on_initialize_open_signed() -> Weight {
		(115_659_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn on_initialize_open_unsigned_with_snapshot() -> Weight {
		(114_970_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn finalize_signed_phase_accept_solution() -> Weight {
		(51_442_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn finalize_signed_phase_reject_solution() -> Weight {
		(23_160_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn on_initialize_open_unsigned_without_snapshot() -> Weight {
		(24_101_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn elect_queued(_v: u32, _t: u32, _a: u32, _d: u32) -> Weight {
		(6_038_989_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn submit(c: u32, ) -> Weight {
		(78_972_000 as Weight)
			// Standard Error: 16_000
			.saturating_add((308_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn submit_unsigned(v: u32, t: u32, a: u32, d: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 12_000
			.saturating_add((3_572_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 42_000
			.saturating_add((23_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 12_000
			.saturating_add((11_529_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 63_000
			.saturating_add((3_333_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn feasibility_check(v: u32, t: u32, a: u32, d: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 7_000
			.saturating_add((3_647_000 as Weight).saturating_mul(v as Weight))
			// Standard Error: 23_000
			.saturating_add((390_000 as Weight).saturating_mul(t as Weight))
			// Standard Error: 7_000
			.saturating_add((9_614_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 35_000
			.saturating_add((3_405_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
	}
}
