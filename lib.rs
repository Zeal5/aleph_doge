#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
pub mod AlephDoge {

    #[ink(storage)]
    pub struct AlephDoge {
        total_supply: Balance,
        balances: ink::storage::Mapping<AccountId, Balance>,
        contract_balance: Balance,
    }

    impl AlephDoge {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut balances = ink::storage::Mapping::new();
            let caller = Self::env().caller();
            balances.insert(caller, &initial_supply);
            Self {
                total_supply: initial_supply,
                balances,
                contract_balance: 0,
            }
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or(0)
        }

        #[ink(message, payable)]
        pub fn mint(&mut self) {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();
            let tokens_to_issue = amount;
            assert!(
                tokens_to_issue <= amount,
            );
            let token_balance = self.balance_of(caller);
            self.balances
                .insert(caller, &(token_balance + tokens_to_issue));
            self.total_supply += tokens_to_issue;

            if tokens_to_issue < amount {
                let refund_amount = amount - tokens_to_issue;
                self.env()
                    .transfer(caller, refund_amount)
                    .expect("Failed to refund excess payment");
            }

        }
    }
}