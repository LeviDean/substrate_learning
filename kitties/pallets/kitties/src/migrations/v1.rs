use crate::{Config, Pallet, Kitties, Kitty, KittyId};
use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
};
use frame_system::pallet_prelude::*;
use frame_support::{migration::storage_key_iter, Blake2_128Concat};

#[derive(Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct V1Kitty(pub [u8; 16]);

pub fn migrate<T: Config>() -> Weight {
    let on_chain_version = Pallet::<T>::on_chain_storage_version();
    let current_version = Pallet::<T>::current_storage_version();

    if on_chain_version != 0 {
        return Weight::zero();
    }

    if current_version != 1 {
        return Weight::zero();
    }

    let module = Kitties::<T>::module_prefix();
    let item = Kitties::<T>::storage_prefix();

    for (index, kitty) in storage_key_iter::<KittyId, V1Kitty, Blake2_128Concat>(module, item).drain(){
        let new_kitty = Kitty{
            dna: kitty.0,
            name: *b"init",
        };
        Kitties::<T>::insert(index, &new_kitty);
    }

    Weight::zero()
}
