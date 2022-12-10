#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

mod types;
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
use types::RecordType;

#[frame_support::pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use crate::types::RecordType;
	// use scale_info::TypeInfo;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// The
		type TLDId: Member + Parameter + MaxEncodedLen + Copy;

		#[pallet::constant]
		type StringLimit: Get<u32>;
	}

	pub type TLDName<T: Config> = BoundedVec<u8, T::StringLimit>;
	pub type DomainName<T: Config> = BoundedVec<u8, T::StringLimit>;
	pub type Record<T: Config> = BoundedVec<u8, T::StringLimit>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/

	#[pallet::storage]
	#[pallet::storage_prefix = "TLD"]
	#[pallet::getter(fn tld)]
	/// Details of a collection.
	pub(super) type TLD<T: Config> = StorageMap<_, Blake2_128Concat, TLDName<T>, T::AccountId>;

	#[pallet::storage]
	#[pallet::storage_prefix = "Domain"]
	#[pallet::getter(fn domain)]
	/// Details of a collection.
	pub(super) type Domain<T: Config> = StorageMap<_, Blake2_128Concat, TLDName<T>, DomainName<T>>;

	//    #[pallet::storage]
	// #[pallet::getter(fn record)]
	// // Learn more about declaring storage items:
	// // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	// pub type DomainRecord<T: Config> = StorageDoubleMap<_, Blake2_128Concat,TLDName<T>,
	// Blake2_128Concat, DomainName<T>, Record<T>, OptionQuery,>;

	#[pallet::storage]
	#[pallet::getter(fn dns_record)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type DNSRecord<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, TLDName<T>>,
			NMapKey<Blake2_128Concat, DomainName<T>>,
			NMapKey<Blake2_128Concat, RecordType>,
		),
		Record<T>,
		OptionQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TLDStored {
			tld: TLDName<T>,
			who: T::AccountId,
		},
		DomainStored {
			tld: TLDName<T>,
			domain: DomainName<T>,
			who: T::AccountId,
		},
		DomainRecordStored {
			tld: TLDName<T>,
			domain: DomainName<T>,
			record: BoundedVec<u8, T::StringLimit>,
			who: T::AccountId,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn mint_tld(
			origin: OriginFor<T>,
			tld_name: BoundedVec<u8, T::StringLimit>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<TLD<T>>::insert(tld_name.clone(), who.clone());

			// Emit an event.
			Self::deposit_event(Event::TLDStored { tld: tld_name, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn mint_domain(
			origin: OriginFor<T>,
			tld_name: BoundedVec<u8, T::StringLimit>,
			domain_name: DomainName<T>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Domain<T>>::insert(&tld_name, &domain_name);

			// Emit an event.
			Self::deposit_event(Event::DomainStored { tld: tld_name, domain: domain_name, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn mint_record(
			origin: OriginFor<T>,
			tld_name: TLDName<T>,
			domain_name: DomainName<T>,
			dns_type: RecordType,
			record: Record<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// <DomainRecord<T>>::insert(tld_name.clone(), domain_name.clone(), record.clone());
			<DNSRecord<T>>::insert(
				(tld_name.clone(), domain_name.clone(), dns_type.clone()),
				record.clone(),
			);
			Self::deposit_event(Event::DomainRecordStored {
				tld: tld_name,
				domain: domain_name,
				record,
				who,
			});
			Ok(())
		}
	}
}
