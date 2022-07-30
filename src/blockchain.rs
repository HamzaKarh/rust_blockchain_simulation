use std::collections::VecDeque;
use std::u64::MAX;

#[derive(PartialEq, Copy, Clone)]
pub struct Account {
    pub public_key: u64,
    pub funds: u128,
}

impl Account {
    pub fn new() -> Account {
        return Account {
            public_key: MAX,
            funds: 0,
        };
    }
}

#[derive(PartialEq, Copy, Clone)]
enum TransactionType {
    AccountCreation,
    FundTransfer,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Transaction {
    id: i128,
    kind: TransactionType,
    sender: Account,
    receiver: Account,
    value: u128,
}

pub struct Block {
    pub id: u32,
    pub transactions: VecDeque<Transaction>,
}

pub struct Blockchain {
    pub running: bool,
    pub miner: Account,
    pub accounts: Vec<Account>,
    pub transaction_queue: VecDeque<Transaction>,
    pub mined_blocks: VecDeque<Block>,
    pub nb_transaction: u64,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut b = Blockchain {
            running: false,
            miner: Account {
                public_key: 0,
                funds: 10000000000000000000000000000,
            },
            accounts: Vec::new(),
            transaction_queue: VecDeque::new(),
            mined_blocks: VecDeque::new(),
            nb_transaction: 0,
        };
        b.accounts.push(b.miner);
        b
    }

    pub fn transfer(&mut self, send: u64, receive: u64, val: u128) {
        let mut ok_status = true;
        let mut s = Account::new();
        match &self.get_account(send) {
            None => {
                println!("Error, sending account ({}) not found", send);
                ok_status = false;
            }
            Some(i) => s = **i,
        }
        let mut r = Account::new();
        match &self.get_account(receive) {
            None => {
                println!("Error, receiving account ({}) not found", receive);
                ok_status = false;
            }
            Some(i) => r = **i,
        }
        if ok_status {
            let transaction: Transaction = Transaction {
                id: -1, //Initializing at -1. We will give the real transaction id during mining
                kind: TransactionType::FundTransfer,
                sender: s,
                receiver: r,
                value: val,
            };
            self.transaction_queue.push_back(transaction);
        }
    }

    pub fn create_account(&mut self, id: u64, funds: u128) {
        let transaction: Transaction = Transaction {
            id: id as i128, //Initializing at -1. We will give the real transaction id during mining
            kind: TransactionType::AccountCreation,
            sender: self.miner,
            receiver: self.miner,
            value: funds,
        };
        self.transaction_queue.push_front(transaction);
    }

    pub fn mine(&mut self) {
        let mut block_transactions: VecDeque<Transaction> = VecDeque::new();
        let mut current_transaction: Transaction;
        // Looping through all queued transactions
        for _ in 0..self.transaction_queue.len() {
            current_transaction = self.transaction_queue.pop_front().unwrap();
            // Switch to handle both types of transactions
            match current_transaction.kind {
                // Account creation
                TransactionType::AccountCreation => {
                    let account_id = current_transaction.id;
                    let account_funds = current_transaction.value;
                    if !self.account_exists(account_id as u64) {
                        let account: Account = Account {
                            public_key: account_id as u64,
                            funds: account_funds,
                        };
                        println!(
                            "Successfully created account {} with {} coins",
                            account.public_key, account.funds
                        );
                        self.accounts.push(account);
                    } else {
                        println!("Failed to create account {}, ID already exists", account_id);
                    }
                }
                // Fund transfer
                TransactionType::FundTransfer => {
                    /*
                    Raise exception if any of the account doesnt exist
                    */
                    if !self.accounts.contains(&current_transaction.sender) {
                        println!(
                            "Error, sending account {} does not currently exist in the blockchain ",
                            current_transaction.sender.public_key
                        );
                        continue;
                    } else if !self.accounts.contains(&current_transaction.receiver) {
                        println!("Error, receiving account {} does not currently exist in the blockchain ",
                                 current_transaction.receiver.public_key);
                        continue;
                    }
                    /*
                    Raise exception if sending account doesn't have the funds
                    */
                    if current_transaction.sender.funds < current_transaction.value {
                        println!(
                            "Error, insufficient funds on account {}, cannot send {}",
                            current_transaction.sender.public_key, current_transaction.value
                        );
                        continue;
                    }
                    self.get_account(current_transaction.sender.public_key)
                        .unwrap()
                        .funds -= current_transaction.value;
                    self.get_account(current_transaction.receiver.public_key)
                        .unwrap()
                        .funds += current_transaction.value;

                    println!(
                        "Successfully transferred {} from account {} to account {}",
                        current_transaction.value,
                        current_transaction.sender.public_key,
                        current_transaction.receiver.public_key
                    );
                }
            }
            // If we get here, everything is in order, so we give the transaction an actual ID and push it into the block transactions queue
            current_transaction.id = self.nb_transaction as i128;
            self.nb_transaction += 1;
            block_transactions.push_back(current_transaction);
        }

        let block: Block = Block {
            id: self.mined_blocks.len() as u32,
            transactions: block_transactions,
        };

        println!(
            "Successfully mined new block. Block number {}",
            self.mined_blocks.len()
        );
        self.mined_blocks.push_back(block);
    }

    pub fn read_balance(&mut self, public_key: u64) -> i128 {
        match &self.get_account(public_key) {
            None => {
                println!("Error, account ({}) not found", public_key);
            }
            Some(i) => {
                println!("Balance for account {} : {}", public_key, i.funds);
                return i.funds as i128;
            }
        }
        -1
    }

    pub fn set_running(&mut self, val: bool) {
        self.running = val;
    }

    pub fn account_exists(&mut self, id: u64) -> bool {
        for i in 1..self.accounts.len() {
            if self.accounts[i].public_key == id {
                return true;
            }
        }
        false
    }

    pub fn get_account(&mut self, id: u64) -> Option<&mut Account> {
        for i in 0..self.accounts.len() {
            if self.accounts[i].public_key == id {
                return Some(&mut self.accounts[i]);
            }
        }
        println!("Did not find account {}", id);
        None
    }
}
