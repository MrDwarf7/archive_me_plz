#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use futures::{StreamExt, stream};
use rayon::prelude::*;
use tokio::fs::create_dir_all;
use tracing::{debug, error, info};

use crate::prelude::ARCHIVE_FOLDER_NAME;
use crate::{Error, Result, UserInput};

#[derive(Clone, Debug)]
pub struct Mover<'a> {
    input: &'a UserInput,
}

impl<'a> Mover<'a> {
    pub fn new(input: &'a UserInput) -> Self {
        Mover { input }
    }
}

impl Mover<'_> {
    // PERF: This is currently the fastest method for moving files by far -
    // #[tracing::instrument]
    pub async fn par_move_files_copy(&self, files: &[PathBuf]) -> Result<()> {
        let archive = Arc::new(self.input.folder_path.clone().join(ARCHIVE_FOLDER_NAME));
        create_dir_all(&*archive).await?;

        let chunk_size = (files.len() / 6).max(1);
        let file_chunks: Vec<&[PathBuf]> = files.chunks(chunk_size).collect();
        let handle = tokio::runtime::Handle::current();

        let results: Vec<_> = file_chunks
            .into_par_iter()
            .map(|chunk| {
                let archive = Arc::clone(&archive);
                let handle = handle.clone();
                handle.block_on(async move {
                    stream::iter(chunk)
                        .map(|file| {
                            let archive = Arc::clone(&archive);
                            async move {
                                let file_name = file.file_name().unwrap();

                                info!("Moving file: {:?}", &file_name);

                                let archive_path = archive.join(file_name);
                                tokio::fs::copy(file, &archive_path).await?;
                                tokio::task::yield_now().await;

                                // REFACTOR: this shouldn't check the entire
                                // list of files in the dir every single time
                                // it moves a file, check that after we finish iterating
                                match Self::check_files(&[file.to_owned()], &[archive_path.to_owned()])
                                    .await
                                    .is_ok()
                                {
                                    true => Ok(tokio::fs::remove_file(file).await),
                                    false => Err(Error::CopiedFilesDontMatch),
                                }
                            }
                        })
                        .buffer_unordered(chunk.len())
                        .collect::<Vec<_>>()
                        .await
                })
            })
            .flatten()
            .collect();

        let mut success_count = 0;
        for result in results.into_iter().flatten() {
            match result {
                Ok(_) => success_count += 1,
                Err(e) => error!("Error: {e}"),
            }
        }

        if success_count >= self.input.required_min_files {
            Ok(())
        } else {
            Err(Error::InvalidInput)
        }
    }
}

// region:		--- helper
impl Mover<'_> {
    pub async fn check_files(files_original: &[PathBuf], files_archive: &[PathBuf]) -> Result<()> {
        let original_names = files_original
            .par_iter()
            .map(|file| file.file_name().unwrap())
            .collect::<Vec<_>>();
        let archive_names = files_archive
            .par_iter()
            .map(|file| file.file_name().unwrap())
            .collect::<Vec<_>>();

        let mut success_count = 0;
        for (original, archive) in original_names.iter().zip(archive_names.iter()) {
            if original == archive {
                success_count += 1;
                debug!("Files match: {:?} == {:?}", original, archive);
            } else {
                error!("Error: Files don't match: {:?} != {:?}", original, archive);
            }
        }

        assert_eq!(original_names.len(), archive_names.len());
        assert_eq!(original_names.len(), success_count);
        assert_eq!(archive_names.len(), success_count);

        if success_count == original_names.len() {
            Ok(())
        } else {
            Err(Error::CopiedFilesDontMatch)
        }
    }
}
// endregion:	--- helper

// region:		--- depr. methods
impl Mover<'_> {
    // DEPR:
    pub async fn par_move_files(&self, files: &[PathBuf]) -> Result<()> {
        let archive = Arc::new(self.input.folder_path.clone().join(ARCHIVE_FOLDER_NAME));
        create_dir_all(&*archive).await?;

        let chunk_size = (files.len() / 6).max(1);
        let file_chunks: Vec<&[PathBuf]> = files.chunks(chunk_size).collect();
        let handle = tokio::runtime::Handle::current();

        let results: Vec<_> = file_chunks
            .into_par_iter()
            .map(|chunk| {
                let archive = Arc::clone(&archive);
                let handle = handle.clone();
                handle.block_on(async move {
                    stream::iter(chunk)
                        .map(|file| {
                            let archive = Arc::clone(&archive);
                            async move {
                                let file_name = file.file_name().unwrap();
                                println!("Moving file: {:?}", file_name);
                                let archive_path = archive.join(file_name);
                                tokio::fs::rename(file, archive_path).await
                            }
                        })
                        .buffer_unordered(chunk.len())
                        .collect::<Vec<_>>()
                        .await
                })
            })
            .collect();

        let mut success_count = 0;
        for result in results.into_iter().flatten() {
            match result {
                Ok(_) => success_count += 1,
                Err(e) => eprintln!("Error: {e}"),
            }
        }

        if success_count >= self.input.required_min_files {
            Ok(())
        } else {
            Err(Error::InvalidInput)
        }
    }

    // DEPR:
    pub async fn chunked_move_files(&self, files: &[PathBuf]) -> Result<()> {
        let archive = Arc::new(self.input.folder_path.clone().join(ARCHIVE_FOLDER_NAME));
        create_dir_all(&*archive).await?;

        let max_concur_tasks = 50;

        let results = stream::iter(files)
            .map(|file| {
                let archive = Arc::clone(&archive);
                async move {
                    let file_name = file.file_name().unwrap();
                    let archive_path = archive.join(file_name);
                    let start = tokio::time::Instant::now();
                    let result = tokio::fs::rename(file, archive_path).await;
                    let duration = start.elapsed();
                    println!("Moved file: {:?} in {:?}", file_name, duration);
                    result
                }
            })
            // .buffer_unordered(file_len)
            .buffer_unordered(max_concur_tasks)
            .collect::<Vec<_>>()
            .await;

        let mut success_count = 0;
        for result in results {
            match result {
                Ok(_) => success_count += 1,
                Err(e) => eprintln!("Error: {e}"),
            }
        }

        if success_count >= self.input.required_min_files {
            Ok(())
        } else {
            Err(Error::InvalidInput)
        }
    }
}
// endregion:	--- depr. methods
