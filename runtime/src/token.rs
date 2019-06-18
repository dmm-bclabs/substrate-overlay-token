/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use rstd::prelude::Vec;
use parity_codec::Codec;
use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result, ensure, StorageMap, Parameter};
use system::ensure_signed;
use runtime_primitives::traits::{SimpleArithmetic, CheckedSub, CheckedAdd, As};

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	/// SimpleArithmetic: for using checked_add, checked_sub
	/// Codec, Default: for using in Store
	/// Parameter: for using as a parameter
	type TokenBalance: SimpleArithmetic + Codec + Default + Copy + Parameter + As<u128>;

	type ChildChainId: Codec + Default + Copy + Parameter + As<u32>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as token {

		Init get(is_init): bool;

		Root get(is_root): bool;

		Owner get(owner): T::AccountId;

		Name get(name): Vec<u8>;
		Ticker get(ticker): Vec<u8>;

		// TotalSupply := LocalSupply + ParentSupply + Sum(ChildSupplies)
		TotalSupply get(total_supply): T::TokenBalance;
		LocalSupply get(local_supply): T::TokenBalance;
		// BalanceOf: only for LocalSupply
		BalanceOf get(balance_of): map T::AccountId => T::TokenBalance;

		ParentSupply get(parent_supply): T::TokenBalance;
		ChildSupplies get(child_supply_of): map T::ChildChainId => T::TokenBalance;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		fn init(origin) -> Result {
			let sender = ensure_signed(origin)?;

			ensure!(Self::is_init() == false, "Already initialized.");

			let zero = <T::TokenBalance as As<u128>>::sa(0);

			<TotalSupply<T>>::put(zero);
			<LocalSupply<T>>::put(zero);
			<ParentSupply<T>>::put(zero);
			<BalanceOf<T>>::insert(sender.clone(), zero);
			<Owner<T>>::put(sender.clone());
			<Init<T>>::put(true);
			<Root<T>>::put(true);

			Ok(())
		}

		fn set_parent(_origin, parent_supply: u128) -> Result {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::owner() == sender, "Only owner can set parent chain");
			ensure!(Self::is_root(), "This chain already has parent chain");

			let supply = <T::TokenBalance as As<u128>>::sa(parent_supply);
			let zero = <T::TokenBalance as As<u128>>::sa(0);

			<TotalSupply<T>>::put(supply);
			<LocalSupply<T>>::put(zero);
			<ParentSupply<T>>::put(supply);
			<Root<T>>::put(false);

			Ok(())
		}

		// Transter token in LocalSupply
		fn transfer(_origin, receiver: T::AccountId, value: u128) -> Result {
			let sender = ensure_signed(_origin)?;

			let value = <T::TokenBalance as As<u128>>::sa(value);
			
			ensure!(<BalanceOf<T>>::exists(sender.clone()), "Account does not own this token");
			let sender_balance = Self::balance_of(sender.clone());
			ensure!(sender_balance >= value, "Not enough balance.");

			let updated_sender_balance = sender_balance.checked_sub(&value).ok_or("overflow")?;
			let receiver_balance = Self::balance_of(receiver.clone());
			let updated_receiver_balance = receiver_balance.checked_add(&value).ok_or("overflow")?;

			<BalanceOf<T>>::insert(sender.clone(), updated_sender_balance);
			<BalanceOf<T>>::insert(receiver.clone(), updated_receiver_balance);

			Ok(())
		}

		// Mint new token by the owner
		fn mint(_origin, value: u128) -> Result {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::owner() == sender, "Only owner can mint the token");

			let value = <T::TokenBalance as As<u128>>::sa(value);

			let sender_balance = match <BalanceOf<T>>::exists(sender.clone()) {
				true => Self::balance_of(sender.clone()),
				_ => <T::TokenBalance as As<u128>>::sa(0)
			};
			let updated_sender_balance = sender_balance.checked_add(&value).ok_or("overflow")?;
			let updated_total_supply = Self::total_supply().checked_add(&value).ok_or("overflow")?;

			let (updated_local_supply, updated_parent_supply) = match Self::is_root() {
				true => (Self::local_supply().checked_add(&value).ok_or("overflow")?, Self::parent_supply()),
				false => (Self::local_supply(), Self::parent_supply().checked_add(&value).ok_or("overflow")?)
			};
			<BalanceOf<T>>::insert(sender.clone(), updated_sender_balance);
			<TotalSupply<T>>::put(updated_total_supply);
			<LocalSupply<T>>::put(updated_local_supply);
			<ParentSupply<T>>::put(updated_parent_supply);

			Ok(())
		}

		// Burn some existing token by the owner
		fn burn(_origin, value: T::TokenBalance) -> Result {
			let sender = ensure_signed(_origin)?;
			ensure!(Self::owner() == sender, "Only owner can burn the token");

			ensure!(<BalanceOf<T>>::exists(sender.clone()), "Account does not own this token");
			let sender_balance = Self::balance_of(sender.clone());
			ensure!(sender_balance >= value, "Not enough balance.");

			let updated_sender_balance = sender_balance.checked_sub(&value).ok_or("overflow")?;
			let updated_total_supply = Self::total_supply().checked_sub(&value).ok_or("overflow")?;

			let (updated_local_supply, updated_parent_supply) = match Self::is_root() {
				true => (Self::local_supply().checked_sub(&value).ok_or("overflow")?, Self::parent_supply()),
				false => (Self::local_supply(), Self::parent_supply().checked_sub(&value).ok_or("overflow")?)
			};
			<BalanceOf<T>>::insert(sender.clone(), updated_sender_balance);
			<TotalSupply<T>>::put(updated_total_supply);
			<LocalSupply<T>>::put(updated_local_supply);
			<ParentSupply<T>>::put(updated_parent_supply);

			Ok(())
		}

		// Send some token to the parent chain
		fn send_to_parent(_origin, value: T::TokenBalance) -> Result {
			let sender = ensure_signed(_origin)?;
			ensure!(<BalanceOf<T>>::exists(sender.clone()), "Account does not own this token");
			let sender_balance = Self::balance_of(sender.clone());
			ensure!(sender_balance >= value, "Not enough balance.");

			let updated_sender_balance = sender_balance.checked_sub(&value).ok_or("overflow")?;
			let updated_local_supply = Self::local_supply().checked_sub(&value).ok_or("overflow")?;
			let updated_parent_supply = Self::parent_supply().checked_add(&value).ok_or("overflow")?;

			<BalanceOf<T>>::insert(sender.clone(), updated_sender_balance);
			<LocalSupply<T>>::put(updated_local_supply);
			<ParentSupply<T>>::put(updated_parent_supply);

			Ok(())
		}

		// Send some token to a child chain
		fn send_to_child(_origin, child: T::ChildChainId, value: T::TokenBalance) -> Result {
			let sender = ensure_signed(_origin)?;
			ensure!(<BalanceOf<T>>::exists(sender.clone()), "Account does not own this token");
			let sender_balance = Self::balance_of(sender.clone());
			ensure!(sender_balance >= value, "Not enough balance.");

			let updated_sender_balance = sender_balance.checked_sub(&value).ok_or("overflow")?;
			let updated_local_supply = Self::local_supply().checked_sub(&value).ok_or("overflow")?;
			let updated_child_supply = Self::child_supply_of(child).checked_add(&value).ok_or("overflow")?;

			<BalanceOf<T>>::insert(sender.clone(), updated_sender_balance);
			<LocalSupply<T>>::put(updated_local_supply);
			<ChildSupplies<T>>::insert(child, updated_child_supply);

			Ok(())
		}

		// Receive some token from the parent chain
		fn receive_from_parent(_origin, value: T::TokenBalance) -> Result {
			let receiver = ensure_signed(_origin)?;
			let parent_supply = Self::parent_supply();
			ensure!(parent_supply >= value, "Not enough balance.");

			let receiver_balance = match <BalanceOf<T>>::exists(receiver.clone()) {
				true => Self::balance_of(receiver.clone()),
				_ => <T::TokenBalance as As<u128>>::sa(0)
			};

			let updated_parent_supply = parent_supply.checked_sub(&value).ok_or("overflow")?;
			let updated_local_supply = Self::local_supply().checked_add(&value).ok_or("overflow")?;
			let updated_receiver_balance = receiver_balance.checked_add(&value).ok_or("overflow")?;

			<ParentSupply<T>>::put(updated_parent_supply);
			<LocalSupply<T>>::put(updated_local_supply);
			<BalanceOf<T>>::insert(receiver.clone(), updated_receiver_balance);

			Ok(())
		}

		// Receive some token from a child chain
		fn receive_from_child(_origin, child: T::ChildChainId, value: T::TokenBalance) -> Result {
			let receiver = ensure_signed(_origin)?;
			ensure!(<ChildSupplies<T>>::exists(child), "Child Chain does not own this token");
			let child_supply = Self::child_supply_of(child);
			ensure!(child_supply >= value, "Not enough balance.");

			let receiver_balance = match <BalanceOf<T>>::exists(receiver.clone()) {
				true => Self::balance_of(receiver.clone()),
				_ => <T::TokenBalance as As<u128>>::sa(0)
			};

			let updated_child_supply = child_supply.checked_sub(&value).ok_or("overflow")?;
			let updated_local_supply = Self::local_supply().checked_add(&value).ok_or("overflow")?;
			let updated_receiver_balance = receiver_balance.checked_add(&value).ok_or("overflow")?;

			<ChildSupplies<T>>::insert(child, updated_child_supply);
			<LocalSupply<T>>::put(updated_local_supply);
			<BalanceOf<T>>::insert(receiver.clone(), updated_receiver_balance);

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type token = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(token::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(token::something(), Some(42));
		});
	}
}
