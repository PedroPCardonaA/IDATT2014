 // Import all the relevant libraries.
use std::thread;
use std::collections::{LinkedList};
use std::io;
use std::sync::{Arc,Mutex};
use std::cmp::max;

fn main() {

    // Vector tht contain all the thread that will be created.
    let mut threads = Vec::new();
    // Define an ARC and Mutex linked list to store the different prime numbers 
    // found by the different threads.

    /* An "ARC" is a smart pointer that keeps track of the number of references to the underlying
    object, and it will automatically deallocated the object when the last reference to it is dropped.
    This enables one to share ownership of the object across multiple threads without the risk of data
    race conditions, which can occur when multiple threads try to access the same data simultaneously 
    without proper synchronization.

    In Rust, a "Mutex" (Short for "Mutual exclusion") is a type of synchronization primitive that allows
    one to protect shared data from concurrent access. A "Mutex" can be thought of as a lock
    that can be acquired by a single thread at a time.
     */

    let shared_list: Arc<Mutex<LinkedList<i128>>> = Arc::new(Mutex::new(LinkedList::new()));

    // Method to read the inputs from a user.
    println!("Enter start number: ");
    let start = read_input();
    println!("Enter end number: ");
    let mut end = read_input();
    println!("Enter number of threads number: ");
    let no_threads = read_input();
    let difference;
    end = end +1;
    if end >= start{
        difference = end - start;
    } else{
        difference = -end + start;
    }
    

    // Algorithm that creates the wanted amount threads and divide equally them
    // across the define sector of number defined by the user.
    for x in 0..no_threads{
        let list_handle = shared_list.clone();
        threads.push(thread::spawn(move || {
            println!("Thread # {} starts!", (x+1));
            for i in (start + (difference/no_threads)*x)..((start +(difference/no_threads))+ (difference/no_threads)*(x)){
                let mut prime = true;
                
                for j in 2..f64::sqrt(i as f64) as i128{
                    if i%j==0{
                        prime  = false;
                    } 
                }
                if prime {
                    let mut list = list_handle.lock().unwrap();
                    list.push_back(i);
                }
            }
            println!("Thread # {} is done!", (x+1));
        }));
        
    }

    

    // Barrier that force the main thread to wait for the other threads to be finished.
    for thread in threads{
        let _ = thread.join();
    }

    //Sorting of the result list and printing of the elements
    let list = shared_list.lock().unwrap();
    let list = list.clone();
    let mut sorted_vec: Vec<i128> = list.into_iter().collect();
    sorted_vec.sort();


    for(i, val)in sorted_vec.iter().enumerate(){
        if i == sorted_vec.len() -1 {
            print!("{}", val)
        } else {
            print!("{} - ", val)
        }
    }

    println!("");
    println!("Total number of primes: {}", sorted_vec.len());

}

//Method that read inputs input.

fn read_input() -> i128{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut num:i128= input.trim().parse().unwrap();
    num= max(num,-num);
    println!("You entered number: {}.", num);
    return num;
    
}
