use std::ffi::OsStr;

#[derive(Debug, PartialEq)]
pub enum Extensions {
    Xlsx,
    Csv,
    Xls,
    Xlsm,
    Txt,
    Tmp,
    Eml,
}

impl Extensions {
    pub fn generate_set() -> Vec<Self> {
        vec![
            Extensions::Xlsx,
            Extensions::Csv,
            Extensions::Xls,
            Extensions::Xlsm,
            Extensions::Txt,
            Extensions::Tmp,
            Extensions::Eml,
        ]
    }

    pub fn from_extension(ext: &OsStr) -> Option<Self> {
        let ext = ext.to_string_lossy().to_lowercase();

        match ext.as_str() {
            "xlsx" => Some(Extensions::Xlsx),
            "csv" => Some(Extensions::Csv),
            "xls" => Some(Extensions::Xls),
            "xlsm" => Some(Extensions::Xlsm),
            "txt" => Some(Extensions::Txt),
            "tmp" => Some(Extensions::Tmp),
            "eml" => Some(Extensions::Eml),
            _ => None,
        }
    }
}
