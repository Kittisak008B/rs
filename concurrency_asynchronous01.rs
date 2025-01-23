// Asynchronous

// Asynchronous programming is a programming paradigm that allows a program to perform multiple tasks concurrently without creating a new thread for each task. 
// This approach enhances efficiency and reduces system resource usage, especially in operations that involve waiting

// | Feature               | Threading                                       | Asynchronous Programming                         |
// |-----------------------|-------------------------------------------------|--------------------------------------------------|
// | **Execution Model**   | Creates a new thread for each task              | Uses a single or fewer threads for multiple tasks|
// | **Resource Usage**    | High due to multiple threads                    | More efficient, quick task switching             |
// | **Context Switching** | High, due to frequent context switching         | Low, as tasks yield control                      |
// | **Best Suited For**   | CPU-intensive tasks requiring parallel execution| I/O-bound tasks (e.g., network, file I/O)        |


use tokio::time::{sleep, Duration};

#[tokio::main] // Marks the main function as asynchronous and runs it using the Tokio runtime.
async fn main() {
    println!("Start");

    // Spawn asynchronous tasks to run in the background.
    let task1 = tokio::spawn(async {
        println!("Task 1 start");
        sleep(Duration::from_secs(2)).await; // Simulate an asynchronous delay of 2 seconds.
        println!("Task 1 end");
        "Result from Task 1" // Return a value from the task.
    });

    let task2 = tokio::spawn(async {
        println!("Task 2 start");
        sleep(Duration::from_secs(1)).await; 
        println!("Task 2 end");
        "Result from Task 2" // Return a value from the task.
    });

    // Use `tokio::join!` to await both tasks concurrently.
    let (result1, result2) = tokio::join!(task1, task2);

    // Unwrap the results since `tokio::spawn` returns `Result<T, JoinError>`.
    println!("{}", result1.unwrap()); 
    println!("{}", result2.unwrap()); 

    println!("End");
}

// Start
// Task 1 start
// Task 2 start
// Task 2 end
// Task 1 end
// Result from Task 1
// Result from Task 2
// End

// Task 1 and Task 2 start running concurrently.
// Task 2 finishes first (because it sleeps for only 1 second).
// Task 1 finishes later (because it sleeps for 2 seconds).
// tokio::join! ensures both tasks finish before continuing execution.
