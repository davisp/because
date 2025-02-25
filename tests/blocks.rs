use because::Because;

/// The main BlocksError for a hypothetical Blocks.txt parsing crate
///
/// This would be in something like src/lib.rs that encapsulates all of the
/// top level error types returned by the library.
///
/// Note that each variant is marked transparent as the first line of an error
/// report being "An error in blocks.rs occurred!" isn't super useful.
#[derive(Debug, thiserror::Error)]
pub enum BlocksError {
    #[error(transparent)]
    Download(#[from] DownloadError),

    #[error(transparent)]
    FromFile(#[from] FromFileError),
}

/// An error encountered while parsing a Blocks.rs that was downloaded
///
/// We have errors issuing the request, reading the response, and parsing
/// the response.
#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Request error while downloading Blocks.txt")]
    Request(#[source] std::io::Error),

    #[error("Error reading resposne body while downlaoding Blocks.txt")]
    ReadBody(#[source] std::io::Error),

    #[error("Error parsing Blocks.txt")]
    Parse(#[from] ParseError),
}

/// An error encountered while parsing a random file on disk
///
/// We have errors reading the file (i.e., it doesn't exist) and also the
/// same ParseError when parsing the loaded data.
#[derive(Debug, thiserror::Error)]
pub enum FromFileError {
    #[error("Error reading Blocks.txt from {0}")]
    ReadFile(std::path::PathBuf, #[source] std::io::Error),

    #[error("Error parsing Blocks.txt from {0}")]
    Parse(std::path::PathBuf, #[source] ParseError),
}

/// An error encounterd while parsing
///
/// This represents that a parse error happened at some location in Blocks.txt
/// and carries the type of parse error encountered.
#[derive(Debug, thiserror::Error)]
#[error("Invalid data on line {line}")]
pub struct ParseError {
    line: usize,
    #[source]
    kind: ParseErrorKind,
}

/// The kind of ParseError that was encountered.
#[derive(Debug, thiserror::Error)]
pub enum ParseErrorKind {
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
        ParseError {
            line: 5,
            kind: ParseErrorKind::ParseInt(root),
        },
    ));

    let mesg = format!("{}\n{}", err, err.because().unwrap());
    let expect = r#"Error parsing Blocks.txt from path/to/Blocks.txt
    1: Invalid data on line 5
    2: one end of range is not a valid hexadecimal integer
    3: invalid digit found in string
"#;

    assert_eq!(mesg, expect);
}
