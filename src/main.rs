#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::*;
use ink::storage;

#[ink::contract]
pub mod chaincerty {
    #[ink(storage)]
    pub struct ChainCerty {
        skills: storage::Mapping<AccountId, Vec<String>>,
    }

    impl ChainCerty {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                skills: storage::Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn add_skill(&mut self, skill: String) {
            let caller = self.env().caller();
            let mut user_skills = self.skills.get(&caller).unwrap_or_default();
            user_skills.push(skill);
            self.skills.insert(&caller, &user_skills);
        }

        #[ink(message)]
        pub fn get_skills(&self, user: AccountId) -> Vec<String> {
            self.skills.get(&user).unwrap_or_default()
        }
    }
}
