use anyhow::{anyhow, Context, Result};
use std::io::{self, ErrorKind};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OtherError {
    #[error("random other error")]
    RandomOther(#[from] io::Error)
}

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("my custom other")]
    CustomOther(String)
}

impl CustomError {
    fn reason(&self) -> &str {
        match self {
            CustomError::CustomOther(reason) => {
                reason
            }
        }
    }
}

// pub fn bottom_2() -> Result<()> {
//     Err(anyhow!(OtherError::RandomOther(io::Error::new(
//         io::ErrorKind::NotFound,
//         "bottom wasn't found!"
//     )))
//     .context("bottom 2 context"))
// }

// pub fn top_2() -> Result<()> {
//     return match bottom_2() {
//         Ok(_) => Ok(()),
//         Err(err) => Err(anyhow!(err)).context("top 2 context"),
//     };
// }

pub fn bottom_level() -> Result<()> {
    Err(anyhow!(ErrorKind::NotFound).context("bottom level context")) // can put inside
}

pub fn top_level() -> Result<()> {
    return match bottom_level() {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(err)).context("top level context"), // or outside
    };
}

pub fn custom_bottom() -> Result<()> {
    Err(anyhow!(CustomError::CustomOther(String::from("iam a hte reson")))).context("bottom ctx")
}

pub fn custom_top() -> Result<()> {
    Ok(match custom_bottom() {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(err)).context("custom top context")
    }?)
}
