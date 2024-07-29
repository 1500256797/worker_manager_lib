use async_trait::async_trait;
use std::sync::Arc;
use anyhow::Result;

use crate::errors::WorkerManagerError;
#[async_trait]
pub trait Task: Send + Sync {
    async fn run(&self);
}

#[derive(Clone)]
pub struct Worker {
    task: Arc<Box<dyn Task>>,
    name: String,
}

impl Worker {
    pub fn new(task: impl Task + 'static, name: String) -> Self {
        Worker {
            task: Arc::new(Box::new(task)),
            name,
        }
    }

    pub async fn start(&self) {
        self.task.run().await;
    }

    pub async fn start_with_log(&self) {
        println!("Starting worker: {}", self.name);
        self.task.run().await;
        println!("Worker finished: {}", self.name);
    }
}

pub struct WorkerManager {
    workers: Vec<Worker>,
}
impl WorkerManager {
    pub fn new() -> Self {
        WorkerManager {
            workers: Vec::new(),
        }
    }

    pub fn add_worker(&mut self, worker: Worker) {
        self.workers.push(worker);
    }

    pub async fn start(&self) ->Result<(),WorkerManagerError> {
        let mut tasks = Vec::new();

        for worker in &self.workers {
            let worker = worker.clone();
            let task = tokio::spawn(async move {
                worker.start().await;
            });
            tasks.push(task);
        }

        for task in tasks {
            if let Err(e) = task.await {
                return Err(WorkerManagerError::TaskError(e));
            }
        }
        Ok(())
    }

    pub async fn start_with_log(&self) ->Result<(),WorkerManagerError> {
        let mut tasks = Vec::new();

        for worker in &self.workers {
            let worker = worker.clone();
            let task = tokio::spawn(async move {
                worker.start_with_log().await;
            });
            tasks.push(task);
        }

        for task in tasks {
            if let Err(e) = task.await {
                return Err(WorkerManagerError::TaskError(e));
            }
        }
        Ok(())
    }
}
