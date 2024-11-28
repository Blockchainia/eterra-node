use crate::{mock::*, Card, ElementalDefenses, Error};
use sp_runtime::traits::Hash;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, EterraCard, RuntimeOrigin};
	use frame_support::{assert_err, assert_ok};

	#[test]
	fn create_card_works() {
		new_test_ext().execute_with(|| {
			let stats = Card {
				level: 1,
				hp: 100,
				mp: 50,
				attack: 30,
				defense: 20,
				magic_strength: 40,
				magic_defense: 25,
				agility: 15,
				spirit: 10,
				luck: 5,
				elemental_defenses: ElementalDefenses {
					earth: 5,
					water: 10,
					wind: 15,
					fire: 20,
					lightning: 25,
					ice: 30,
				},
			};

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
			let stats = Card {
				level: 1,
				hp: 100,
				mp: 50,
				attack: 30,
				defense: 20,
				magic_strength: 40,
				magic_defense: 25,
				agility: 15,
				spirit: 10,
				luck: 5,
				elemental_defenses: ElementalDefenses {
					earth: 5,
					water: 10,
					wind: 15,
					fire: 20,
					lightning: 25,
					ice: 30,
				},
			};

			let card_id = <Test as frame_system::Config>::Hashing::hash_of(&stats);

			// Create a card
			assert_ok!(EterraCard::create_card(RuntimeOrigin::signed(1), stats.clone()));

			// Try creating a duplicate
			assert_err!(
				EterraCard::create_card(RuntimeOrigin::signed(1), stats),
				Error::<Test>::CardAlreadyExists
			);

			// Ensure duplicate wasn't added
			assert_eq!(
				EterraCard::cards(card_id),
				Some(Card {
					level: 1,
					hp: 100,
					mp: 50,
					attack: 30,
					defense: 20,
					magic_strength: 40,
					magic_defense: 25,
					agility: 15,
					spirit: 10,
					luck: 5,
					elemental_defenses: ElementalDefenses {
						earth: 5,
						water: 10,
						wind: 15,
						fire: 20,
						lightning: 25,
						ice: 30,
					}
				})
			);
		});
	}
}
