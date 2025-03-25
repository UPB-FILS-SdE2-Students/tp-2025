use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
 
    let number = Box::new(10);
    println!("Valeur dans la Box : {}", number);

  
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Compteur mis à jour par un thread : {}", num);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Valeur finale du compteur : {}", *counter.lock().unwrap());
}
