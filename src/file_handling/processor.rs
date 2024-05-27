use crate::{Error, Result, UserInput};
use chrono::Utc;
use futures::stream::{self, StreamExt};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::io::AsyncWriteExt;

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

impl<'a> Processor<'a> {
    pub async fn is_recent(&self, path: &Path) -> Result<bool> {
        let metadata = tokio::fs::metadata(path).await?;

        let modified_dt = metadata.modified()?;

        let modified_date = chrono::DateTime::<Utc>::from(modified_dt)
            .naive_utc()
            .date();

        Ok(modified_date <= self.input.oldest_to_keep)
    }
}

// #[async_trait]
impl<'a> Processor<'a> {
    pub async fn process(&self) -> Result<Vec<PathBuf>> {
        if self.input.folder_path.is_dir() {
            self.process_dir().await
        } else {
            Err(Error::InvalidInput)
        }
    }

    async fn process_dir(&self) -> Result<Vec<PathBuf>> {
        let mut files = vec![];
        let mut handles = vec![];

        let mut entries = tokio::fs::read_dir(&self.input.folder_path).await?;
        println!("Processing files...");

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            handles.push(self.process_file(path));
        }

        let max_len = handles.len();

        let results = stream::iter(handles)
            .buffer_unordered(max_len)
            .collect::<Vec<_>>()
            .await;

        results.iter().flatten().for_each(|path| {
            if let Some(path) = path {
                files.push(path.to_owned());
            }
        });

        println!("Files processed: {:?}", files.len());

        if files.len() >= self.input.required_min_files {
            let print_files = files.clone();
            tokio::task::spawn(async move {
                Self::print_files(print_files).await.unwrap();
            })
            .await?;

            Ok(files.clone())
        } else {
            Err(Error::InvalidInput)
        }
    }

    async fn process_file(&self, path: PathBuf) -> Result<Option<PathBuf>> {
        println!("Processing file: {:?}", &path);

        let ext = match Extensions::from_extension(path.extension().unwrap_or(OsStr::new(""))) {
            Some(ext) => ext,
            None => return Ok(None),
        };
        println!("Extension: {:?}", &ext);

        if self.ext.contains(&ext) && self.is_recent(&path).await.unwrap_or(false) {
            println!("File is recent and supported: {:?}", path);
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

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
}
