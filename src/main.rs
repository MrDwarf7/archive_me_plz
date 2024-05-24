// TODO:
// Modularize via interface intergration

// Add a walkdir implementation, so it can be used in the processor
// should then be able to put the tool in the root, and with the "Qualifier" flag,
// it should be able to run the tool on the root, and all subdirectories
// and only moving items etc. when there are X amount of files in the folder

// Create printer/logger for the tool

// Add tests

mod error;
mod file_handling;
mod parser;
mod prelude;

use std::path::PathBuf;

pub use self::prelude::{Error, Result, W};

use parser::UserInput;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let user_input = UserInput::from_user_input(args)?;

    if let Err(e) = begin_processing(user_input).await {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn begin_processing(input: UserInput) -> Result<()> {
    let processor = Box::new(file_handling::Processor::new(&input));
    println!("->> {:<12} -  processor created", "BEGIN_PROCESSING");

    let mover = Box::new(file_handling::Mover::new(&input));
    println!("->> {:<12} - mover created", "BEGIN_PROCESSING");

    let files = processor.process_dir().await?;
    println!("->> {:<12} - DIR PROCESSED", "LAST STAGE - MOVE");

    mover
        .move_files(&files.iter().map(PathBuf::from).collect::<Vec<PathBuf>>())
        .await?;

    Ok(())
}
