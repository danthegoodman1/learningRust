use snafu::{prelude::*, Location};

#[derive(Debug, Snafu)]
#[snafu(display("ID may not be less than 10, but it was {id}"))]
pub struct InvalidIdError {
    pub id: u16,
    #[snafu(implicit)]
    pub location: Location,
}

pub fn invalid_id(id: u16) -> Result<(), InvalidIdError> {
    ensure!(id > 10, InvalidIdSnafu { id });
    Ok(())
}


#[derive(Debug, Snafu)]
#[snafu(display("File error: {source}"))]
pub struct FileInvalidError {
    pub path: String,
    pub source: std::io::Error,
    #[snafu(implicit)]
    pub location: Location,
}

pub fn file_invalid(path: &str) -> Result<(), FileInvalidError> {
    let file = std::fs::File::open(path).context(FileInvalidSnafu { path })?;
    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
  #[snafu(display("Name is invalid: {name}\nLocation: {location}"))]
  InvalidName {
    name: String,
    #[snafu(implicit)]
    location: Location,
  },
}

impl Error {
    pub fn location(&self) -> &Location {
        match self {
            Error::InvalidName { location, .. } => location,
        }
    }
}

pub fn invalid_name(name: &str) -> Result<(), Error> {
    ensure!(name.len() > 3, InvalidNameSnafu { name: name.to_string() });
    Ok(())
}

pub fn generic_error() -> Result<(), Error> {
    ensure!(false, InvalidNameSnafu { name: "Dan".to_string() });
    Ok(())
}
