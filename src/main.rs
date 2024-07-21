mod condition;
mod escrow;
mod transaction;
mod user;
mod utils;

use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;
use escrow::Escrow;
use utils::{ generate_users, random_condition, random_transaction };

fn main() {
    let escrow = Arc::new(Mutex::new(Escrow::new()));
    let users = generate_users(5);
    let handles: Vec<_> = users
        .into_iter()
        .map(|user| {
            let escrow_clone = Arc::clone(&escrow);
            thread::spawn(move || {
                let transaction = random_transaction(&user);
                {
                    let mut contract = escrow_clone.lock().unwrap();
                    contract.add_participant(user.clone());
                    contract.deposit(transaction);
                }
                thread::sleep(Duration::from_millis(100));
                let condition = random_condition();
                {
                    let mut contract = escrow_clone.lock().unwrap();
                    contract.meet_condition(condition);
                }
                thread::sleep(Duration::from_millis(100));
                {
                    let mut contract = escrow_clone.lock().unwrap();
                    match contract.release_funds(&user) {
                        Ok(amount) => println!("Successfully released: {}", amount),
                        Err(e) => println!("Failed to release funds: {}", e),
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
