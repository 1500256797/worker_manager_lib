pub mod traits;
pub mod errors;


#[cfg(test)]
mod tests {
    use crate::traits::{Task, Worker, WorkerManager};
    use async_trait::async_trait;
    struct MyTask1 {
    }

    #[async_trait]
    impl Task for MyTask1 {
        async fn run(&self) {
            println!("Running task...");
        }
    }

    // task two
    struct MyTask2 {
    }

    #[async_trait]
    impl Task for MyTask2 {
        async fn run(&self) {
            for i in 1..10 {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                println!("Running task2...{}", i);
            }
        }
    }
    
    
    
    #[tokio::test]
    async fn test_worker_manager() {
        let task = MyTask1 {};
        let worker = Worker::new(task, "test".to_string());

        let task2 = MyTask2 {};
        let worker2 = Worker::new(task2, "test2".to_string());
        let mut worker_manager = WorkerManager::new();
        worker_manager.add_worker(worker);
        worker_manager.add_worker(worker2);

        let res = worker_manager.start_with_log().await;
    }
}