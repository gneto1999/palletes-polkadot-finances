#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;
    use frame_support::pallet_prelude::Encode;
    use frame_support::pallet_prelude::Decode;

    #[pallet::config]
    pub trait Config: frame_system::Config + TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub enum Category {
        Alimentacao,
        Transporte,
        Lazer,
        Saude,
        Educacao,
        Cobrancas,
        Outros,
    }

    impl Default for Category {
        fn default() -> Self {
            Category::Outros
        }
    }

    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, TypeInfo)]
    pub struct Expense {
        pub id: u64,
        pub title: Vec<u8>,
        pub description: Vec<u8>,
        pub amount: u64,
        pub date: Vec<u8>,
        pub category: Category,
    }

    #[pallet::storage]
    #[pallet::getter(fn expense)]
    pub type Expenses<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, Expense, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_expense_id)]
    pub type NextExpenseId<T> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ExpenseCreated(u64),
        ExpenseUpdated(u64),
        ExpenseDeleted(u64),
    }

    #[pallet::error]
    pub enum Error<T> {
        ExpenseNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_expense())]
        pub fn create_expense(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            amount: u64,
            date: Vec<u8>,
            category: Category, 
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            if NextExpenseId::<T>::get() == 0 {
                NextExpenseId::<T>::put(1);
            }
            let expense_id = NextExpenseId::<T>::get();
            let expense = Expense {
                id: expense_id,
                title,
                description,
                amount,
                date,
                category,
            };
            <Expenses<T>>::insert(expense_id, expense);
            <NextExpenseId<T>>::put(expense_id + 1);
            Self::deposit_event(Event::ExpenseCreated(expense_id));
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::update_expense())]
        pub fn update_expense(
            origin: OriginFor<T>,
            expense_id: u64,
            title: Vec<u8>,
            description: Vec<u8>,
            amount: u64,
            date: Vec<u8>,
            category: Category,
        ) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let mut expense = <Expenses<T>>::get(expense_id).ok_or(Error::<T>::ExpenseNotFound)?;
            expense.title = title;
            expense.description = description;
            expense.amount = amount;
            expense.date = date;
            expense.category = category;
            <Expenses<T>>::insert(expense_id, expense);
            Self::deposit_event(Event::ExpenseUpdated(expense_id));
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::delete_expense())]
        pub fn delete_expense(origin: OriginFor<T>, expense_id: u64) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let _expense = <Expenses<T>>::get(expense_id).ok_or(Error::<T>::ExpenseNotFound)?;
            <Expenses<T>>::remove(expense_id);
            Self::deposit_event(Event::ExpenseDeleted(expense_id));
            Ok(())
        }
    }
}