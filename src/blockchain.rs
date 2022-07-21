use std::{thread, time};
use std::collections::VecDeque;
use std::time::Duration;


#[derive(PartialEq, Copy, Clone)]
pub struct Account {
    pub public_key:u64,
    pub funds: u128
}

#[derive(PartialEq, Copy, Clone)]
enum TransactionType {
    AccountCreation,
    FundTransfer
}

#[derive(PartialEq, Copy, Clone)]
pub struct Transaction{
    id: i128,
    kind: TransactionType,
    sender: Account,
    receiver: Account,
    value: u128
}

// #[derive(Copy, Clone)]
pub struct Block{
    pub id: u32,
    pub transactions: VecDeque<Transaction>,
}

// #[derive(Copy, Clone)]
pub struct Blockchain {
    pub running: bool,
    pub miner: Account,
    pub accounts: Vec<Account>,
    pub transaction_queue: VecDeque<Transaction>,
    pub mined_blocks: VecDeque<Block>,
    pub nb_transaction: u64,
}

impl Blockchain{
    pub fn new() ->  Blockchain{
        let mut b = Blockchain{
            running: false,
            miner: Account { public_key: 0, funds: 10000000000000000000000000000 },
            accounts: Vec::new(),
            transaction_queue: VecDeque::new(),
            mined_blocks: VecDeque::new(),
            nb_transaction: 0
        };
        b.accounts.push(b.miner);
        b
    }



    pub fn transfer(&mut self, send: u64, receive: u64, val: u128){
        let & s = &self.accounts[send as usize];
        let & r = &self.accounts[receive as usize];
        let transaction:Transaction  = Transaction{
            id: -1, //Initializing at -1. We will give the real transaction id during mining
            kind: TransactionType::FundTransfer,
            sender: s,
            receiver: r,
            value: val
        };
        self.transaction_queue.push_back(transaction);
        // println!("Current transactions : {:?}", self.transaction_queue);
    }

    pub fn create_account(&mut self){
        let transaction:Transaction  = Transaction{
            id: -1, //Initializing at -1. We will give the real transaction id during mining
            kind: TransactionType::AccountCreation,
            sender: self.miner,
            receiver: self.miner,
            value: 0
        };
        self.transaction_queue.push_front(transaction);
    }

    pub fn mine(&mut self){
        let mut block_transactions: VecDeque<Transaction> = VecDeque::new();
        let mut current_transaction: Transaction;
        // Looping through all queued transactions
        for i in 0..self.transaction_queue.len(){
            current_transaction = self.transaction_queue.pop_front().unwrap();
            // Switch to handle both types of transactions
            match current_transaction.kind {
                // Account creation
                TransactionType::AccountCreation => {
                    let account: Account = Account {
                        public_key: self.accounts.len() as u64,
                        funds: 0
                    };
                    println!("Successfully created account {}", account.public_key);
                    self.accounts.push(account);
                }
                // Fund transfer
                TransactionType::FundTransfer => {

                    /*
                    Raise exception if any of the account doesnt exist
                    */
                    if !self.accounts.contains(&current_transaction.sender) {
                        println!("Error, sending account {} does not currently exist in the blockchain ",
                                 current_transaction.sender.public_key);
                        continue;
                    }else if !self.accounts.contains(&current_transaction.receiver) {
                        println!("Error, receiving account {} does not currently exist in the blockchain ",
                                 current_transaction.receiver.public_key);
                        continue;
                    }
                    /*
                    Raise exception if sending account doesn't have the funds
                    */
                    if current_transaction.sender.funds < current_transaction.value {
                        println!("Error, insufficient funds on account {}, cannot send {}",
                                 current_transaction.sender.public_key, current_transaction.value);
                        continue;
                    }

                    self.accounts.get_mut(current_transaction.sender.public_key as usize ).unwrap().funds -= current_transaction.value;
                    self.accounts.get_mut(current_transaction.receiver.public_key as usize ).unwrap().funds += current_transaction.value;

                    println!("Successfully transferred {} from account {} to account {}",
                              current_transaction.value, current_transaction.sender.public_key, current_transaction.receiver.public_key);
                }

            }
            // If we get here, everything is in order, so we give the transaction an actual ID and push it into the block transactions queue
            current_transaction.id = self.nb_transaction as i128;
            self.nb_transaction += 1;
            block_transactions.push_back(current_transaction);
        }

        let block: Block = Block{
            id: self.mined_blocks.len() as u32,
            transactions: block_transactions
        };

        println!("Successfully mined new block. Block number {}", self.mined_blocks.len());
        self.mined_blocks.push_back(block);
    }

    pub fn read_balance(&self, public_key: u64){
        println!("Balance for account {} : {}",public_key, self.accounts[public_key as usize].funds);
    }

    pub fn set_running(&mut self, val: bool){
        self.running = val;
    }

}



