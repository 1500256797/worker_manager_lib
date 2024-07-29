use tokio::task::JoinError;
#[derive(Debug, thiserror::Error)]
pub enum WorkerManagerError {
    #[error("Task error: {0}")]
    TaskError(#[from] JoinError),
}
