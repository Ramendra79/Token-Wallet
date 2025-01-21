use candid::{CandidType, Deserialize};
use ic_cdk::export_candid;
use ic_cdk_macros::*;
use std::collections::HashMap;

// Define the Account Structure
#[derive(CandidType, Deserialize, Clone, Debug)]
struct Account {
    balance: u64,
}

// Global State for Accounts
static mut ACCOUNTS: Option<HashMap<String, Account>> = None;

// Initialize the Canister
#[init]
fn init() {
    unsafe {
        ACCOUNTS = Some(HashMap::new());
    }
}

// Create an Account
#[update]
fn create_account(owner: String, initial_balance: u64) {
    unsafe {
        let accounts = ACCOUNTS.as_mut().unwrap();
        accounts.insert(owner, Account { balance: initial_balance });
    }
}

#[update]
fn send_tokens(from: String, to: String, amount: u64) -> String {
    unsafe {
        let accounts = ACCOUNTS.as_mut().unwrap();
        if let Some(sender_account) = accounts.get_mut(&from) {
            if sender_account.balance >= amount {
                sender_account.balance -= amount;

                let receiver_account = accounts.entry(to.clone()).or_insert(Account { balance: 0 });
                receiver_account.balance += amount;

                return format!("Transaction successful: {} sent {} tokens to {}", from, amount, to);
            } else {
                return String::from("Insufficient balance!");
            }
        }
        String::from("Sender account does not exist!")
    }
}

// Receive Tokens
#[update]
fn receive_tokens(owner: String, amount: u64) -> String {
    unsafe {
        let accounts = ACCOUNTS.as_mut().unwrap();
        let receiver_account = accounts.entry(owner.clone()).or_insert(Account { balance: 0 });
        receiver_account.balance += amount;

        format!("{} received {} tokens. New balance: {}", owner, amount, receiver_account.balance)
    }
}

// Get Account Balance
#[query]
fn get_balance(owner: String) -> u64 {
    unsafe {
        let accounts = ACCOUNTS.as_ref().unwrap();
        if let Some(account) = accounts.get(&owner) {
            return account.balance;
        }
        0
    }
}

export_candid!();