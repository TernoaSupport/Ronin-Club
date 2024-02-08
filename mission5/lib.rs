#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod ronin_mission5_user {
    use ink::storage::Mapping;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use scale::{Decode, Encode};

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum CrudError {
        MessageAlreadyCreatedBySender,
        SenderNotFound,
    }

    #[ink(storage)]
    pub struct CrudContract {
        messages: Mapping<AccountId, String>,
        senders: Vec<AccountId>,
    }

    impl CrudContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let creator = Self::env().caller();

            let mut messages = Mapping::new();
            let init_message = String::from("I created my CRUD contract");
            messages.insert(creator, &init_message);

            let mut senders = Vec::new();
            senders.push(creator);

            Self { messages, senders }
        }

        #[ink(message)]
        pub fn create_message(&mut self, message: String) -> Result<(), CrudError> {
            let caller = self.env().caller();

            if self.messages.contains(caller) {
                return Err(CrudError::MessageAlreadyCreatedBySender);
            }

            /* TODO: (optional) Verify if message has a minimal length of 10 */
            self.messages.insert(caller, &message);
            self.senders.push(caller);

            Ok(())
        }

        #[ink(message)]
        pub fn read_message_from(&mut self, sender: AccountId) -> Result<String, CrudError> {
            if self.messages.contains(&sender) {
                Ok(self.messages.get(&sender).unwrap())
            } else {
                return Err(CrudError::SenderNotFound);
            }
        }

        #[ink(message)]
        pub fn read_all_messages(&mut self) -> Vec<(AccountId, String)> {
            /* TODO: (optional) Once we can remove messages
                -> Return Error if there is no message to display
            */

            /* TODO: (optional) Convert (AccountId, String) to custom struct */
            let mut all_messages = Vec::<(AccountId, String)>::new();

            for a in self.senders.clone() {
                let account = a.clone();
                if self.messages.contains(account) {
                    all_messages.push((account, self.messages.get(account).unwrap()));
                }
            }

            return all_messages;
        }

        /* TODO: update_message() function
            -> sender can update his own message
            -> verify if message is identical
        */

        /* TODO: delete_message() function
            -> sender can delete his own message
        */
    }
}
