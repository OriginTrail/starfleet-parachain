
//! Autogenerated weights for `pallet_xc_asset_config`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-03, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/neuroweb
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_xc_asset_config
// --steps
// 20
// --repeat
// 10
// --extrinsic
// *
// --output
// .

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_xc_asset_config`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xc_asset_config::WeightInfo for WeightInfo<T> {
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	fn register_asset_location() -> Weight {
		Weight::from_parts(15_599_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XcAssetConfig AssetLocationToId (r:1 w:0)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	fn set_asset_units_per_second() -> Weight {
		Weight::from_parts(14_326_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:1 w:2)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:2)
	fn change_existing_asset_location() -> Weight {
		Weight::from_parts(20_459_000, 0)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	fn remove_payment_asset() -> Weight {
		Weight::from_parts(11_221_000, 0)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XcAssetConfig AssetIdToLocation (r:1 w:1)
	// Storage: EVM AccountCodes (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationUnitsPerSecond (r:0 w:1)
	// Storage: XcAssetConfig AssetLocationToId (r:0 w:1)
	fn remove_asset() -> Weight {
		Weight::from_parts(17_804_000, 0)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
}