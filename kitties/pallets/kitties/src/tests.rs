use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		
		let kitty_id = 0;
		let account_id = 1;

		Balances::force_set_balance(RuntimeOrigin::root(), account_id, 10000000);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"abcdefgh"));
		
		// check the emitted event of fn create
		System::assert_last_event(Event::KittyCreated{ who: account_id, kitty_id: kitty_id, kitty: KittiesModule::kitties(kitty_id).unwrap()}.into());
		
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id), *b"abcdefgh"),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with( || {

		let kitty_id = 0;
		let account_id = 1;
		let another_account_id = 2;

		Balances::force_set_balance(RuntimeOrigin::root(), account_id, 10000000);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"abcdefgh"));

		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		
		assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(another_account_id), account_id, kitty_id), Error::<Test>::NotOwner);
		
		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), another_account_id, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(another_account_id));
	})
}

#[test]
fn breed_workes() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		Balances::force_set_balance(RuntimeOrigin::root(), account_id, 10000000);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, *b"abcdefgh"),
			Error::<Test>::SameParentsId
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, *b"abcdefgh"),
			Error::<Test>::InvalidKittyId
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"abcdefgh"));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"abcdefgh"));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, *b"abcdefgh"));

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));
	});
}


#[test]
fn set_price_and_buy_works() {
	new_test_ext().execute_with(|| {
		
		let kitty_id = 0;
		let account_id = 1;
		let another_account_id = 2;

		Balances::force_set_balance(RuntimeOrigin::root(), account_id, 10000000);
		Balances::force_set_balance(RuntimeOrigin::root(), another_account_id, 10000000);

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), *b"abcdefgh"));
		
		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		assert_ok!(KittiesModule::set_price(RuntimeOrigin::signed(account_id), kitty_id, 100));

		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(another_account_id), kitty_id));
	});
}



