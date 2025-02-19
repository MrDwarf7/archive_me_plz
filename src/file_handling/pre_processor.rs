use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use chrono::Utc;
use futures::stream::{self, StreamExt};
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, warn};

use crate::filetypes::Extensions;
use crate::prelude::ARCHIVE_FOLDER_NAME;
use crate::{Error, Result, UserInput};

#[derive(Debug, PartialEq)]
pub struct PreProcessor<'a> {
    input: &'a UserInput,
    ext: Vec<Extensions>,
}

impl<'a> PreProcessor<'a> {
    pub fn new(input: &'a UserInput) -> Self {
        let ext = Extensions::generate_set();
        PreProcessor { input, ext }
    }
}

// #[async_trait]
impl PreProcessor<'_> {
    // #[tracing::instrument]
    pub async fn process(&self) -> Result<Vec<PathBuf>> {
        if self.input.folder_path.is_dir() {
            let files = self.process_dir().await;
            match files {
                Ok(files) => {
                    match files.len() {
                        0 => {
                            debug!("No files found in given directory");
                            Err(Error::NoFilesFoundInGivenDir)
                        }
                        _ => Ok(files),
                    }
                }
                Err(e) => {
                    debug!("Error: {:?}", e);
                    Err(e)
                }
            }
        } else {
            Err(Error::InvalidInput)
        }
    }

    // #[tracing::instrument]
    async fn process_dir(&self) -> Result<Vec<PathBuf>> {
        let mut files = vec![];
        let mut handles = vec![];

        let mut entries = tokio::fs::read_dir(&self.input.folder_path).await?;
        println!("Reading directory: {:?}", &self.input.folder_path);

        while let Some(entry) = entries.next_entry().await? {
            handles.push(self.process_file(entry.path()));
        }

        let max_len = handles.len();

        let results = stream::iter(handles)
            .buffer_unordered(max_len)
            .collect::<Vec<_>>()
            .await;

        info!("Potential items found to be processed: {:?}", results.len());

        results.iter().flatten().for_each(|path| {
            if path == &Some(self.input.folder_path.join(ARCHIVE_FOLDER_NAME)) {
                warn!("Skipping archive folder");
                return;
            }

            if let Some(path) = path {
                files.push(path.to_owned());
            }
        });

        info!("Total files processed via PreProcessor<'a>: {:?}", files.len());
        // println!("Files processed: {:?}", files.len());

        if files.len() >= self.input.required_min_files {
            // DEBUG:
            // let print_files = files.clone();
            // tokio::task::spawn(async move {
            //     Self::print_files(print_files).await.unwrap();
            // })
            // .await?;

            Ok(files.clone())
        } else {
            warn!("Failure during files.len() >= self.input.required_min_files: {:?}", files.len());
            Err(Error::NoFilesOutsideOfGivenBounds)
        }
    }

    // #[tracing::instrument]
    async fn process_file(&self, path: PathBuf) -> Result<Option<PathBuf>> {
        debug!("Processing file: {:?}", &path);

        let ext = match Extensions::from_extension(path.extension().unwrap_or(OsStr::new(""))) {
            Some(ext) => ext,
            None => return Ok(None),
        };
        debug!("Extension: {:?}", &ext);

        if self.ext.contains(&ext) && self.is_recent(&path).await.unwrap_or(false) {
            debug!("File is recent and supported: {:?}", path);
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

    #[allow(dead_code)]
    async fn print_files(files: Vec<PathBuf>) -> Result<()> {
        let std_out = tokio::io::stdout();
        let mut buffer = tokio::io::BufWriter::new(std_out);

        for file in files {
            buffer
                .write_all(format!("File: {}\n", file.to_str().unwrap()).as_bytes())
                .await?;
        }
        buffer.flush().await?;
        Ok(())
    }

    // #[tracing::instrument]
    pub async fn is_recent(&self, path: &Path) -> Result<bool> {
        let metadata = tokio::fs::metadata(path).await?;
        debug!("Metadata: {:?}", metadata);

        let modified_dt = metadata.modified()?;

        let modified_date = chrono::DateTime::<Utc>::from(modified_dt).naive_utc().date();

        debug!("Modified date: {:?}", modified_date);

        Ok(modified_date <= self.input.oldest_to_keep)
    }
}
