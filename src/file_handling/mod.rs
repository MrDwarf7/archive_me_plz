mod mover;
mod processor;

pub use mover::Mover;
pub use processor::Processor;

// use crate::Result;
// use std::path::{Path, PathBuf};
// use async_trait::async_trait;

// #[async_trait]
// pub trait FileOperation {
//     async fn process_dir(&self) -> Result<Vec<PathBuf>>;
//     async fn move_files(&self, files: Path) -> Result<()>;
// }
