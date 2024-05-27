// TODO:
// Modularize via interface intergration

// Add a walkdir implementation, so it can be used in the processor
// should then be able to put the tool in the root, and with the "Qualifier" flag,
// it should be able to run the tool on the root, and all subdirectories
// and only moving items etc. when there are X amount of files in the folder

// Create printer/logger for the tool

// Add tests

// Additions: ----------
// ie: a folder must contain min. 800 files to even be considered > Okay we have our list of things,
// create a spawned thread for each (via tokio::spawn() as we don't care when each finishes and
// need to run through all of them regardless of order (Maybe rayon??)
//
// then for each spawned thread(ie: each dir that's valid) - go through and spawn N tasks to get metadata,
// then moved the file
// (Where N is the number of files in the folder)
// Potential cons
//
// We could end up with severe backpressure, or the system could run out of resources?

mod error;
mod file_handling;
mod parser;
mod prelude;

pub use self::prelude::{Error, Result, W};

use parser::UserInput;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let user_input = UserInput::from_user_input(args)?;

    let start = tokio::time::Instant::now();
    if let Err(e) = tokio::join!(begin_processing(user_input)).0 {
        eprintln!("Error: {e}");
    };

    let duration = start.elapsed();
    println!("->> {:<12} - TOTAL TIME", duration.as_secs_f64());

    // if let Err(e) = begin_processing(user_input).await {
    //     eprintln!("Error: {e}");
    // }

    Ok(())
}

async fn begin_processing(input: UserInput) -> Result<()> {
    let processor = file_handling::Processor::new(&input);
    println!("->> {:<12} -  processor created", "BEGIN_PROCESSING");

    let mover = file_handling::Mover::new(&input);
    println!("->> {:<12} - mover created", "BEGIN_PROCESSING");

    let files = processor.process().await?;
    println!("->> {:<12} - DIR PROCESSED", "LAST STAGE - MOVE");

    mover.par_move_files_copy(files.as_slice()).await?;
    println!("->> {:<12} - MOVE PROCESSED", "COMPLETED");

    Ok(())
}
