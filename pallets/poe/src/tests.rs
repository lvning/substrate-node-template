use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoEModule::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim = 
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoEModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoEModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn create_claim_failed_when_claim_too_long() {
	new_test_ext().execute_with(|| {
		let claim = vec![0; 513];

		assert_noop!(
			PoEModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimTooLong
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoEModule::create_claim(Origin::signed(1), claim.clone());

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		
		let _ = PoEModule::revoke_claim(Origin::signed(1), claim.clone());

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Option::None
		);
	})
}

#[test]
fn revoke_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoEModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoEModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn revoke_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_noop!(
			PoEModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoEModule::create_claim(Origin::signed(1), claim.clone());

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		
		let _ = PoEModule::transfer_claim(Origin::signed(1), claim.clone(), 2);

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoEModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoEModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoEModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}