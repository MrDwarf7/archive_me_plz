use archive_me_plz::{init_logger, *};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    init_logger().init();

    // let cli = Cli::default();
    // info!("->> {:<12} - CLI created", "MAIN");
    // debug!("{:#?}", cli);
    // todo!();

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
