use tokio::time::{sleep, Duration};

async fn run_task(id: usize, duration_ms: u64) -> Result<String, String> {
    sleep(Duration::from_millis(duration_ms)).await;

    if duration_ms > 800 {
        Err(format!("Task {} timed out", id))
    } else {
        Ok(format!("Task {} finished in {} ms", id, duration_ms))
    }
}

#[tokio::main]
async fn main() {
    let tasks = vec![
        (1, 300),
        (2, 1200),
        (3, 500),
        (4, 900),
        (5, 200),
    ];

    let mut handles = Vec::new();

    for (id, duration) in tasks {
        let handle = tokio::spawn(run_task(id, duration));
        handles.push(handle);
    }

    let mut success = 0;
    let mut failure = 0;

    for handle in handles {
        match handle.await {
            Ok(result) => match result {
                Ok(msg) => {
                    println!("{}", msg);
                    success += 1;
                }
                Err(err) => {
                    println!("{}", err);
                    failure += 1;
                }
            },
            Err(join_err) => {
                println!("Task panicked: {}", join_err);
                failure += 1;
            }
        }
    }

    println!("Successful tasks: {}", success);
    println!("Failed tasks: {}", failure);
}
