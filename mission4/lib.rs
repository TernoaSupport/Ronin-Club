#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod discord_register {
    use ink::storage::Mapping;
    use ink_prelude::vec::Vec;
    use ink_prelude::string::String;
    use scale::{Encode, Decode};

    // Custom error types for the transaction contract
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum TransactionError {
        SubmissionLimitReached, // Error when submission limit is reached
        InvalidDiscordIDLength, // Error when the input is not 18 chars like Discord ID's
    }
    
    // The main struct for the transaction contract
    #[ink(storage)]
    pub struct TransactionContract {
        author_wallet: String,
        // Mapping of user addresses to a boolean indicating whether they have submitted a transaction
        submissions: Mapping<AccountId, bool>,
        // Vector to store the addresses of users who have submitted a transaction
        submitted_addresses: Vec<AccountId>,
        // Counter for the number of submissions
        submission_count: u32,

        discord_ids: Mapping<AccountId, Vec<u8>>, // Store Discord IDs
        discord_usernames: Mapping<AccountId, Vec<u8>>, // Store Discord Usernames
        all_discord_ids: Vec<Vec<u8>>, // Vector to store all submitted Discord IDs
    }

    impl TransactionContract {
        // Constructor to create a new TransactionContract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                // update the author wallet before building
                author_wallet: String::from("YOUR_RONIN_WALLET"),
                
                submissions: Mapping::new(),
                submitted_addresses: Vec::new(),
                submission_count: 0,
                discord_ids: Mapping::new(),
                discord_usernames: Mapping::new(),
                all_discord_ids: Vec::new(),
            }
        }

        #[ink(message)]
        pub fn submit_transaction(&mut self, discord_id: Vec<u8>, discord_username: Vec<u8>) -> Result<(), TransactionError> {
            let caller = self.env().caller(); // Get the wallet of the caller
        
            // Check if the submission count is more than 1. Only one submit for this example.
            if self.submission_count >= 1 {
                return Err(TransactionError::SubmissionLimitReached);
            }
        
            // Check if the Discord ID length is valid
            if discord_id.len() < 18 || discord_id.len() > 19 {
                return Err(TransactionError::InvalidDiscordIDLength);
            }
        
            // Add the Discord ID and Username to the respective vectors
            self.all_discord_ids.push(discord_id.clone());
            self.discord_ids.insert(caller, &discord_id.clone());
            self.discord_usernames.insert(caller, &discord_username.clone());


            // Add the caller to the submissions and submitted_addresses
            self.submissions.insert(caller, &true);
            self.submitted_addresses.push(caller);
            self.submission_count += 1;
        
            Ok(())
        }

        // Function to list all addresses that have submitted a transaction with their Discord IDs and Usernames
        #[ink(message)]
        pub fn list_submitted_users(&self) -> Vec<(String, AccountId, Vec<u8>, Vec<u8>)> {
            self.submitted_addresses.iter()
                .map(|&acc| (self.author_wallet.clone(),
                            acc, 
                             self.discord_ids.get(&acc).unwrap_or_default(),
                             self.discord_usernames.get(&acc).unwrap_or_default()))
                .collect()        
        }
    }
}
