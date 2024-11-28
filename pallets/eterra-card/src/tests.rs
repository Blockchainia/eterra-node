use crate::{mock::*, Card, Error};
use sp_runtime::traits::Hash;

#[cfg(test)]
mod tests {
	use super::*; // Import the necessary items from the library/module
	use crate::mock::{new_test_ext, EterraCard, RuntimeOrigin};
	use frame_support::{assert_err, assert_ok};

	#[test]
	fn create_card_works() {
		new_test_ext().execute_with(|| {
			let stats = Card { level: 1, hp: 100, mp: 50 };

			// Create a card
			assert_ok!(EterraCard::create_card(RuntimeOrigin::signed(1), stats.clone()));

			// Verify storage
			let card_id = <Test as frame_system::Config>::Hashing::hash_of(&stats);
			assert_eq!(EterraCard::cards(card_id), Some(stats));
		});
	}

	#[test]
	fn create_card_fails_if_duplicate() {
		new_test_ext().execute_with(|| {
			let stats = Card { level: 1, hp: 100, mp: 50 };

			let card_id = <Test as frame_system::Config>::Hashing::hash_of(&stats);

			// Create a card
			assert_ok!(EterraCard::create_card(RuntimeOrigin::signed(1), stats.clone()));

			// Try creating a duplicate
			assert_err!(
				EterraCard::create_card(RuntimeOrigin::signed(1), stats),
				Error::<Test>::CardAlreadyExists
			);

			// Ensure duplicate wasn't added
			assert_eq!(EterraCard::cards(card_id), Some(Card { level: 1, hp: 100, mp: 50 }));
		});
	}
}
