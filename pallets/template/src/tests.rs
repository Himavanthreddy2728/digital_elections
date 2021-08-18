use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_stores_correctly() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::store_land(Origin::signed(1),"Vamshi", [1,2,3,4]));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::proof(), Some("Vamshi", [1,2,3,4]));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
