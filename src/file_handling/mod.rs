mod mover;
mod pre_processor;

pub use mover::Mover;
pub use pre_processor::PreProcessor;

// use crate::Result;
// use std::path::{Path, PathBuf};
// use async_trait::async_trait;

// #[async_trait]
// pub trait FileOperation {
//     async fn process_dir(&self) -> Result<Vec<PathBuf>>;
//     async fn move_files(&self, files: Path) -> Result<()>;
// }
