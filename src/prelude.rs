// in-crate Error type
pub use crate::error::Error;

// in-crate result type
pub type Result<T> = std::result::Result<T, Error>;

// Wrapper struct
pub struct W<T>(pub T);

pub const ARCHIVE_FOLDER_NAME: &str = "_archive";

pub const SUPPORTED_EXTENSIONS: [&str; 4] = ["xlsx", "csv", "xls", "xlsm"];

pub const HELP_TEXT: &str = r#"
Usage: archive_me_plz [qualifier oldest_to_keep folder_path] -> [output]

    info:
    qualifier:          The minimum qualifier of files required to qualify a folder for archiving
    oldest_to_keep:     A date (YYYY-MM-DD), from today backwards to the date given. 
                        Files within that range won't be archived.
                    
    folder_path:        The path to the folder to be archived
    output:             The path to the output folder, default is 'folder_path/_archive'

    help:               Display this help text"
    -------------------------
    Example: (Current date: 2024-02-01)
    
    archive_me_plz 5 2024-01-01 /home/user/folder_with_lots_of_files
    
    -------------------------
    In the Example above, files from 2024-01-01 to the earliest date available
    will be archived. 
    Leaving any files from 2024-01-01 to current (2024-02-01) in the folder.
    The folder /home/user/folder_with_lots_of_files will be
    archived if it contains 5 or more files.

"#;
