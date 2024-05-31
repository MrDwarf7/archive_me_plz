use crate::{prelude::ARCHIVE_FOLDER_NAME, Error, Result, UserInput};
use chrono::Utc;
use futures::stream::{self, StreamExt};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, warn};

#[derive(Debug, PartialEq)]
pub enum Extensions {
    Xlsx,
    Csv,
    Xls,
    Xlsm,
}

impl Extensions {
    pub fn generate_set() -> Vec<Self> {
        vec![
            Extensions::Xlsx,
            Extensions::Csv,
            Extensions::Xls,
            Extensions::Xlsm,
        ]
    }

    pub fn from_extension(ext: &OsStr) -> Option<Self> {
        let ext = ext.to_string_lossy().to_lowercase();

        match ext.as_str() {
            "xlsx" => Some(Extensions::Xlsx),
            "csv" => Some(Extensions::Csv),
            "xls" => Some(Extensions::Xls),
            "xlsm" => Some(Extensions::Xlsm),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Processor<'a> {
    input: &'a UserInput,
    ext: Vec<Extensions>,
}

impl<'a> Processor<'a> {
    pub fn new(input: &'a UserInput) -> Self {
        let ext = Extensions::generate_set();
        Processor { input, ext }
    }
}

// #[async_trait]
impl<'a> Processor<'a> {
    // #[tracing::instrument]
    pub async fn process(&self) -> Result<Vec<PathBuf>> {
        if self.input.folder_path.is_dir() {
            let files = self.process_dir().await;
            match files {
                Ok(files) => match files.len() {
                    0 => {
                        debug!("No files found in given directory");
                        Err(Error::NoFilesFoundInGivenDir)
                    }
                    _ => Ok(files),
                },
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

        info!("Total files processed via Processor<'a>: {:?}", files.len());
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
            warn!(
                "Failure during files.len() >= self.input.required_min_files: {:?}",
                files.len()
            );
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
        let mut std_out = tokio::io::stdout();

        for file in files {
            std_out.write_all(b"File: ").await?;
            std_out
                .write_all(file.to_str().unwrap().as_bytes())
                .await
                .unwrap();
            std_out.write_all(b"\n").await?;
        }

        std_out.flush().await?;
        Ok(())
    }

    // #[tracing::instrument]
    pub async fn is_recent(&self, path: &Path) -> Result<bool> {
        let metadata = tokio::fs::metadata(path).await?;
        debug!("Metadata: {:?}", metadata);

        let modified_dt = metadata.modified()?;

        let modified_date = chrono::DateTime::<Utc>::from(modified_dt)
            .naive_utc()
            .date();

        debug!("Modified date: {:?}", modified_date);

        Ok(modified_date <= self.input.oldest_to_keep)
    }
}
