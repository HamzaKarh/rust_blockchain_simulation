use crate::blockchain::Blockchain;
use std::borrow::BorrowMut;
use std::sync::mpsc::{channel, TryRecvError};
use std::time::{Duration, SystemTime};
use std::{io, thread};
use std::io::Read;
use std::sync::mpsc;
mod blockchain;

const BLOCK_MINING_TIME: Duration = Duration::from_secs(10);

fn main() {
    let mut running = true;
    let mut action: Vec<u8> = Vec::new();
    let (send, recv) = channel();

    let thr = thread::spawn(move || {
        let mut command = String::new();

        println!("Hello! Welcome to the blockchain interactive CLI !");
        println!("\n-------------------------------------------------\n\n\n");
        println!("There are several commands at your disposal : \n");
        println!("Type <<start_node>> to start the blockchain\n");
        println!("Type <<create_account>> to create a new account on the blockchain\n");
        println!("Type <<transfer>> to transfer funds from one account to another");
        println!("Type <<read_balance>> to read the balance of an existing account");
        println!("Type <<exit>> or press CTRL-C to leave this CLI and end the blockchain");
        'console: while running {

            command.clear();
            io::stdin().read_line(&mut command).unwrap();

            // Handling commands in main loop
            match command.as_str() {
                "start_node\n" => {
                    // Starting a new thread for the blockchain
                    action.push(0);

                }
                "create_account\n" => {
                    action.push(1);
                }
                "transfer\n" => {
                    let mut ok = false;
                    while !ok {
                        action.push(2);
                        println!("Please enter sending account number for transfer");
                        let mut sender = String::new();
                        io::stdin().read_line(&mut sender).unwrap();
                        println!("Please enter receiving account number for transfer");
                        let mut receiver = String::new();
                        io::stdin().read_line(&mut receiver).unwrap();
                        println!("Please enter funds to transfer");
                        let mut val = String::new();
                        io::stdin().read_line(&mut val).unwrap();
                        // Checking if all the command arguments are in the right format
                        // If not, continue asking for the arguments
                        for i in [sender.trim(), receiver.trim(), val.trim()] {
                            match i.parse::<u8>() {
                                Ok(v) => {
                                    action.push(v);
                                }
                                Err(..) => {
                                    println!("Error: this was not an integer: {}", i);
                                    break;
                                }
                            };
                            ok = true;

                        }
                        if !ok {
                            action = Vec::new();
                        }
                    }

                }
                "read_balance\n" => {
                    let mut ok = false;
                        while !ok {
                            println!("test");
                            action.push(3);
                            println!("Please type the id of an account you would like to check");
                            let mut sender = String::new();
                            io::stdin().read_line(&mut sender).unwrap();
                            // Checking if all the command arguments are in the right format
                            // If not, continue asking for the arguments
                            // for i in [sender, receiver, val] {

                            match sender.trim().parse::<u8>() {
                                Ok(v) => {
                                    action.push(v);
                                    ok = true;
                                }
                                Err(..) => {
                                    println!("Error: this was not an integer: {}", sender);
                                    // ok = false;
                                    // break;
                                }

                            };
                            // }


                        }
                }
                "exit\n" => {
                    action.push(4);
                    running = false;
                }
                _ => {
                    println!("Unknown command {}, please try again", command);
                }
            }
            send.send(action.to_vec()).unwrap();
            action.clear();

            println!("\n\n\n-------------------------------------------------\n\n\n");
        }
    });

    let mut b = Blockchain::new();
    while !b.running  {
        match recv.try_recv() {
            Ok(i) => {
                if i.first() == None{
                    continue;
                }
                if *i.first().unwrap() == 0 as u8{
                    b.set_running(true);
                } else if  *i.first().unwrap() == 4 as u8{
                    println!("Exiting without starting the blockchain? Alright... bye!");
                    return
                }
                else {
                    println!(" \n \n \n !!!! Error : Please start the blockchain first");
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(200));
                continue;
            }
        }

    }
    let start_time = SystemTime::now();;
    while b.running {
        match recv.try_recv() {
            Ok(i) => {
                handle_commands(i, &mut b);
            }
            Err(_) => {}
        }
        if start_time.elapsed().unwrap().as_secs() % BLOCK_MINING_TIME.as_secs()  < 1{
            b.mine();
            thread::sleep(Duration::from_secs(1));
        }


    }

    println!("EXITING....");

    thr.join().unwrap();


}

fn handle_commands(commands: Vec<u8>, b: &mut Blockchain) {
    // for i in commands {
    match commands[0] {
        1 => b.create_account(),

        2 => b.transfer(commands[1] as u64, commands[2] as u64, commands[3] as u128),

        3 => b.read_balance(commands[1] as u64),

        4 => b.set_running(false),

        _ => println!("Error: an unknown command slipped through the cracks command : {}", commands[0]),
    }
    // }
}
