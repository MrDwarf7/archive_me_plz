use crate::{prelude::HELP_TEXT, Error, Result};
use chrono::NaiveDate;
use std::{path::PathBuf, str::FromStr};
use tracing::{debug, warn};

pub enum ArgumentIn {
    Amount,
    Oldest,
    FolderPath,
}

impl FromStr for ArgumentIn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "amount" => Ok(ArgumentIn::Amount),
            "oldest" => Ok(ArgumentIn::Oldest),
            "folder_path" => Ok(ArgumentIn::FolderPath),
            _ => {
                println!("Invalid input found - in FromStr, ArgumentIn");
                Err(Error::InvalidInput)
            }
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct UserInput {
    pub required_min_files: usize,
    pub oldest_to_keep: NaiveDate,
    pub folder_path: PathBuf,
}

impl UserInput {
    // #[tracing::instrument("debug")]
    pub fn from_user_input(mut args: Vec<String>) -> Result<Self> {
        let mut user_input = UserInput::default();
        args.remove(0);

        dbg!(&args);

        if args.is_empty() {
            warn!("No arguments provided");
            UserInput::help_and_exit()
        }

        let de_dashed_args = args
            .iter()
            .map(|arg| {
                if arg.starts_with('-') {
                    arg.trim_start_matches('-').to_string()
                } else if arg.starts_with("--") {
                    arg.trim_start_matches("--").to_string()
                } else {
                    arg.to_string()
                }
            })
            .collect::<Vec<String>>();

        let _ = match de_dashed_args.len() {
            1 => {
                if de_dashed_args[0] == "help" {
                    warn!("Help requested");
                    UserInput::help_and_exit()
                } else {
                    return Err(Error::OnlyProvidedOneArgument);
                }
            }
            2 => return Err(Error::OnlyProvideTwoArguments),
            3 => de_dashed_args.iter().map(|arg| {
                user_input.by_index(
                    de_dashed_args.iter().position(|x| x == arg).unwrap(),
                    &args[de_dashed_args.iter().position(|x| x == arg).unwrap()],
                );
            }),
            _ => return Err(Error::TooManyArguments),
        }
        .collect::<Vec<_>>();

        // dbg!(&user_input);

        Ok(user_input)
    }

    fn by_index(&mut self, index: usize, value: &str) {
        debug!("Index: {:?}, Value: {:?}", index, value);
        match index {
            0 => self.required_min_files = value.parse().expect("Failed to parse int"),
            1 => self.oldest_to_keep = NaiveDate::from_str(value).expect("Failed to parse date"),
            2 => self.folder_path = PathBuf::from(value),
            _ => (),
        }
    }

    fn help_and_exit() -> ! {
        println!("{HELP_TEXT}");
        std::process::exit(0);
    }
}
