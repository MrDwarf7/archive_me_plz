#![allow(clippy::blocks_in_conditions)]

use crate::{prelude::SUPPORTED_EXTENSIONS, Error, Result, UserInput};
use chrono::Utc;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Debug, PartialEq)]
pub enum Extensions {
    Xlsx,
    Csv,
    Xls,
    Xlsm,
}

impl Extensions {
    pub fn as_str(&self) -> String {
        let supported = SUPPORTED_EXTENSIONS;

        match self {
            Extensions::Xlsx => supported[0].to_string(),
            Extensions::Csv => supported[1].to_string(),
            Extensions::Xls => supported[2].to_string(),
            Extensions::Xlsm => supported[3].to_string(),
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
        let ext = vec![
            Extensions::Xlsx,
            Extensions::Csv,
            Extensions::Xls,
            Extensions::Xlsm,
        ];
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
    pub async fn process_dir(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        let se_arr: [String; 4] = [
            Extensions::Xlsx.as_str(),
            Extensions::Csv.as_str(),
            Extensions::Xls.as_str(),
            Extensions::Xlsm.as_str(),
        ];

        let mut entries = tokio::fs::read_dir(&self.input.folder_path).await?;

        while let Some(entry) = entries
            //
            .next_entry()
            .await
            .unwrap()
            .take()
        {
            let path = entry.path();

            if path.extension().map_or(false, |ext| {
                let ext = ext.to_string_lossy().to_lowercase();
                ext == se_arr[0] || ext == se_arr[1] || ext == se_arr[2] || ext == se_arr[3]
            }) && self.is_recent(&path).await.unwrap_or(false)
            {
                files.push(path)
            }
        }

        let print_files = files.clone();
        tokio::spawn(async move {
            Self::print_files(print_files).await.unwrap();
        })
        .await?;

        if files.len() >= self.input.required_min_files as usize {
            Ok(files.to_vec())
        } else {
            Err(Error::InvalidInput)
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
