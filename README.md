Purpose
-------

The code is an implementation of a basic token management system on the Internet Computer (IC) using Rust. It allows users to create accounts, send tokens, receive tokens, and check account balances.

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Code Components
---------------

1.	Libraries Imported
          o	candid::{CandidType, Deserialize}: For serialization and deserialization of data types.
          o	ic_cdk::export_candid: To export the Candid interface.
          o	ic_cdk_macros::*: Provides macros like #[init], #[update], and #[query] to define canister entry points.
          o	std::collections::HashMap: Used to maintain a mapping of account owners to their account details.
2.	Account Structure
          o	struct Account: Contains one field:
               	balance (u64): The balance of the account.
3.	Global State
o	A static, mutable HashMap named ACCOUNTS stores all accounts. It uses Option to handle initialization.
4.	Functions
o	init: Initializes the ACCOUNTS global variable as an empty HashMap.
o	create_account: Creates a new account with a given owner and initial balance.
o	send_tokens: Facilitates the transfer of tokens from one account to another.
	Checks if the sender has sufficient balance.
	Updates the sender and receiver accounts.
o	receive_tokens: Adds tokens to an account, creating it if it doesn't already exist.
o	get_balance: Retrieves the balance of a specific account.
5.	Candid Interface
o	export_candid!(): Generates the Candid interface for the canister, enabling integration with frontend tools and other canisters.

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------



Features
--------

1.	Account Management
 
o	Accounts are identified by a String representing the owner.
o	The balance is stored as an unsigned 64-bit integer (u64).
2.	Token Operations
o	Tokens can be sent between accounts using send_tokens.
o	Tokens can be added to an account using receive_tokens.
3.	Safety Checks
o	Ensures that the sender has sufficient funds before transferring tokens.
o	Creates new accounts automatically when receiving tokens for non-existent accounts.
4.	Concurrency Model
o	Uses the Internet Computer's update and query calls (#[update] and #[query]).

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


Example Usage and Output Creating Accounts
------------------------------------------

create_account("Harsh".to_string(), 1000);
create_account("Aryan".to_string(), 500);
Output:
•	Two accounts, "Harsh" with 1000 tokens and "Aryan" with 500 tokens, are created.

![alt text](https://github.com/Ramendra79/Token-Wallet/blob/main/createAccount.png)

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


Checking Balances
----------------


get_balance("Harsh".to_string()); // 1000
get_balance("Aryan".to_string()); // 500
Output:
•	"Harsh"'s balance: 1000
•	"Aryan"'s balance: 500

![alt text](https://github.com/Ramendra79/Token-Wallet/blob/main/getBalance.png)

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


Sending Tokens
--------------


send_tokens("Harsh".to_string(), "Aryan".to_string(), 200);
Output:
•	"Transaction successful: Harsh sent 200 tokens to Aryans" Balances after transaction:
•	"Harsh": 800
•	"Aryan": 700

![alt text](https://github.com/Ramendra79/Token-Wallet/blob/main/sendToken.png)

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


Receiving Tokens
------------------

receive_tokens("Ashu".to_string(), 300);
Output:
•	"Ashu received 300 tokens. New balance: 300"

![alt text](https://github.com/Ramendra79/Token-Wallet/blob/main/receiveToken.png)

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------



Error Cases
---------------

1.	Insufficient Balance

send_tokens("Hasrh".to_string(), "Aryan".to_string(), 2000);
Output:
"Insufficient balance!"
2.	Non-existent Sender

send_tokens("Ishu".to_string(), "Harsh".to_string(), 50);
Output:
"Sender account does not exist!"

![alt text](https://github.com/Ramendra79/Token-Wallet/blob/main/outputLog.png)

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------



Considerations and Suggestions
---------------------------------

1.	Safety and Synchronization
 
o	Using unsafe for global mutable state (ACCOUNTS) is error-prone. Consider using RefCell, Mutex, or other safe concurrency primitives to avoid race conditions.
2.	Error Handling
o	Replace unwrap with proper error handling mechanisms to avoid panics.
3.	Performance
o	The use of HashMap is efficient for account lookups, but scaling and storage limits should be considered for real-world canisters.
4.	Scalability
o	The code assumes the entire state can fit in memory. For large-scale systems, consider mechanisms for partitioning state.
5.	Candid Interface
o	Ensure the Candid file generated by export_candid!() is available for integration testing and frontend development.

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


Summary
-------

The code provides a simple yet functional token management system. It demonstrates core concepts of the Internet Computer, such as update and query calls, and integrates Rust's features with the IC platform effectively. While functional, improving safety and scalability will enhance its production
readiness.

--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

