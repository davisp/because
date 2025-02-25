use because::Because;

#[derive(Debug, thiserror::Error)]
pub enum BlocksError {
    #[error(transparent)]
    Download(#[from] DownloadError),

    #[error(transparent)]
    FromFile(#[from] FromFileError),
}

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Request error while downloading Blocks.txt")]
    Request(#[source] std::io::Error),

    #[error("Error reading resposne body while downlaoding Blocks.txt")]
    ReadBody(#[source] std::io::Error),

    #[error("Error parsing Blocks.txt")]
    Parse(#[from] ParserError),
}

#[derive(Debug, thiserror::Error)]
pub enum FromFileError {
    #[error("Error reading Blocks.txt from {0}")]
    ReadFile(std::path::PathBuf, #[source] std::io::Error),

    #[error("Error parsing Blocks.txt from {0}")]
    Parse(std::path::PathBuf, #[source] ParserError),
}

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Invalid data on line {0}")]
    InvalidData(usize, #[source] ParseErrorReason),
}
#[derive(Debug, thiserror::Error)]
pub enum ParseErrorReason {
    #[error("missing semicolon")]
    NoSemicolon,

    #[error("no `..` in range")]
    NoDotDot(usize),

    #[error("one end of range is not a valid hexadecimal integer")]
    ParseInt(#[source] std::num::ParseIntError),
}

#[test]
fn some_test() {
    let root = "foo".parse::<u32>().unwrap_err();
    let err = BlocksError::FromFile(FromFileError::Parse(
        std::path::PathBuf::from("path/to/Blocks.txt"),
        ParserError::InvalidData(5, ParseErrorReason::ParseInt(root)),
    ));

    let mesg = format!("{}\n{}", err, err.because().unwrap());
    let expect = r#"Error parsing Blocks.txt from path/to/Blocks.txt
    1: Invalid data on line 5
    2: one end of range is not a valid hexadecimal integer
    3: invalid digit found in string
"#;

    assert_eq!(mesg, expect);
}
