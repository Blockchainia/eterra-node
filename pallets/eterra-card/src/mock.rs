use crate as pallet_eterra_card;
use frame_support::{parameter_types, traits::GenesisBuild, weights::RuntimeDbWeight};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	generic,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = generic::UncheckedExtrinsic<u64, RuntimeCall, (), ()>;
type Block = generic::Block<Header, UncheckedExtrinsic>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		EterraCard: pallet_eterra_card,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub DbWeight: RuntimeDbWeight = RuntimeDbWeight {
		read: 1_000,   // Customize these weights as needed
		write: 10_000, // Customize these weights as needed
	};
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = DbWeight;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_eterra_card::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_eterra_card::GenesisConfig::<Test> { cards: vec![] }
		.build_storage()
		.unwrap();
	t.into()
}
