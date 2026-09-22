use crate as pallet_pq_staking;
use frame::testing_prelude::*;
use polkadot_sdk::polkadot_sdk_frame as frame;

construct_runtime! {
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        PqStaking: pallet_pq_staking,
    }
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type Block = MockBlock<Test>;
    type AccountId = u64;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

impl crate::Config for Test {
    type Currency = Balances;
    type MinStake = ConstU128<10>;
    type RewardPerInference = ConstU128<5>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 10_000.into()), (2, 10_000.into()), (3, 10_000.into())],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}