use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomErrorA {
    #[error("Error A occurred: {0}")]
    ErrorA(String),
}

#[derive(Error, Debug)]
pub enum CustomErrorB {
    #[error("Error B occurred: {0}")]
    ErrorB(String),
}

#[derive(Error, Debug)]
pub enum CustomErrorC {
    #[error("Error C occurred: {0}")]
    ErrorC(String),
}


pub fn operation_c() -> Result<()> {
    Err(CustomErrorC::ErrorC("Something went wrong in C".to_string()).into())
}

pub fn operation_b() -> Result<()> {
    operation_c().context(CustomErrorB::ErrorB("Error in B".to_string())).context(format!("some more info on b on {}:{}", file!(), line!()))
}

pub fn operation_a() -> Result<()> {
    operation_b().context(CustomErrorA::ErrorA("Error in A".to_string())).context(format!("error calling operation_b {}:{}", file!(), line!()))?; // can use ?
    Ok(())
}

pub fn operation_root() -> Result<()> {
    operation_a().context(format!("failed to call operation a {}:{}", file!(), line!()))?;
    Ok(())
}
