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

// pub use self::prelude::{Error, Result, W};
// use parser::UserInput;
// use tracing::{debug, error, info};

use archive_me_plz::*;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        // .pretty()
        .init();

    let args = std::env::args().collect::<Vec<String>>();
    let user_input = UserInput::from_user_input(args)?;

    let start = tokio::time::Instant::now();
    if let Err(e) = tokio::join!(begin_processing(user_input)).0 {
        debug!("From within main, caught error: {e}");
        error!("Error: {e}");
    };

    let duration = start.elapsed();

    if tracing_subscriber::filter::LevelFilter::current() >= tracing::Level::INFO {
        info!("Time taken: {:?}", duration);
    } else {
        println!("{}", duration.as_secs_f64());
    }

    Ok(())
}

// #[tracing::instrument]
async fn begin_processing(input: UserInput) -> Result<()> {
    let processor = PreProcessor::new(&input);
    // info!("->> {:<12} - processor created", "BEGIN_PROCESSING");

    let mover = Mover::new(&input);
    // info!("->> {:<12} - mover created", "BEGIN_PROCESSING");

    let files = processor.process().await?;
    // info!("->> {:<12} - DIR PROCESSED", "LAST STAGE - MOVE");

    mover.par_move_files_copy(files.as_slice()).await?;
    println!("->> {:<12} - MOVE PROCESSED", "COMPLETED");

    Ok(())
}
