pub enum Error {
    FileNotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileNotFound(file_name) => endium_error("File {file_name} not found."),
        }

        Ok(())
    }
}

fn endium_error(error: &str) {
    panic!("{:#?}", error);
}
