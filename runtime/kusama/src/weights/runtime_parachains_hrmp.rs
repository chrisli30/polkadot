// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::hrmp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_parachains_hrmp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::hrmp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::hrmp::WeightInfo for WeightInfo<T> {
	// Storage: Paras ParaLifecycles (r:2 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	fn hrmp_init_open_channel() -> Weight {
		(36_911_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Paras ParaLifecycles (r:1 w:0)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:1 w:0)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	fn hrmp_accept_open_channel() -> Weight {
		(33_665_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Hrmp HrmpChannels (r:1 w:0)
	// Storage: Hrmp HrmpCloseChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:1)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	fn hrmp_close_channel() -> Weight {
		(31_988_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Hrmp HrmpIngressChannelsIndex (r:128 w:127)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:1 w:1)
	// Storage: Hrmp HrmpChannels (r:127 w:127)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:0 w:1)
	// Storage: Hrmp HrmpChannelContents (r:0 w:127)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:0 w:1)
	/// The range of component `i` is `[0, 127]`.
	/// The range of component `e` is `[0, 127]`.
	fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 18_000
			.saturating_add((10_100_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 18_000
			.saturating_add((10_101_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(e as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(e as Weight)))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:0)
	// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
	// Storage: Paras ParaLifecycles (r:4 w:0)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:2 w:2)
	// Storage: Hrmp HrmpAcceptedChannelRequestCount (r:2 w:2)
	// Storage: Hrmp HrmpChannels (r:0 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_open(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 28_000
			.saturating_add((23_324_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((7 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((6 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Hrmp HrmpCloseChannelRequestsList (r:1 w:0)
	// Storage: Hrmp HrmpChannels (r:2 w:2)
	// Storage: Hrmp HrmpEgressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpIngressChannelsIndex (r:2 w:2)
	// Storage: Hrmp HrmpCloseChannelRequests (r:0 w:2)
	// Storage: Hrmp HrmpChannelContents (r:0 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn force_process_hrmp_close(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 15_000
			.saturating_add((13_318_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((5 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequests (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequestCount (r:1 w:1)
	/// The range of component `c` is `[0, 128]`.
	fn hrmp_cancel_open_request(c: u32, ) -> Weight {
		(26_997_000 as Weight)
			// Standard Error: 0
			.saturating_add((58_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Hrmp HrmpOpenChannelRequestsList (r:1 w:1)
	// Storage: Hrmp HrmpOpenChannelRequests (r:2 w:2)
	/// The range of component `c` is `[0, 128]`.
	fn clean_open_channel_requests(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 7_000
			.saturating_add((3_869_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}
