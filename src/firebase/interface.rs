use std::{
    collections::HashMap,
    fmt::Display,
    io::{self},
    process::{Command, Stdio},
    string::FromUtf8Error,
};
use rayon::prelude::*;
use thiserror::Error;

pub struct FirebaseInterface {
    hash_map: HashMap<String, String>,
}

impl FirebaseInterface {
    pub fn new(hash_map: HashMap<String, String>) -> Self {
        Self { hash_map }
    }

    fn set_project(alias: String) -> Result<(), FirebaseInterfaceError> {
        println!("Setting project alias to {}...", &alias);
        let child = Command::new("firebase")
            .arg("use")
            .arg(&alias)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| FirebaseInterfaceError::io_error("could not run firebase command, check if firebase-tools are installed in your machine", err) )?;

        let stdout_output = child
            .wait_with_output()
            .map_err(|err| FirebaseInterfaceError::io_error("debug", err))?;

        let output = String::from_utf8(stdout_output.stdout)?;

        if output.contains("Error") {
            return Err(FirebaseInterfaceError::firebase_error(output));
        };
        println!("Project is set to {}", &alias);
        Ok(())
    }
    pub fn set_secrets(&self, alias: String) -> Result<(), FirebaseInterfaceError> {
        Self::set_project(alias)?;
        todo!()    
    }
}

#[derive(Error, Debug)]
pub enum FirebaseInterfaceError {
    Io {
        msg: &'static str,
        #[source]
        source: io::Error,
    },
    SecretSetError {
        key: String,
        value: String,
        message: String,
    },
    FirebaseError(String),
    StringParse(FromUtf8Error),
}

impl From<FromUtf8Error> for FirebaseInterfaceError {
    fn from(value: FromUtf8Error) -> Self {
        Self::StringParse(value)
    }
}

impl FirebaseInterfaceError {
    pub fn io_error(msg: &'static str, source: io::Error) -> Self {
        Self::Io { msg, source }
    }
    pub fn firebase_error(msg: String) -> Self {
        Self::FirebaseError(msg)
    }
}

impl Display for FirebaseInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirebaseInterfaceError::Io { msg, source } => {
                write!(f, "{}, source: {:?}", msg, source)
            }
            FirebaseInterfaceError::SecretSetError {
                key,
                value,
                message,
            } => write!(f, "couldn't set {key} = {value}, {message}"),
            FirebaseInterfaceError::FirebaseError(err) => {
                write!(f, "{err}")
            }
            FirebaseInterfaceError::StringParse(err) => {
                write!(f, "error parsing firebase output: {:?}", err)
            }
        }
    }
}
