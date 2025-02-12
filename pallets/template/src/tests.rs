#![cfg(test)]

use super::*;
use frame_support::{assert_ok, assert_noop, traits::{OnInitialize, OnFinalize}};
use sp_core::H256;
use frame_system as system;
use sp_runtime::{testing::Header, traits::{BlakeTwo256, IdentityLookup}};

// Configuração do ambiente de teste
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        ExpenseModule: pallet::{Pallet, Call, Storage, Event<T>},
    }
);

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = (); 
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = (); 
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = (); 
}

impl Config for Test {
    type RuntimeEvent = Event;
    type WeightInfo = ();
}

fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

#[test]
fn create_expense_works() {
    new_test_ext().execute_with(|| {
        let title = b"Aluguel".to_vec();
        let description = b"Pagamento do mês".to_vec();
        let amount = 1000;
        let date = b"2024-02-12".to_vec();
        let category = Category::Alimentacao;

        assert_ok!(ExpenseModule::create_expense(
            Origin::signed(1),
            title.clone(),
            description.clone(),
            amount,
            date.clone(),
            category.clone()
        ));

        let expense = ExpenseModule::expense(1).unwrap();
        assert_eq!(expense.title, title);
        assert_eq!(expense.amount, amount);
    });
}

#[test]
fn update_expense_works() {
    new_test_ext().execute_with(|| {
        let title = b"Aluguel".to_vec();
        let description = b"Pagamento do mês".to_vec();
        let amount = 1000;
        let date = b"2024-02-12".to_vec();
        let category = Category::Alimentacao;

        assert_ok!(ExpenseModule::create_expense(
            Origin::signed(1),
            title.clone(),
            description.clone(),
            amount,
            date.clone(),
            category.clone()
        ));

        let new_title = b"Supermercado".to_vec();
        let new_amount = 500;
        assert_ok!(ExpenseModule::update_expense(
            Origin::signed(1),
            1,
            new_title.clone(),
            description.clone(),
            new_amount,
            date.clone(),
            category.clone()
        ));

        let expense = ExpenseModule::expense(1).unwrap();
        assert_eq!(expense.title, new_title);
        assert_eq!(expense.amount, new_amount);
    });
}

#[test]
fn delete_expense_works() {
    new_test_ext().execute_with(|| {
        let title = b"Aluguel".to_vec();
        let description = b"Pagamento do mês".to_vec();
        let amount = 1000;
        let date = b"2024-02-12".to_vec();
        let category = Category::Alimentacao;

        assert_ok!(ExpenseModule::create_expense(
            Origin::signed(1),
            title.clone(),
            description.clone(),
            amount,
            date.clone(),
            category.clone()
        ));

        assert_ok!(ExpenseModule::delete_expense(Origin::signed(1), 1));
        assert_eq!(ExpenseModule::expense(1), None);
    });
}

#[test]
fn update_nonexistent_expense_fails() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ExpenseModule::update_expense(
                Origin::signed(1),
                99,
                b"Test".to_vec(),
                b"Description".to_vec(),
                500,
                b"2024-02-12".to_vec(),
                Category::Lazer
            ),
            Error::<Test>::ExpenseNotFound
        );
    });
}

#[test]
fn delete_nonexistent_expense_fails() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ExpenseModule::delete_expense(Origin::signed(1), 99),
            Error::<Test>::ExpenseNotFound
        );
    });
}
