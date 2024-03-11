// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-bn-ce5rx-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/substrate-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./substrate/frame/vesting/src/weights.rs
// --header=./substrate/HEADER-APACHE2
// --template=./substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_vesting`.
pub trait WeightInfo {
	fn vest_locked(l: u32, s: u32, ) -> Weight;
	fn vest_unlocked(l: u32, s: u32, ) -> Weight;
	fn vest_other_locked(l: u32, s: u32, ) -> Weight;
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight;
	fn vested_transfer(l: u32, s: u32, ) -> Weight;
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight;
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
	fn force_remove_vesting_schedule(l: u32, s: u32, ) -> Weight;
}

/// Weights for `pallet_vesting` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 31_890_000 picoseconds.
		Weight::from_parts(31_128_476, 4764)
			// Standard Error: 1_193
			.saturating_add(Weight::from_parts(63_760, 0).saturating_mul(l.into()))
			// Standard Error: 2_124
			.saturating_add(Weight::from_parts(57_247, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 33_958_000 picoseconds.
		Weight::from_parts(33_537_005, 4764)
			// Standard Error: 1_354
			.saturating_add(Weight::from_parts(60_604, 0).saturating_mul(l.into()))
			// Standard Error: 2_410
			.saturating_add(Weight::from_parts(51_378, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 33_243_000 picoseconds.
		Weight::from_parts(32_552_805, 4764)
			// Standard Error: 1_413
			.saturating_add(Weight::from_parts(67_137, 0).saturating_mul(l.into()))
			// Standard Error: 2_515
			.saturating_add(Weight::from_parts(70_000, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 35_396_000 picoseconds.
		Weight::from_parts(35_774_949, 4764)
			// Standard Error: 1_404
			.saturating_add(Weight::from_parts(55_349, 0).saturating_mul(l.into()))
			// Standard Error: 2_497
			.saturating_add(Weight::from_parts(39_228, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `555 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 68_441_000 picoseconds.
		Weight::from_parts(68_961_970, 4764)
			// Standard Error: 2_509
			.saturating_add(Weight::from_parts(83_345, 0).saturating_mul(l.into()))
			// Standard Error: 4_464
			.saturating_add(Weight::from_parts(124_158, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `6196`
		// Minimum execution time: 70_190_000 picoseconds.
		Weight::from_parts(72_355_984, 6196)
			// Standard Error: 2_295
			.saturating_add(Weight::from_parts(59_968, 0).saturating_mul(l.into()))
			// Standard Error: 4_084
			.saturating_add(Weight::from_parts(87_090, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 33_911_000 picoseconds.
		Weight::from_parts(33_243_006, 4764)
			// Standard Error: 1_199
			.saturating_add(Weight::from_parts(70_586, 0).saturating_mul(l.into()))
			// Standard Error: 2_214
			.saturating_add(Weight::from_parts(60_985, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 36_242_000 picoseconds.
		Weight::from_parts(35_812_994, 4764)
			// Standard Error: 1_351
			.saturating_add(Weight::from_parts(68_256, 0).saturating_mul(l.into()))
			// Standard Error: 2_496
			.saturating_add(Weight::from_parts(66_171, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn force_remove_vesting_schedule(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `555 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 37_817_000 picoseconds.
		Weight::from_parts(37_397_127, 4764)
			// Standard Error: 1_665
			.saturating_add(Weight::from_parts(72_049, 0).saturating_mul(l.into()))
			// Standard Error: 3_075
			.saturating_add(Weight::from_parts(60_973, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 31_890_000 picoseconds.
		Weight::from_parts(31_128_476, 4764)
			// Standard Error: 1_193
			.saturating_add(Weight::from_parts(63_760, 0).saturating_mul(l.into()))
			// Standard Error: 2_124
			.saturating_add(Weight::from_parts(57_247, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 33_958_000 picoseconds.
		Weight::from_parts(33_537_005, 4764)
			// Standard Error: 1_354
			.saturating_add(Weight::from_parts(60_604, 0).saturating_mul(l.into()))
			// Standard Error: 2_410
			.saturating_add(Weight::from_parts(51_378, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 33_243_000 picoseconds.
		Weight::from_parts(32_552_805, 4764)
			// Standard Error: 1_413
			.saturating_add(Weight::from_parts(67_137, 0).saturating_mul(l.into()))
			// Standard Error: 2_515
			.saturating_add(Weight::from_parts(70_000, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 35_396_000 picoseconds.
		Weight::from_parts(35_774_949, 4764)
			// Standard Error: 1_404
			.saturating_add(Weight::from_parts(55_349, 0).saturating_mul(l.into()))
			// Standard Error: 2_497
			.saturating_add(Weight::from_parts(39_228, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `555 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 68_441_000 picoseconds.
		Weight::from_parts(68_961_970, 4764)
			// Standard Error: 2_509
			.saturating_add(Weight::from_parts(83_345, 0).saturating_mul(l.into()))
			// Standard Error: 4_464
			.saturating_add(Weight::from_parts(124_158, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `6196`
		// Minimum execution time: 70_190_000 picoseconds.
		Weight::from_parts(72_355_984, 6196)
			// Standard Error: 2_295
			.saturating_add(Weight::from_parts(59_968, 0).saturating_mul(l.into()))
			// Standard Error: 4_084
			.saturating_add(Weight::from_parts(87_090, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 33_911_000 picoseconds.
		Weight::from_parts(33_243_006, 4764)
			// Standard Error: 1_199
			.saturating_add(Weight::from_parts(70_586, 0).saturating_mul(l.into()))
			// Standard Error: 2_214
			.saturating_add(Weight::from_parts(60_985, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 36_242_000 picoseconds.
		Weight::from_parts(35_812_994, 4764)
			// Standard Error: 1_351
			.saturating_add(Weight::from_parts(68_256, 0).saturating_mul(l.into()))
			// Standard Error: 2_496
			.saturating_add(Weight::from_parts(66_171, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn force_remove_vesting_schedule(l: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `555 + l * (25 ±0) + s * (36 ±0)`
		//  Estimated: `4764`
		// Minimum execution time: 37_817_000 picoseconds.
		Weight::from_parts(37_397_127, 4764)
			// Standard Error: 1_665
			.saturating_add(Weight::from_parts(72_049, 0).saturating_mul(l.into()))
			// Standard Error: 3_075
			.saturating_add(Weight::from_parts(60_973, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
