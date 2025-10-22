use std::thread;

use crate::tasks::c10_concurrency::{
    BankAccount,
    SharedCounter,
    calculate_squares,
    parallel_factorials,
    parallel_prime_check,
    run_work_queue,
};

#[test]
fn test_calculate_squares() {
    let input_numbers = vec![1, -2, 3, -4];
    let squared_numbers = calculate_squares(input_numbers);

    assert_eq!(squared_numbers, vec![1, 4, 9, 16]);
}

#[test]
fn test_parallel_prime_check() {
    let numbers_to_check = vec![2, 3, 4, 17, 18, 19, 22];
    let answers_vec = vec![
        (2, true),
        (3, true),
        (4, false),
        (17, true),
        (18, false),
        (19, true),
        (22, false),
    ];
    let mut results = parallel_prime_check(numbers_to_check, 3);
    results.sort_unstable();

    assert_eq!(answers_vec, results);
}

#[test]
fn test_parallel_factorials() {
    let mut results = parallel_factorials(vec![3, 4, 5]);
    results.sort_unstable();

    assert_eq!(vec![6, 24, 120], results);
}

#[test]
fn test_shared_counter() {
    let counter = SharedCounter::new(0);
    let mut thread_handles = Vec::new();

    for _ in 0..5 {
        // let counter_clone = SharedCounter { value: Arc::clone(&counter.value) };
        let counter_clone = counter.clone();
        let thread_handle = thread::spawn(move || {
            counter_clone.increment();
        });
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    assert_eq!(counter.get_value(), 5);
}

#[test]
fn test_bank_account() {
    let account = BankAccount::new(100);
    let mut thread_handles = Vec::new();

    for _ in 0..3 {
        let account_clone = account.clone();
        let thread_handle = thread::spawn(move || {
            account_clone.deposit(50);
        });
        thread_handles.push(thread_handle);
    }

    for _ in 0..2 {
        let account_clone = account.clone();
        let thread_handle = thread::spawn(move || {
            account_clone.withdraw(30);
        });
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    assert_eq!(190, account.get_balance());
}

#[test]
fn test_run_work_queue() {
    let mut results = run_work_queue(vec![1, 2, 3, 4], 3);

    assert_eq!(results.len(), 4);
    results.sort_by_key(|(_, square)| *square);
    results.iter().zip(vec![1, 4, 9, 16]).for_each(|((_, result), square)| {
        assert_eq!(*result, square);
    });
}
