#![feature(prelude_import)]
//! # NicksMigration Module
//!
//! - [`nicks::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//!
//! NicksMigration is an example module for migrating the storage of a pallet. It is based on the
//! Nicks example pallet.
//! *Warning: Do not use this pallet as-is in production.*
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `set_name` - Set the associated name of an account; a small deposit is reserved if not already
//!   taken.
//! * `clear_name` - Remove an account's associated name; the deposit is returned.
//! * `kill_name` - Forcibly remove the associated name; the deposit is lost.
//!
//! [`Call`]: ./enum.Call.html
//! [`Trait`]: ./trait.Trait.html
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use sp_std::prelude::*;
use sp_runtime::{
    traits::{StaticLookup, Zero},
};
use frame_support::{
    decl_module, decl_event, decl_storage, ensure, decl_error,
    traits::{Currency, EnsureOrigin, ReservableCurrency, OnUnbalanced, Get},
    weights::Weight,
};
use frame_system::ensure_signed;
type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
pub trait Trait: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    /// The currency trait.
    type Currency: ReservableCurrency<Self::AccountId>;
    /// Reservation fee.
    type ReservationFee: Get<BalanceOf<Self>>;
    /// What to do with slashed funds.
    type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;
    /// The origin which may forcibly set or remove a name. Root can always do this.
    type ForceOrigin: EnsureOrigin<Self::Origin>;
    /// The maximum length a name may be.
    type MaxLength: Get<usize>;
}
/// A nickname with a first and last part.
pub struct Nickname {
    first: Vec<u8>,
    last: Option<Vec<u8>>,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for Nickname {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            _parity_scale_codec::Encode::encode_to(&self.first, __codec_dest_edqy);
            _parity_scale_codec::Encode::encode_to(&self.last, __codec_dest_edqy);
        }
    }
    impl _parity_scale_codec::EncodeLike for Nickname {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for Nickname {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(Nickname {
                first: {
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Nickname::first`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
                last: {
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Nickname::last`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                },
            })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for Nickname {
    #[inline]
    fn default() -> Nickname {
        Nickname {
            first: ::core::default::Default::default(),
            last: ::core::default::Default::default(),
        }
    }
}
impl core::fmt::Debug for Nickname {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("Nickname")
            .field("first", &self.first)
            .field("last", &self.last)
            .finish()
    }
}
impl ::core::marker::StructuralPartialEq for Nickname {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Nickname {
    #[inline]
    fn eq(&self, other: &Nickname) -> bool {
        match *other {
            Nickname {
                first: ref __self_1_0,
                last: ref __self_1_1,
            } => match *self {
                Nickname {
                    first: ref __self_0_0,
                    last: ref __self_0_1,
                } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Nickname) -> bool {
        match *other {
            Nickname {
                first: ref __self_1_0,
                last: ref __self_1_1,
            } => match *self {
                Nickname {
                    first: ref __self_0_0,
                    last: ref __self_0_1,
                } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
            },
        }
    }
}
/// Utility type for managing upgrades/migrations.
pub enum StorageVersion {
    V1Bytes,
    V2Struct,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for StorageVersion {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                StorageVersion::V1Bytes => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                }
                StorageVersion::V2Struct => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for StorageVersion {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for StorageVersion {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `StorageVersion`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(StorageVersion::V1Bytes),
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => Ok(StorageVersion::V2Struct),
                _ => Err("Could not decode `StorageVersion`, variant doesn\'t exist".into()),
            }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for StorageVersion {
    #[inline]
    fn clone(&self) -> StorageVersion {
        match (&*self,) {
            (&StorageVersion::V1Bytes,) => StorageVersion::V1Bytes,
            (&StorageVersion::V2Struct,) => StorageVersion::V2Struct,
        }
    }
}
impl core::fmt::Debug for StorageVersion {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::V1Bytes => fmt.debug_tuple("StorageVersion::V1Bytes").finish(),
            Self::V2Struct => fmt.debug_tuple("StorageVersion::V2Struct").finish(),
            _ => Ok(()),
        }
    }
}
impl ::core::marker::StructuralPartialEq for StorageVersion {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for StorageVersion {
    #[inline]
    fn eq(&self, other: &StorageVersion) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
    IterableStorageMap as _, IterableStorageDoubleMap as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type NameOf;
    type PalletVersion;
}
impl<T: Trait + 'static> Store for Module<T> {
    type NameOf = NameOf<T>;
    type PalletVersion = PalletVersion;
}
impl<T: Trait + 'static> Module<T> {}
#[doc(hidden)]
pub struct __GetByteStructNameOf<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_NameOf:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructNameOf<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_NameOf
            .get_or_init(|| {
                let def_val: Option<(Nickname, BalanceOf<T>)> = Default::default();
                <Option<(Nickname, BalanceOf<T>)> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructNameOf<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructNameOf<T> {}
#[doc(hidden)]
pub struct __GetByteStructPalletVersion<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_PalletVersion:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructPalletVersion<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_PalletVersion
            .get_or_init(|| {
                let def_val: StorageVersion = StorageVersion::V1Bytes;
                <StorageVersion as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructPalletVersion<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructPalletVersion<T> {}
impl<T: Trait + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "NicksMigration" ) , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "NameOf" ) , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Twox64Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(Nickname, BalanceOf<T>)" ) , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNameOf :: < T > ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ " The lookup table for names." ] ) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "PalletVersion" ) , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Plain ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "StorageVersion" ) ) , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructPalletVersion :: < T > ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ " The current version of the pallet." ] ) , } ] [ .. ] ) , }
    }
}
/// Hidden instance generated to be internally used when module is used without
/// instance.
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for __InherentHiddenInstance {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
        }
    }
    impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for __InherentHiddenInstance {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
    for __InherentHiddenInstance
{
    const PREFIX: &'static str = "NicksMigration";
}
/// The lookup table for names.
struct NameOf<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
        Nickname,
        BalanceOf<T>,
    )> for NameOf<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"NameOf"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        (Nickname, BalanceOf<T>),
    > for NameOf<T>
{
    type Query = Option<(Nickname, BalanceOf<T>)>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Twox64Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"NameOf"
    }
    fn from_optional_value_to_query(v: Option<(Nickname, BalanceOf<T>)>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<(Nickname, BalanceOf<T>)> {
        v
    }
}
/// The current version of the pallet.
struct PalletVersion(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<()>,
);
impl
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<
        StorageVersion,
    > for PalletVersion
{
    type Query = StorageVersion;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
    }
    fn storage_prefix() -> &'static [u8] {
        b"PalletVersion"
    }
    fn from_optional_value_to_query(v: Option<StorageVersion>) -> Self::Query {
        v.unwrap_or_else(|| StorageVersion::V1Bytes)
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<StorageVersion> {
        Some(v)
    }
}
/// [`RawEvent`] specialized for the configuration [`Config`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Config`]: trait.Config.html
pub type Event<T> = RawEvent<<T as frame_system::Config>::AccountId, BalanceOf<T>>;
/// Events for this module.
///
pub enum RawEvent<AccountId, Balance> {
    /// A name was set. \[who\]
    NameSet(AccountId),
    /// A name was forcibly set. \[target\]
    NameForced(AccountId),
    /// A name was changed. \[who\]
    NameChanged(AccountId),
    /// A name was cleared, and the given balance returned. \[who, deposit\]
    NameCleared(AccountId, Balance),
    /// A name was removed and the given balance slashed. \[target, deposit\]
    NameKilled(AccountId, Balance),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone, Balance: ::core::clone::Clone> ::core::clone::Clone
    for RawEvent<AccountId, Balance>
{
    #[inline]
    fn clone(&self) -> RawEvent<AccountId, Balance> {
        match (&*self,) {
            (&RawEvent::NameSet(ref __self_0),) => {
                RawEvent::NameSet(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::NameForced(ref __self_0),) => {
                RawEvent::NameForced(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::NameChanged(ref __self_0),) => {
                RawEvent::NameChanged(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&RawEvent::NameCleared(ref __self_0, ref __self_1),) => RawEvent::NameCleared(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::NameKilled(ref __self_0, ref __self_1),) => RawEvent::NameKilled(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
        }
    }
}
impl<AccountId, Balance> ::core::marker::StructuralPartialEq for RawEvent<AccountId, Balance> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq, Balance: ::core::cmp::PartialEq> ::core::cmp::PartialEq
    for RawEvent<AccountId, Balance>
{
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId, Balance>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::NameSet(ref __self_0), &RawEvent::NameSet(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&RawEvent::NameForced(ref __self_0), &RawEvent::NameForced(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &RawEvent::NameChanged(ref __self_0),
                        &RawEvent::NameChanged(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    (
                        &RawEvent::NameCleared(ref __self_0, ref __self_1),
                        &RawEvent::NameCleared(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::NameKilled(ref __self_0, ref __self_1),
                        &RawEvent::NameKilled(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId, Balance>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&RawEvent::NameSet(ref __self_0), &RawEvent::NameSet(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&RawEvent::NameForced(ref __self_0), &RawEvent::NameForced(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &RawEvent::NameChanged(ref __self_0),
                        &RawEvent::NameChanged(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    (
                        &RawEvent::NameCleared(ref __self_0, ref __self_1),
                        &RawEvent::NameCleared(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::NameKilled(ref __self_0, ref __self_1),
                        &RawEvent::NameKilled(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<AccountId, Balance> ::core::marker::StructuralEq for RawEvent<AccountId, Balance> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::Eq, Balance: ::core::cmp::Eq> ::core::cmp::Eq
    for RawEvent<AccountId, Balance>
{
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Balance>;
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId, Balance> _parity_scale_codec::Encode for RawEvent<AccountId, Balance>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RawEvent::NameSet(ref aa) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RawEvent::NameForced(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RawEvent::NameChanged(ref aa) => {
                    __codec_dest_edqy.push_byte(2usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                RawEvent::NameCleared(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(3usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                RawEvent::NameKilled(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(4usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<AccountId, Balance> _parity_scale_codec::EncodeLike for RawEvent<AccountId, Balance>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
        Balance: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId, Balance> _parity_scale_codec::Decode for RawEvent<AccountId, Balance>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
        Balance: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `RawEvent`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(RawEvent::NameSet({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `RawEvent::NameSet.0`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => Ok(RawEvent::NameForced({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `RawEvent::NameForced.0`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                __codec_x_edqy if __codec_x_edqy == 2usize as u8 => Ok(RawEvent::NameChanged({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `RawEvent::NameChanged.0`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                __codec_x_edqy if __codec_x_edqy == 3usize as u8 => Ok(RawEvent::NameCleared(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `RawEvent::NameCleared.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `RawEvent::NameCleared.1`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                __codec_x_edqy if __codec_x_edqy == 4usize as u8 => Ok(RawEvent::NameKilled(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `RawEvent::NameKilled.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `RawEvent::NameKilled.1`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                _ => Err("Could not decode `RawEvent`, variant doesn\'t exist".into()),
            }
        }
    }
};
impl<AccountId, Balance> core::fmt::Debug for RawEvent<AccountId, Balance>
where
    AccountId: core::fmt::Debug,
    Balance: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::NameSet(ref a0) => fmt.debug_tuple("RawEvent::NameSet").field(a0).finish(),
            Self::NameForced(ref a0) => fmt.debug_tuple("RawEvent::NameForced").field(a0).finish(),
            Self::NameChanged(ref a0) => {
                fmt.debug_tuple("RawEvent::NameChanged").field(a0).finish()
            }
            Self::NameCleared(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::NameCleared")
                .field(a0)
                .field(a1)
                .finish(),
            Self::NameKilled(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::NameKilled")
                .field(a0)
                .field(a1)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId, Balance> From<RawEvent<AccountId, Balance>> for () {
    fn from(_: RawEvent<AccountId, Balance>) -> () {
        ()
    }
}
impl<AccountId, Balance> RawEvent<AccountId, Balance> {
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameSet"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was set. \[who\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameForced"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was forcibly set. \[target\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameChanged"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was changed. \[who\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameCleared"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Balance",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was cleared, and the given balance returned. \[who, deposit\]",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("NameKilled"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Balance",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A name was removed and the given balance slashed. \[target, deposit\]",
                ]),
            },
        ]
    }
}
/// Error for the nicks module.
pub enum Error<T: Trait> {
    #[doc(hidden)]
    __Ignore(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    /// A name is too long.
    TooLong,
    /// An account isn't named.
    Unnamed,
}
impl<T: Trait> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl<T: Trait> Error<T> {
    fn as_u8(&self) -> u8 {
        match self {
            Error::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::TooLong => 0,
            Error::Unnamed => 0 + 1,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::TooLong => "TooLong",
            Error::Unnamed => "Unnamed",
        }
    }
}
impl<T: Trait> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        err.as_str()
    }
}
impl<T: Trait> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::PalletInfo as ::frame_support::traits::PalletInfo>::index::<Module<T>>()
            .expect("Every active module has an index in the runtime; qed")
            as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: err.as_u8(),
            message: Some(err.as_str()),
        }
    }
}
impl<T: Trait> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("TooLong"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" A name is too long.",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("Unnamed"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" An account isn't named.",
                ]),
            },
        ]
    }
}
/// Nicks module declaration.
pub struct Module<T: Trait>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
impl<T: Trait> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Trait> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Trait> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: frame_system::Config + Trait>
    ::frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber> for Module<T>
{
}
impl<T: Trait> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "on_runtime_upgrade",
                        "pallet_nicks_migration",
                        ::tracing::Level::TRACE,
                        Some("frame/nicks-migration/src/lib.rs"),
                        Some(124u32),
                        Some("pallet_nicks_migration"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let result: frame_support::weights::Weight = (|| migration::migrate_to_v2::<T>())();
        frame_support::traits::PalletVersion {
            major: 2u16,
            minor: 0u8,
            patch: 1u8,
        }
        .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
        let additional_write =
            <<T as frame_system::Config>::DbWeight as ::frame_support::traits::Get<_>>::get()
                .writes(1);
        result.saturating_add(additional_write)
    }
}
impl<T: frame_system::Config + Trait>
    ::frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber> for Module<T>
{
}
impl<T: frame_system::Config + Trait>
    ::frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
    for Module<T>
{
}
impl<T: Trait> Module<T> {
    /// Deposits an event using `frame_system::Module::deposit_event`.
    fn deposit_event(event: impl Into<<T as Trait>::Event>) {
        <frame_system::Module<T>>::deposit_event(event.into())
    }
}
#[cfg(feature = "std")]
impl<T: Trait> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Trait> Module<T> {
    #[allow(unreachable_code)]
    /// Set an account's name. The name should be a UTF-8-encoded string by convention, though
    /// we don't check it.
    ///
    /// The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.
    ///
    /// If the account doesn't already have a name, then a fee of `ReservationFee` is reserved
    /// in the account.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn set_name(
        origin: T::Origin,
        first: Vec<u8>,
        last: Option<Vec<u8>>,
    ) -> ::frame_support::dispatch::DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "set_name",
                        "pallet_nicks_migration",
                        ::tracing::Level::TRACE,
                        Some("frame/nicks-migration/src/lib.rs"),
                        Some(124u32),
                        Some("pallet_nicks_migration"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        {
            let sender = ensure_signed(origin)?;
            let len = match last {
                None => first.len(),
                Some(ref last_name) => first.len() + last_name.len(),
            };
            {
                if !(len <= T::MaxLength::get()) {
                    {
                        return Err(Error::<T>::TooLong.into());
                    };
                }
            };
            let deposit = if let Some((_, deposit)) = <NameOf<T>>::get(&sender) {
                Self::deposit_event(RawEvent::NameChanged(sender.clone()));
                deposit
            } else {
                let deposit = T::ReservationFee::get();
                T::Currency::reserve(&sender, deposit.clone())?;
                Self::deposit_event(RawEvent::NameSet(sender.clone()));
                deposit
            };
            <NameOf<T>>::insert(&sender, (Nickname { first, last }, deposit));
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    /// Clear an account's name and return the deposit. Fails if the account was not named.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - One balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn clear_name(origin: T::Origin) -> ::frame_support::dispatch::DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "clear_name",
                        "pallet_nicks_migration",
                        ::tracing::Level::TRACE,
                        Some("frame/nicks-migration/src/lib.rs"),
                        Some(124u32),
                        Some("pallet_nicks_migration"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        {
            let sender = ensure_signed(origin)?;
            let deposit = <NameOf<T>>::take(&sender).ok_or(Error::<T>::Unnamed)?.1;
            let _ = T::Currency::unreserve(&sender, deposit.clone());
            Self::deposit_event(RawEvent::NameCleared(sender, deposit));
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    /// Remove an account's name and take charge of the deposit.
    ///
    /// Fails if `who` has not been named. The deposit is dealt with through `T::Slashed`
    /// imbalance handler.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - One unbalanced handler (probably a balance transfer)
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn kill_name(
        origin: T::Origin,
        target: <T::Lookup as StaticLookup>::Source,
    ) -> ::frame_support::dispatch::DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "kill_name",
                        "pallet_nicks_migration",
                        ::tracing::Level::TRACE,
                        Some("frame/nicks-migration/src/lib.rs"),
                        Some(124u32),
                        Some("pallet_nicks_migration"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        {
            T::ForceOrigin::ensure_origin(origin)?;
            let target = T::Lookup::lookup(target)?;
            let deposit = <NameOf<T>>::take(&target).ok_or(Error::<T>::Unnamed)?.1;
            T::Slashed::on_unbalanced(T::Currency::slash_reserved(&target, deposit.clone()).0);
            Self::deposit_event(RawEvent::NameKilled(target, deposit));
        }
        Ok(())
    }
    #[allow(unreachable_code)]
    /// Set a third-party account's name with no deposit.
    ///
    /// No length checking is done on the name.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn force_name(
        origin: T::Origin,
        target: <T::Lookup as StaticLookup>::Source,
        first: Vec<u8>,
        last: Option<Vec<u8>>,
    ) -> ::frame_support::dispatch::DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "force_name",
                        "pallet_nicks_migration",
                        ::tracing::Level::TRACE,
                        Some("frame/nicks-migration/src/lib.rs"),
                        Some(124u32),
                        Some("pallet_nicks_migration"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        {
            T::ForceOrigin::ensure_origin(origin)?;
            let target = T::Lookup::lookup(target)?;
            let deposit = <NameOf<T>>::get(&target)
                .map(|x| x.1)
                .unwrap_or_else(Zero::zero);
            <NameOf<T>>::insert(&target, (Nickname { first, last }, deposit));
            Self::deposit_event(RawEvent::NameForced(target));
        }
        Ok(())
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Trait> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    /// Set an account's name. The name should be a UTF-8-encoded string by convention, though
    /// we don't check it.
    ///
    /// The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.
    ///
    /// If the account doesn't already have a name, then a fee of `ReservationFee` is reserved
    /// in the account.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    set_name(Vec<u8>, Option<Vec<u8>>),
    #[allow(non_camel_case_types)]
    /// Clear an account's name and return the deposit. Fails if the account was not named.
    ///
    /// The dispatch origin for this call must be _Signed_.
    ///
    /// # <weight>
    /// - O(1).
    /// - One balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    clear_name(),
    #[allow(non_camel_case_types)]
    /// Remove an account's name and take charge of the deposit.
    ///
    /// Fails if `who` has not been named. The deposit is dealt with through `T::Slashed`
    /// imbalance handler.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - One unbalanced handler (probably a balance transfer)
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    kill_name(<T::Lookup as StaticLookup>::Source),
    #[allow(non_camel_case_types)]
    /// Set a third-party account's name with no deposit.
    ///
    /// No length checking is done on the name.
    ///
    /// The dispatch origin for this call must match `T::ForceOrigin`.
    ///
    /// # <weight>
    /// - O(1).
    /// - At most one balance operation.
    /// - One storage read/write.
    /// - One event.
    /// # </weight>
    force_name(
        <T::Lookup as StaticLookup>::Source,
        Vec<u8>,
        Option<Vec<u8>>,
    ),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Encode for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::set_name(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                Call::clear_name() => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                }
                Call::kill_name(ref aa) => {
                    __codec_dest_edqy.push_byte(2usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::force_name(ref aa, ref ba, ref ca) => {
                    __codec_dest_edqy.push_byte(3usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ca, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Decode for Call<T>
    where
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
        <T::Lookup as StaticLookup>::Source: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(Call::set_name(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::set_name.0`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::set_name.1`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => Ok(Call::clear_name()),
                __codec_x_edqy if __codec_x_edqy == 2usize as u8 => Ok(Call::kill_name({
                    let __codec_res_edqy = _parity_scale_codec::Decode::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Call::kill_name.0`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                __codec_x_edqy if __codec_x_edqy == 3usize as u8 => Ok(Call::force_name(
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::force_name.0`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::force_name.1`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    {
                        let __codec_res_edqy =
                            _parity_scale_codec::Decode::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => return Err(e.chain("Could not decode `Call::force_name.2`")),
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                )),
                _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
            }
        }
    }
};
impl<T: Trait> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::set_name(ref first, ref last) => {
                let base_weight = 50_000_000;
                let weight = < dyn :: frame_support :: dispatch :: WeighData < ( & Vec < u8 > , & Option < Vec < u8 > > ) > > :: weigh_data ( & base_weight , ( first , last ) ) ;
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &Vec<u8>,
                    &Option<Vec<u8>>,
                )>>::classify_dispatch(&base_weight, (first, last));
                let pays_fee = < dyn :: frame_support :: dispatch :: PaysFee < ( & Vec < u8 > , & Option < Vec < u8 > > ) > > :: pays_fee ( & base_weight , ( first , last ) ) ;
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::clear_name() => {
                let base_weight = 70_000_000;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<()>>::weigh_data(&base_weight, ());
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                        &base_weight,
                        (),
                    );
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<()>>::pays_fee(&base_weight, ());
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::kill_name(ref target) => {
                let base_weight = 70_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &<T::Lookup as StaticLookup>::Source,
                )>>::weigh_data(&base_weight, (target,));
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &<T::Lookup as StaticLookup>::Source,
                )>>::classify_dispatch(&base_weight, (target,));
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &<T::Lookup as StaticLookup>::Source,
                )>>::pays_fee(&base_weight, (target,));
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::force_name(ref target, ref first, ref last) => {
                let base_weight = 70_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &<T::Lookup as StaticLookup>::Source,
                    &Vec<u8>,
                    &Option<Vec<u8>>,
                )>>::weigh_data(&base_weight, (target, first, last));
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<(
                        &<T::Lookup as StaticLookup>::Source,
                        &Vec<u8>,
                        &Option<Vec<u8>>,
                    )>>::classify_dispatch(&base_weight, (target, first, last));
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &<T::Lookup as StaticLookup>::Source,
                    &Vec<u8>,
                    &Option<Vec<u8>>,
                )>>::pays_fee(&base_weight, (target, first, last));
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::set_name(ref first, ref last) => {
                let _ = (first, last);
                "set_name"
            }
            Call::clear_name() => {
                let _ = ();
                "clear_name"
            }
            Call::kill_name(ref target) => {
                let _ = (target);
                "kill_name"
            }
            Call::force_name(ref target, ref first, ref last) => {
                let _ = (target, first, last);
                "force_name"
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &["set_name", "clear_name", "kill_name", "force_name"]
    }
}
pub use ::frame_support::traits::GetPalletVersion as _;
impl<T: Trait> ::frame_support::traits::GetPalletVersion for Module<T> {
    fn current_version() -> ::frame_support::traits::PalletVersion {
        frame_support::traits::PalletVersion {
            major: 2u16,
            minor: 0u8,
            patch: 1u8,
        }
    }
    fn storage_version() -> Option<::frame_support::traits::PalletVersion> {
        let key = ::frame_support::traits::PalletVersion::storage_key::<
            <T as frame_system::Config>::PalletInfo,
            Self,
        >()
        .expect("Every active pallet has a name in the runtime; qed");
        ::frame_support::storage::unhashed::get(&key)
    }
}
impl<T: Trait> ::frame_support::traits::OnGenesis for Module<T> {
    fn on_genesis() {
        frame_support::traits::PalletVersion {
            major: 2u16,
            minor: 0u8,
            patch: 1u8,
        }
        .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
    }
}
impl<T: Trait> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::set_name(ref first, ref last) => {
                Call::set_name((*first).clone(), (*last).clone())
            }
            Call::clear_name() => Call::clear_name(),
            Call::kill_name(ref target) => Call::kill_name((*target).clone()),
            Call::force_name(ref target, ref first, ref last) => {
                Call::force_name((*target).clone(), (*first).clone(), (*last).clone())
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::set_name(ref first, ref last) => {
                let self_params = (first, last);
                if let Call::set_name(ref first, ref last) = *_other {
                    self_params == (first, last)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::clear_name() => {
                let self_params = ();
                if let Call::clear_name() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::kill_name(ref target) => {
                let self_params = (target,);
                if let Call::kill_name(ref target) = *_other {
                    self_params == (target,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::force_name(ref target, ref first, ref last) => {
                let self_params = (target, first, last);
                if let Call::force_name(ref target, ref first, ref last) = *_other {
                    self_params == (target, first, last)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Trait> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::set_name(ref first, ref last) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"set_name", &(first.clone(), last.clone())) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::clear_name() => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"clear_name", &()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::kill_name(ref target) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"kill_name", &(target.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::force_name(ref target, ref first, ref last) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (
                        &"force_name",
                        &(target.clone(), first.clone(), last.clone()),
                    ) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
    type Origin = T::Origin;
    fn dispatch_bypass_filter(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::set_name(first, last) => <Module<T>>::set_name(_origin, first, last)
                .map(Into::into)
                .map_err(Into::into),
            Call::clear_name() => <Module<T>>::clear_name(_origin)
                .map(Into::into)
                .map_err(Into::into),
            Call::kill_name(target) => <Module<T>>::kill_name(_origin, target)
                .map(Into::into)
                .map_err(Into::into),
            Call::force_name(target, first, last) => {
                <Module<T>>::force_name(_origin, target, first, last)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Trait> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("set_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("first"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("last"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Option<Vec<u8>>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Set an account's name. The name should be a UTF-8-encoded string by convention, though",
                    r" we don't check it.",
                    r"",
                    r" The name may not be more than `T::MaxLength` bytes, nor less than `T::MinLength` bytes.",
                    r"",
                    r" If the account doesn't already have a name, then a fee of `ReservationFee` is reserved",
                    r" in the account.",
                    r"",
                    r" The dispatch origin for this call must be _Signed_.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - At most one balance operation.",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("clear_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Clear an account's name and return the deposit. Fails if the account was not named.",
                    r"",
                    r" The dispatch origin for this call must be _Signed_.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - One balance operation.",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("kill_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("target"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode(
                            "<T::Lookup as StaticLookup>::Source",
                        ),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Remove an account's name and take charge of the deposit.",
                    r"",
                    r" Fails if `who` has not been named. The deposit is dealt with through `T::Slashed`",
                    r" imbalance handler.",
                    r"",
                    r" The dispatch origin for this call must match `T::ForceOrigin`.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - One unbalanced handler (probably a balance transfer)",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("force_name"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("target"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode(
                            "<T::Lookup as StaticLookup>::Source",
                        ),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("first"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("last"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Option<Vec<u8>>"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Set a third-party account's name with no deposit.",
                    r"",
                    r" No length checking is done on the name.",
                    r"",
                    r" The dispatch origin for this call must match `T::ForceOrigin`.",
                    r"",
                    r" # <weight>",
                    r" - O(1).",
                    r" - At most one balance operation.",
                    r" - One storage read/write.",
                    r" - One event.",
                    r" # </weight>",
                ]),
            },
        ]
    }
}
impl<T: 'static + Trait> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        #[allow(non_upper_case_types)]
        #[allow(non_camel_case_types)]
        struct ReservationFeeDefaultByteGetter<T: Trait>(
            ::frame_support::dispatch::marker::PhantomData<(T,)>,
        );
        impl<T: 'static + Trait> ::frame_support::dispatch::DefaultByte
            for ReservationFeeDefaultByteGetter<T>
        {
            fn default_byte(&self) -> ::frame_support::dispatch::Vec<u8> {
                let value: BalanceOf<T> = T::ReservationFee::get();
                ::frame_support::dispatch::Encode::encode(&value)
            }
        }
        unsafe impl<T: 'static + Trait> Send for ReservationFeeDefaultByteGetter<T> {}
        unsafe impl<T: 'static + Trait> Sync for ReservationFeeDefaultByteGetter<T> {}
        #[allow(non_upper_case_types)]
        #[allow(non_camel_case_types)]
        struct MaxLengthDefaultByteGetter<T: Trait>(
            ::frame_support::dispatch::marker::PhantomData<(T,)>,
        );
        impl<T: 'static + Trait> ::frame_support::dispatch::DefaultByte for MaxLengthDefaultByteGetter<T> {
            fn default_byte(&self) -> ::frame_support::dispatch::Vec<u8> {
                let value: u32 = T::MaxLength::get() as u32;
                ::frame_support::dispatch::Encode::encode(&value)
            }
        }
        unsafe impl<T: 'static + Trait> Send for MaxLengthDefaultByteGetter<T> {}
        unsafe impl<T: 'static + Trait> Sync for MaxLengthDefaultByteGetter<T> {}
        &[
            ::frame_support::dispatch::ModuleConstantMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("ReservationFee"),
                ty: ::frame_support::dispatch::DecodeDifferent::Encode("BalanceOf<T>"),
                value: ::frame_support::dispatch::DecodeDifferent::Encode(
                    ::frame_support::dispatch::DefaultByteGetter(
                        &ReservationFeeDefaultByteGetter::<T>(
                            ::frame_support::dispatch::marker::PhantomData,
                        ),
                    ),
                ),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Reservation fee.",
                ]),
            },
            ::frame_support::dispatch::ModuleConstantMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("MaxLength"),
                ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                value: ::frame_support::dispatch::DecodeDifferent::Encode(
                    ::frame_support::dispatch::DefaultByteGetter(&MaxLengthDefaultByteGetter::<T>(
                        ::frame_support::dispatch::marker::PhantomData,
                    )),
                ),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" The maximum length a name may be.",
                ]),
            },
        ]
    }
}
impl<T: Trait> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <Error<T> as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
pub mod deprecated {
    use crate::Trait;
    use frame_support::{
        decl_module, decl_storage,
        traits::{Currency, ReservableCurrency, Get},
    };
    use sp_std::prelude::*;
    type BalanceOf<T> =
        <<T as Trait>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    use self::sp_api_hidden_includes_decl_storage::hidden_include::{
        StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
        IterableStorageMap as _, IterableStorageDoubleMap as _,
    };
    #[doc(hidden)]
    mod sp_api_hidden_includes_decl_storage {
        pub extern crate frame_support as hidden_include;
    }
    trait Store {
        type NameOf;
    }
    impl<T: Trait + 'static> Store for Module<T> {
        type NameOf = NameOf<T>;
    }
    impl<T: Trait + 'static> Module<T> {}
    #[doc(hidden)]
    pub struct __GetByteStructNameOf<T>(
        pub  self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_NameOf:
        self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
        > =
        self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
    #[cfg(feature = "std")]
    impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructNameOf<T>
    {
        fn default_byte(
            &self,
        ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>
        {
            use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_NameOf
                .get_or_init(|| {
                    let def_val: Option<(Vec<u8>, BalanceOf<T>)> = Default::default();
                    <Option<(Vec<u8>, BalanceOf<T>)> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    unsafe impl<T: Trait> Send for __GetByteStructNameOf<T> {}
    unsafe impl<T: Trait> Sync for __GetByteStructNameOf<T> {}
    impl<T: Trait + 'static> Module<T> {
        #[doc(hidden)]
        pub fn storage_metadata(
        ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata
        {
            self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "NicksMigration" ) , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "NameOf" ) , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Twox64Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(Vec<u8>, BalanceOf<T>)" ) , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNameOf :: < T > ( self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData ) ) ) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ] [ .. ] ) , }
        }
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub struct __InherentHiddenInstance;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for __InherentHiddenInstance {
        #[inline]
        fn clone(&self) -> __InherentHiddenInstance {
            match *self {
                __InherentHiddenInstance => __InherentHiddenInstance,
            }
        }
    }
    impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for __InherentHiddenInstance {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {}
        }
    }
    impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for __InherentHiddenInstance {
        #[inline]
        fn eq(&self, other: &__InherentHiddenInstance) -> bool {
            match *other {
                __InherentHiddenInstance => match *self {
                    __InherentHiddenInstance => true,
                },
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Encode for __InherentHiddenInstance {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
            }
        }
        impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl _parity_scale_codec::Decode for __InherentHiddenInstance {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                Ok(__InherentHiddenInstance)
            }
        }
    };
    impl core::fmt::Debug for __InherentHiddenInstance {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("__InherentHiddenInstance").finish()
        }
    }
    impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
        for __InherentHiddenInstance
    {
        const PREFIX: &'static str = "NicksMigration";
    }
    /// Genesis config for the module, allow to build genesis storage.
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(
        serialize = "Vec < (T :: AccountId, Vec < u8 >) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include::serde::Serialize, "
    ))]
    #[serde(bound(
        deserialize = "Vec < (T :: AccountId, Vec < u8 >) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include::serde::de::DeserializeOwned, "
    ))]
    pub struct GenesisConfig<T: Trait> {
        pub nicks: Vec<(T::AccountId, Vec<u8>)>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<T: Trait> _serde::Serialize for GenesisConfig<T>
        where
            Vec<(T::AccountId, Vec<u8>)>:
                self::sp_api_hidden_includes_decl_storage::hidden_include::serde::Serialize,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "nicks",
                    &self.nicks,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(rust_2018_idioms, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl < 'de , T : Trait > _serde :: Deserialize < 'de > for GenesisConfig < T > where Vec < ( T :: AccountId , Vec < u8 > ) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { fn deserialize < __D > ( __deserializer : __D ) -> _serde :: __private :: Result < Self , __D :: Error > where __D : _serde :: Deserializer < 'de > { # [ allow ( non_camel_case_types ) ] enum __Field { __field0 , } struct __FieldVisitor ; impl < 'de > _serde :: de :: Visitor < 'de > for __FieldVisitor { type Value = __Field ; fn expecting ( & self , __formatter : & mut _serde :: __private :: Formatter ) -> _serde :: __private :: fmt :: Result { _serde :: __private :: Formatter :: write_str ( __formatter , "field identifier" ) } fn visit_u64 < __E > ( self , __value : u64 ) -> _serde :: __private :: Result < Self :: Value , __E > where __E : _serde :: de :: Error { match __value { 0u64 => _serde :: __private :: Ok ( __Field :: __field0 ) , _ => _serde :: __private :: Err ( _serde :: de :: Error :: invalid_value ( _serde :: de :: Unexpected :: Unsigned ( __value ) , & "field index 0 <= i < 1" ) ) , } } fn visit_str < __E > ( self , __value : & str ) -> _serde :: __private :: Result < Self :: Value , __E > where __E : _serde :: de :: Error { match __value { "nicks" => _serde :: __private :: Ok ( __Field :: __field0 ) , _ => { _serde :: __private :: Err ( _serde :: de :: Error :: unknown_field ( __value , FIELDS ) ) } } } fn visit_bytes < __E > ( self , __value : & [ u8 ] ) -> _serde :: __private :: Result < Self :: Value , __E > where __E : _serde :: de :: Error { match __value { b"nicks" => _serde :: __private :: Ok ( __Field :: __field0 ) , _ => { let __value = & _serde :: __private :: from_utf8_lossy ( __value ) ; _serde :: __private :: Err ( _serde :: de :: Error :: unknown_field ( __value , FIELDS ) ) } } } } impl < 'de > _serde :: Deserialize < 'de > for __Field { # [ inline ] fn deserialize < __D > ( __deserializer : __D ) -> _serde :: __private :: Result < Self , __D :: Error > where __D : _serde :: Deserializer < 'de > { _serde :: Deserializer :: deserialize_identifier ( __deserializer , __FieldVisitor ) } } struct __Visitor < 'de , T : Trait > where Vec < ( T :: AccountId , Vec < u8 > ) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { marker : _serde :: __private :: PhantomData < GenesisConfig < T > > , lifetime : _serde :: __private :: PhantomData < & 'de ( ) > , } impl < 'de , T : Trait > _serde :: de :: Visitor < 'de > for __Visitor < 'de , T > where Vec < ( T :: AccountId , Vec < u8 > ) > : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: serde :: de :: DeserializeOwned { type Value = GenesisConfig < T > ; fn expecting ( & self , __formatter : & mut _serde :: __private :: Formatter ) -> _serde :: __private :: fmt :: Result { _serde :: __private :: Formatter :: write_str ( __formatter , "struct GenesisConfig" ) } # [ inline ] fn visit_seq < __A > ( self , mut __seq : __A ) -> _serde :: __private :: Result < Self :: Value , __A :: Error > where __A : _serde :: de :: SeqAccess < 'de > { let __field0 = match match _serde :: de :: SeqAccess :: next_element :: < Vec < ( T :: AccountId , Vec < u8 > ) > > ( & mut __seq ) { _serde :: __private :: Ok ( __val ) => __val , _serde :: __private :: Err ( __err ) => { return _serde :: __private :: Err ( __err ) ; } } { _serde :: __private :: Some ( __value ) => __value , _serde :: __private :: None => { return _serde :: __private :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct GenesisConfig with 1 element" ) ) ; } } ; _serde :: __private :: Ok ( GenesisConfig { nicks : __field0 , } ) } # [ inline ] fn visit_map < __A > ( self , mut __map : __A ) -> _serde :: __private :: Result < Self :: Value , __A :: Error > where __A : _serde :: de :: MapAccess < 'de > { let mut __field0 : _serde :: __private :: Option < Vec < ( T :: AccountId , Vec < u8 > ) > > = _serde :: __private :: None ; while let _serde :: __private :: Some ( __key ) = match _serde :: de :: MapAccess :: next_key :: < __Field > ( & mut __map ) { _serde :: __private :: Ok ( __val ) => __val , _serde :: __private :: Err ( __err ) => { return _serde :: __private :: Err ( __err ) ; } } { match __key { __Field :: __field0 => { if _serde :: __private :: Option :: is_some ( & __field0 ) { return _serde :: __private :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "nicks" ) ) ; } __field0 = _serde :: __private :: Some ( match _serde :: de :: MapAccess :: next_value :: < Vec < ( T :: AccountId , Vec < u8 > ) > > ( & mut __map ) { _serde :: __private :: Ok ( __val ) => __val , _serde :: __private :: Err ( __err ) => { return _serde :: __private :: Err ( __err ) ; } } ) ; } } } let __field0 = match __field0 { _serde :: __private :: Some ( __field0 ) => __field0 , _serde :: __private :: None => match _serde :: __private :: de :: missing_field ( "nicks" ) { _serde :: __private :: Ok ( __val ) => __val , _serde :: __private :: Err ( __err ) => { return _serde :: __private :: Err ( __err ) ; } } , } ; _serde :: __private :: Ok ( GenesisConfig { nicks : __field0 , } ) } } const FIELDS : & 'static [ & 'static str ] = & [ "nicks" ] ; _serde :: Deserializer :: deserialize_struct ( __deserializer , "GenesisConfig" , FIELDS , __Visitor { marker : _serde :: __private :: PhantomData :: < GenesisConfig < T > > , lifetime : _serde :: __private :: PhantomData , } ) } }
    };
    #[cfg(feature = "std")]
    impl<T: Trait> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                nicks: Default::default(),
            }
        }
    }
    #[cfg(feature = "std")]
    impl<T: Trait> GenesisConfig<T> {
        /// Build the storage for this module.
        pub fn build_storage(
            &self,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_decl_storage::hidden_include::sp_runtime::Storage,
            String,
        > {
            let mut storage = Default::default();
            self.assimilate_storage(&mut storage)?;
            Ok(storage)
        }
        /// Assimilate the storage for this module into pre-existing overlays.
        pub fn assimilate_storage(
            &self,
            storage : & mut self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_runtime :: Storage,
        ) -> std::result::Result<(), String> {
            self :: sp_api_hidden_includes_decl_storage :: hidden_include :: BasicExternalities :: execute_with_storage ( storage , | | { let extra_genesis_builder : fn ( & Self ) = | config | { let deposit = T :: ReservationFee :: get ( ) ; for ( acc , nick ) in & config . nicks { T :: Currency :: reserve ( acc , deposit . clone ( ) ) . expect ( "reservation for nick should not fail in genesis" ) ; < NameOf < T > > :: insert ( acc , ( nick . clone ( ) , deposit ) ) ; } } ; extra_genesis_builder ( self ) ; Ok ( ( ) ) } )
        }
    }
    #[cfg(feature = "std")]
    impl < T : Trait , __GeneratedInstance : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage < T , __GeneratedInstance > for GenesisConfig < T > { fn build_module_genesis_storage ( & self , storage : & mut self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_runtime :: Storage ) -> std :: result :: Result < ( ) , String > { self . assimilate_storage :: < > ( storage ) } }
    pub struct NameOf<T: Trait>(
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<
                (T,),
            >,
    );
    impl<T: Trait>
        self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
            Vec<u8>,
            BalanceOf<T>,
        )> for NameOf<T>
    {
        fn module_prefix() -> &'static [u8] {
            < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
        }
        fn storage_prefix() -> &'static [u8] {
            b"NameOf"
        }
    }
    impl<T: Trait>
        self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
            T::AccountId,
            (Vec<u8>, BalanceOf<T>),
        > for NameOf<T>
    {
        type Query = Option<(Vec<u8>, BalanceOf<T>)>;
        type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Twox64Concat;
        fn module_prefix() -> &'static [u8] {
            < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ( )
        }
        fn storage_prefix() -> &'static [u8] {
            b"NameOf"
        }
        fn from_optional_value_to_query(v: Option<(Vec<u8>, BalanceOf<T>)>) -> Self::Query {
            v.or_else(|| Default::default())
        }
        fn from_query_to_optional_value(v: Self::Query) -> Option<(Vec<u8>, BalanceOf<T>)> {
            v
        }
    }
    pub struct Module<T: Trait>(::frame_support::sp_std::marker::PhantomData<(T,)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
    impl<T: Trait> ::core::marker::StructuralPartialEq for Module<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl<T: Trait> ::core::marker::StructuralEq for Module<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<
                    ::frame_support::sp_std::marker::PhantomData<(T,)>,
                >;
            }
        }
    }
    impl<T: Trait> core::fmt::Debug for Module<T>
    where
        T: core::fmt::Debug,
    {
        fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
            fmt.debug_tuple("Module").field(&self.0).finish()
        }
    }
    impl<T: frame_system::Config + Trait>
        ::frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {
        fn on_runtime_upgrade() -> ::frame_support::dispatch::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_upgrade",
                            "pallet_nicks_migration::deprecated",
                            ::tracing::Level::TRACE,
                            Some("frame/nicks-migration/src/lib.rs"),
                            Some(278u32),
                            Some("pallet_nicks_migration::deprecated"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            frame_support::traits::PalletVersion {
                major: 2u16,
                minor: 0u8,
                patch: 1u8,
            }
            .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
            <<T as frame_system::Config>::DbWeight as ::frame_support::traits::Get<_>>::get()
                .writes(1)
        }
    }
    impl<T: frame_system::Config + Trait>
        ::frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Module<T>
    {
    }
    impl<T: frame_system::Config + Trait>
        ::frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Module<T>
    {
    }
    #[cfg(feature = "std")]
    impl<T: Trait> ::frame_support::traits::IntegrityTest for Module<T> {}
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {}
    /// Dispatchable calls.
    ///
    /// Each variant of this enum maps to a dispatchable function from the associated module.
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        #[codec(skip)]
        __PhantomItem(
            ::frame_support::sp_std::marker::PhantomData<(T,)>,
            ::frame_support::Never,
        ),
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Trait> _parity_scale_codec::Encode for Call<T> {}
        impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T> {}
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Trait> _parity_scale_codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
                }
            }
        }
    };
    impl<T: Trait> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
            match *self {
                Call::__PhantomItem(_, _) => {
                    ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
    }
    impl<T: Trait> ::frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Call::__PhantomItem(_, _) => {
                    ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &[]
        }
    }
    pub use ::frame_support::traits::GetPalletVersion as _;
    impl<T: Trait> ::frame_support::traits::GetPalletVersion for Module<T> {
        fn current_version() -> ::frame_support::traits::PalletVersion {
            frame_support::traits::PalletVersion {
                major: 2u16,
                minor: 0u8,
                patch: 1u8,
            }
        }
        fn storage_version() -> Option<::frame_support::traits::PalletVersion> {
            let key = ::frame_support::traits::PalletVersion::storage_key::<
                <T as frame_system::Config>::PalletInfo,
                Self,
            >()
            .expect("Every active pallet has a name in the runtime; qed");
            ::frame_support::storage::unhashed::get(&key)
        }
    }
    impl<T: Trait> ::frame_support::traits::OnGenesis for Module<T> {
        fn on_genesis() {
            frame_support::traits::PalletVersion {
                major: 2u16,
                minor: 0u8,
                patch: 1u8,
            }
            .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
        }
    }
    impl<T: Trait> ::frame_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
            }
        }
    }
    impl<T: Trait> ::frame_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
            }
        }
    }
    impl<T: Trait> ::frame_support::dispatch::Eq for Call<T> {}
    impl<T: Trait> ::frame_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::frame_support::dispatch::fmt::Formatter,
        ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error>
        {
            match *self {
                _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
            }
        }
    }
    impl<T: Trait> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = T::Origin;
        fn dispatch_bypass_filter(
            self,
            _origin: Self::Origin,
        ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Call::__PhantomItem(_, _) => {
                    ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ))
                }
            }
        }
    }
    impl<T: Trait> ::frame_support::dispatch::Callable<T> for Module<T> {
        type Call = Call<T>;
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
            &[]
        }
    }
    impl<T: 'static + Trait> Module<T> {
        #[doc(hidden)]
        #[allow(dead_code)]
        pub fn module_constants_metadata(
        ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
            &[]
        }
    }
    impl<T: Trait> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
        fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
            <&'static str as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
        }
    }
}
pub mod migration {
    use super::*;
    pub fn migrate_to_v2<T: Trait>() -> frame_support::weights::Weight {
        frame_support::debug::RuntimeLogger::init();
        if PalletVersion::get() == StorageVersion::V1Bytes {
            let count = deprecated::NameOf::<T>::iter().count();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                " >>> Updating NicksMigration storage. Migrating ",
                                " nicknames...",
                            ],
                            &match (&count,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ),
                        lvl,
                        &(
                            "pallet_nicks_migration::migration",
                            "pallet_nicks_migration::migration",
                            "frame/nicks-migration/src/lib.rs",
                            293u32,
                        ),
                    );
                }
            };
            NameOf::<T>::translate::<(Vec<u8>, BalanceOf<T>), _>(
                |k: T::AccountId, (nick, deposit): (Vec<u8>, BalanceOf<T>)| {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                ::core::fmt::Arguments::new_v1(
                                    &["     Migrated nickname for ", "..."],
                                    &match (&k,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Debug::fmt,
                                        )],
                                    },
                                ),
                                lvl,
                                &(
                                    "pallet_nicks_migration::migration",
                                    "pallet_nicks_migration::migration",
                                    "frame/nicks-migration/src/lib.rs",
                                    298u32,
                                ),
                            );
                        }
                    };
                    match nick.iter().rposition(|&x| x == b" "[0]) {
                        Some(ndx) => Some((
                            Nickname {
                                first: nick[0..ndx].to_vec(),
                                last: Some(nick[ndx + 1..].to_vec()),
                            },
                            deposit,
                        )),
                        None => Some((
                            Nickname {
                                first: nick,
                                last: None,
                            },
                            deposit,
                        )),
                    }
                },
            );
            PalletVersion::put(StorageVersion::V2Struct);
            let count = NameOf::<T>::iter().count();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                " <<< NicksMigration storage updated! Migrated ",
                                " nicknames \u{2705}",
                            ],
                            &match (&count,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ),
                        lvl,
                        &(
                            "pallet_nicks_migration::migration",
                            "pallet_nicks_migration::migration",
                            "frame/nicks-migration/src/lib.rs",
                            315u32,
                        ),
                    );
                }
            };
            T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
        } else {
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &[" >>> Unused migration!"],
                            &match () {
                                () => [],
                            },
                        ),
                        lvl,
                        &(
                            "pallet_nicks_migration::migration",
                            "pallet_nicks_migration::migration",
                            "frame/nicks-migration/src/lib.rs",
                            320u32,
                        ),
                    );
                }
            };
            0
        }
    }
}
