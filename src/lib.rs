// mod cli;
// mod cli_qualifiers;
mod error;
mod file_handling;
mod filetypes;
mod parser;
mod prelude;

pub use tracing::{debug, error, info};
use tracing_subscriber::fmt::time::Uptime;

pub use self::{
    // cli::*,
    // cli_qualifiers::*,
    file_handling::{Mover, PreProcessor},
    parser::UserInput,
    prelude::{Error, Result, W},
};

pub type TracingSubscriber = tracing_subscriber::fmt::SubscriberBuilder<
    tracing_subscriber::fmt::format::DefaultFields,
    tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Full, Uptime>,
>;

pub fn init_logger() -> TracingSubscriber {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_max_level(tracing::Level::DEBUG)

    // .with_env_filter(tracing_subscriber::EnvFilter::INFO)
    // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
    // .with_timer(tracing_subscriber::fmt::time::SystemTime)
}
