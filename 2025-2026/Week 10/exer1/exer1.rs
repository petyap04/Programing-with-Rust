use std::thread;
use std::sync::Arc;

#[derive(Clone)]
enum Predicate {
    GreaterThan100,
    Even,
    Prime,
    DivisibleBy7,
}

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

impl Predicate {
    fn check(&self, n: i32) -> bool {
        match self {
            Predicate::GreaterThan100 => n > 100,
            Predicate::Even => n % 2 == 0,
            Predicate::Prime => is_prime(n),
            Predicate::DivisibleBy7 => n % 7 == 0,
        }
    }
}

fn parallel_filter(data: Vec<i32>, n_threads: usize, predicate: Predicate) -> Vec<i32> {
    let chunk_size = (data.len() + n_threads - 1) / n_threads;

    let predicate = Arc::new(predicate);
    let data = Arc::new(data);

    let mut handles = Vec::new();

    for i in 0..n_threads {
        let predicate = predicate.clone();
        let data = data.clone();

        let start = i * chunk_size;
        let end = (start + chunk_size).min(data.len());

        let handle = thread::spawn(move || {
            let mut result = Vec::new();
            for &value in &data[start..end] {
                if predicate.check(value) {
                    result.push(value);
                }
            }
            result
        });

        handles.push(handle);
    }

    let mut final_result = Vec::new();
    for handle in handles {
        let partial = handle.join().expect("Thread failed");
        final_result.extend(partial);
    }

    final_result
}

fn main() {
    let numbers: Vec<i32> = (1..200).collect();

    let filtered1 = parallel_filter(numbers.clone(), 4, Predicate::GreaterThan100);
    println!(">100 = {:?}", filtered1);

    let filtered2 = parallel_filter(numbers.clone(), 4, Predicate::Even);
    println!("Even = {:?}", filtered2);

    let filtered3 = parallel_filter(numbers.clone(), 4, Predicate::Prime);
    println!("Prime = {:?}", filtered3);

    let filtered4 = parallel_filter(numbers.clone(), 4, Predicate::DivisibleBy7);
    println!("Divisible by 7 = {:?}", filtered4);
}