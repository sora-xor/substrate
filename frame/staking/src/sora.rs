pub use crate::weights::WeightInfo;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{Perbill, Percent, RuntimeDebug};
use sp_std::prelude::*;

pub use crate::pallet::{pallet::*, *};

pub use sp_std::time::Duration;
use traits::MultiCurrency;

/// CurrencyId type for MultiCurrency.
pub type MultiCurrencyIdOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
	<T as frame_system::Config>::AccountId,
>>::CurrencyId;

/// The balance type for MultiCurrency.
pub type MultiCurrencyBalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub trait ValBurnedNotifier<A> {
	fn notify_val_burned(amount: A);
}

pub struct ValRewardCurve {
	/// The time it will take for the reward to reach `MinValBurnedPercentageReward`
	/// from `MaxValBurnedPercentageReward` and flatline there.
	pub duration_to_reward_flatline: Duration,

	/// Minimum percentage of the VAL burned in the active era, that can be used as a reward pool.
	pub min_val_burned_percentage_reward: Percent,

	/// Maximum percentage of the VAL burned in the active era, that can be used as a reward pool.
	pub max_val_burned_percentage_reward: Percent,
}

#[allow(non_snake_case)]
impl ValRewardCurve {
	pub fn current_reward_coefficient(&self, duration_since_genesis: Duration) -> Perbill {
		if duration_since_genesis >= self.duration_to_reward_flatline {
			Perbill::from_percent(self.min_val_burned_percentage_reward.deconstruct() as u32)
		} else {
			let min_percentage = self.min_val_burned_percentage_reward.deconstruct() as u64;
			let max_percentage = self.max_val_burned_percentage_reward.deconstruct() as u64;
			let five_years = self.duration_to_reward_flatline.as_secs();
			let elapsed = duration_since_genesis.as_secs();
			Perbill::from_rational(
				max_percentage * (five_years - elapsed) + min_percentage * elapsed,
				100_u64 * five_years,
			)
		}
	}
}

#[derive(PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct DurationWrapper {
	secs: u64,
	nanos: u32,
}

impl From<Duration> for DurationWrapper {
	fn from(duration: Duration) -> Self {
		Self { nanos: duration.subsec_nanos(), secs: duration.as_secs() }
	}
}

impl From<DurationWrapper> for Duration {
	fn from(duration: DurationWrapper) -> Self {
		Self::from_secs(duration.secs) + Self::from_nanos(duration.nanos as u64)
	}
}

impl<T: Config> ValBurnedNotifier<MultiCurrencyBalanceOf<T>> for Pallet<T> {
	/// Notify the pallet that this `amount` of VAL token was burned.
	fn notify_val_burned(amount: MultiCurrencyBalanceOf<T>) {
		let total_val_burned: MultiCurrencyBalanceOf<T> = EraValBurned::<T>::get() + amount;
		EraValBurned::<T>::put(total_val_burned);
	}
}
