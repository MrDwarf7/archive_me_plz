use crate::prelude::ARCHIVE_FOLDER_NAME;
use crate::{Result, UserInput};

use std::path::PathBuf;
use tokio::fs::create_dir_all;

#[derive(Clone)]
pub struct Mover<'a> {
    input: &'a UserInput,
}

impl<'a> Mover<'a> {
    pub fn new(input: &'a UserInput) -> Self {
        Mover { input }
    }
}

// #[async_trait]
impl<'a> Mover<'a> {
    pub async fn move_files(&self, files: &[PathBuf]) -> Result<()> {
        let file_len = files.len();
        let (tx, mut rx) = tokio::sync::mpsc::channel(file_len);

        let archive = self.input.folder_path.clone().join(ARCHIVE_FOLDER_NAME);

        dbg!(archive.clone());

        create_dir_all(&archive).await?;

        // let mut handles: Vec<JoinHandle<std::prelude::v1::Result<(), _>>> = Vec::new();

        (0..file_len).for_each(|file| {
            let file = files[file].clone();
            let archive = archive.clone();

            let tx = tx.clone();
            println!("1 file: {:?}", file);

            tokio::spawn(async move {
                let file_name = file.file_name().unwrap();
                let archive = archive.join(file_name);

                println!("1.5 file: {:?}", file);

                let rename_fut = tokio::join!(tokio::fs::rename(file, archive)).0;
                // let rename_fut = tokio::fs::rename(file, archive).into_future();

                let _ = tokio::spawn(async move {
                    let _ = tx.send(rename_fut).await;
                    println!("2 length: {:?}", file_len);
                    drop(tx)
                })
                .await;
            });

            // let _ = tx.send(handle).await;
            // drop(tx);
            // handles.push(handle);
        });

        tokio::spawn(async move {
            for _i in 0..file_len {
                println!("3 Waiting for file to be moved");
                println!("4 Files left: {:?}", rx.len());
                println!("5 iter val: {:?}", _i);
                while let Ok(h) = rx.try_recv() {
                    h.unwrap();
                }
            }
            rx.close();
        })
        .await?;

        // for h in handles {
        //     h.await??;
        // }

        Ok(())
    }
}
