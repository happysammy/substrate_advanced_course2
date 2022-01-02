#![cfg(test)]

use std::ops::Sub;

use crate::{
	mock::*, pallet::{Error}, KittyCnt, assert_has_event
};
use frame_support::{assert_ok, assert_noop};

// fn events() -> Vec<Event> {
// 	let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();

// 	System::reset_events();

// 	evt
// }

// //获取最新事件
// fn last_event() -> Event {
// 	frame_system::Pallet::<Test>::events().pop().expect("Event expected").event
// }


//检验创世小猫是否存在
#[test]
fn should_build_genesis_kitties() {
	new_test_ext().execute_with(|| {
		// Check we have 2 kitties, as specified
		assert_eq!(SubstrateKitties::kitty_cnt(), 2);

		// Check owners own the correct amount of kitties
		let kitties_owned_by_1 = SubstrateKitties::kitties_owned(1);
		assert_eq!(kitties_owned_by_1.len(), 1);

		let kitties_owned_by_2 = SubstrateKitties::kitties_owned(2);
		assert_eq!(kitties_owned_by_2.len(), 1);

		// Check that kitties are owned correctly
		let kid1 = kitties_owned_by_1[0];
		let kitty1 = SubstrateKitties::kitties(kid1)
			.expect("Could have this kitty ID owned by acct 1");
		assert_eq!(kitty1.owner, 1);

		let kid2 = kitties_owned_by_2[0];
		let kitty2 = SubstrateKitties::kitties(kid2)
			.expect("Could have this kitty ID owned by acct 2");
		assert_eq!(kitty2.owner, 2);
	});
}

//创建小猫成功
#[test]
fn create_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// create a kitty with account #10.
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		// check that 3 kitties exists (together with the two from genesis)
		assert_eq!(SubstrateKitties::kitty_cnt(), 3);

		// check that account #10 owns 1 kitty
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 1);

		// check that some random account #5 does not own a kitty
		assert_eq!(SubstrateKitties::kitties_owned(5).len(), 0);

		// check that this kitty is specifically owned by account #10
		let hash = SubstrateKitties::kitties_owned(10)[0];
		let kitty = SubstrateKitties::kitties(hash).expect("should found the kitty");
		assert_eq!(kitty.owner, 10);
		assert_eq!(kitty.price, None);
	});
}

//创建小猫失败：小猫数量溢出
#[test]
fn create_failed_when_kittiescount_overflow() {
	new_test_ext().execute_with(|| {
		<KittyCnt<Test>>::put(u64::max_value());
		let account_id: u64 = 1;
		assert_noop!(SubstrateKitties::create_kitty(Origin::signed(account_id)), Error::<Test>::KittyCntOverflow);
	});
}


//创建小猫失败：一个账户不能超过3只小猫
#[test]
fn create_failed_when_owned_kitties_cnt_exceed_max() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(account_id))); //第二只 
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(account_id))); //第三只
		assert_noop!(SubstrateKitties::create_kitty(Origin::signed(account_id)), Error::<Test>::ExceedMaxKittyOwned);
	});
}



//转移小猫成功
#[test]
fn transfer_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// check that acct 10 own a kitty
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 1);
		let hash = SubstrateKitties::kitties_owned(10)[0];

		// acct 10 send kitty to acct 3
		assert_ok!(SubstrateKitties::transfer(Origin::signed(10), 3, hash));

		// acct 10 now has nothing
		assert_eq!(SubstrateKitties::kitties_owned(10).len(), 0);
		// but acct 3 does
		assert_eq!(SubstrateKitties::kitties_owned(3).len(), 1);
		let new_hash = SubstrateKitties::kitties_owned(3)[0];
		// and it has the same hash
		assert_eq!(hash, new_hash);
	});
}

//转移小猫失败:执行转移操作的账户不是小猫的拥有者
#[test]
fn transfer_failed_when_non_owned_kitty() {
	new_test_ext().execute_with(|| {
		let hash = SubstrateKitties::kitties_owned(1)[0];

		// account 0 cannot transfer a kitty with this hash.
		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(9), 2, hash),
			Error::<Test>::NotKittyOwner
		);
	});
}

// 繁殖小猫成功
#[test]
fn breed_kitty_should_work() {
	new_test_ext().execute_with(|| {
		
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(3)));
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(4)));
		let kid1_hash = SubstrateKitties::kitties_owned(3)[0];
		let kid2_hash = SubstrateKitties::kitties_owned(4)[0];
		assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(3), kid1_hash, kid2_hash));
	});
}

//卖出小猫成功
#[test]
fn sell_should_work() {
	new_test_ext().execute_with(|| {

		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
		let price: u64 = 5;
		let kitty_hash = SubstrateKitties::kitties_owned(1)[0];
		assert_ok!(SubstrateKitties::set_price(Origin::signed(1), kitty_hash, Some(price)));
		
	});
}

//买入小猫成功
#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
		let price: u64 = 5;
		let kitty_hash = SubstrateKitties::kitties_owned(1)[0];
		assert_ok!(SubstrateKitties::set_price(Origin::signed(1), kitty_hash, Some(price)));
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(2), kitty_hash, price));
	});
}








