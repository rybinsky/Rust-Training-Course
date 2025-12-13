// This chapter is dedicated to the concurrency.

use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// THREADS & JOIN
// ================================================================================================

// ----- 1 --------------------------------------
// Spawn multiple threads to calculate squares of the provided numbers and collect the results.

pub fn calculate_squares(input_numbers: Vec<i32>) -> Vec<i32> {
    let mut handles = Vec::new();

    for n in input_numbers {
        handles.push(thread::spawn(move || n * n));
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

// ----- 2 --------------------------------------
// Implement a `parallel_prime_check` function that splits work across multiple threads.

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

/// Inputs:
/// - `numbers` - a `u64` vector of values which should be checked.
/// - `number_of_threads` - a number of threads you must use to *efficiently* distribute the values
///   from the numbers vector.
///
/// Outputs:
/// - `Vec<(u64, bool)>` is a vector of the provided values along with the boolean flag whether this
///   value is prime.
pub fn parallel_prime_check(numbers: Vec<u64>, number_of_threads: usize) -> Vec<(u64, bool)> {
    let chunk_size = (numbers.len() + number_of_threads - 1) / number_of_threads;
    let mut handles = Vec::new();

    for chunk in numbers.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            chunk.into_iter().map(|n| (n, is_prime(n))).collect::<Vec<_>>()
        }));
    }

    let mut result = Vec::new();
    for h in handles {
        result.extend(h.join().unwrap());
    }
    result
}

// MPSC CHANNELS
// ================================================================================================

// ----- 3 --------------------------------------
// Compute the factorial for each value in the provided vector.
// Use a separate thread for each computation.
// Send the factorial results to the main thread using a channel transmitter.
// Using a channel receiver, collect the resulting factorial values into a vector and return it from
// the function.

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

pub fn parallel_factorials(numbers: Vec<u32>) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();

    for n in numbers {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(factorial(n)).unwrap();
        });
    }

    drop(tx);

    rx.into_iter().collect()
}

// MUTEX + ARC
// ================================================================================================

// ----- 4 --------------------------------------
// Implement a `SharedCounter` struct with one `value: ?<i32>` field and methods:
// - `pub fn new(initial_value: i32) -> Self`, which creates a new instance of the `SharedCounter`.
// - `pub fn increment(&self)` which will increment the internal value.
// - `pub fn get_value(&self) -> i32` which will return the internal value.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(initial_value: i32) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial_value)),
        }
    }

    pub fn increment(&self) {
        let mut v = self.value.lock().unwrap();
        *v += 1;
    }

    pub fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

// ----- 5 --------------------------------------
// Simulate a bank account system with concurrent deposits and withdrawals.
//
// Implement a `BankAccount` struct with one `balance: ?<i32>` field and methods:
// - `pub fn new(initial_balance: i32) -> Self`, which creates a new instance of the `BankAccount`.
// - `pub fn deposit(&self, amount: i32)` which adds the provided amount to the balance.
// - `pub fn withdraw(&self, amount: i32) -> bool` which attempts to remove the provided amount from
//   the balance. If the balance have sufficient funds, it removes the provided amount and returns
//   `true`, otherwise returns `false`.
// - `pub fn get_balance(&self)` which returns the current balance.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct BankAccount {
    balance: Arc<Mutex<i32>>,
}

impl BankAccount {
    pub fn new(initial_balance: i32) -> Self {
        Self {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    pub fn deposit(&self, amount: i32) {
        let mut b = self.balance.lock().unwrap();
        *b += amount;
    }

    pub fn withdraw(&self, amount: i32) -> bool {
        let mut b = self.balance.lock().unwrap();
        if *b >= amount {
            *b -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_balance(&self) -> i32 {
        *self.balance.lock().unwrap()
    }
}

// FINAL BOSS: CHANNELS + MUTEX + ARC
// ================================================================================================

// ----- 6 --------------------------------------
// Implement a work queue where multiple workers consume tasks and send results back (a simple task
// distribution system).
//
// You will need to implement two procedures:
// - `worker(id: usize, task_receiver: ?<Receiver<i32>>, result_sender: ?<Sender<(usize, i32)>>)`,
//   which has the ID of the worker, the task receiver, which waits for the task to be provided to
//   this worker, and the result sender, which sends the computed result back to the main thread.
//   This procedure should:
//   - Loop over all incoming tasks from `task_receiver`.
//   - For each task, compute the square of the value this task provided.
//   - Send the result back via `result_sender` along with the worker’s ID:
//    `(worker_id, result_value)`.
//   - Decide by your own whether this procedure should return something or not, and if should --
//    what exactly.
//   - Use `Arc` or `Mutex` if needed.
// - `run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)>` which has the
//   vector of tasks (just values, which square we should compute) and the total number of workers
//   which should be spawned. It returns the vector of worker IDs (`usize`) and the resulting value
//   computed by this worker (`i32`). This procedure should:
//   - Create two channels: for sending tasks to workers and for collecting results from workers.
//   - For each worker spawn a thread which runs the worker function, consuming tasks and sending
//     results.
//   - Send each task from the input list into the task_sender.
//   - Collect all results from the result_receiver into a vector and return it.

fn worker(
    worker_id: usize,
    task_receiver: Arc<Mutex<Receiver<i32>>>,
    result_sender: Sender<(usize, i32)>,
) {
    loop {
        let task = {
            let rx = task_receiver.lock().unwrap();
            rx.recv()
        };

        match task {
            Ok(task) => {
                let result = task * task;
                result_sender.send((worker_id, result)).unwrap();
            },
            Err(_) => break,
        }
    }
}

pub fn run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)> {
    let (task_tx, task_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    let task_rx = Arc::new(Mutex::new(task_rx));

    for id in 0..number_of_workers {
        let rx = Arc::clone(&task_rx);
        let tx = result_tx.clone();
        thread::spawn(move || {
            worker(id, rx, tx);
        });
    }

    for task in tasks {
        task_tx.send(task).unwrap();
    }

    drop(task_tx);
    drop(result_tx);

    result_rx.into_iter().collect()
}
