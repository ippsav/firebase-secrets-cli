use indicatif::{ProgressBar, ProgressStyle, style::TemplateError};
use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, Write},
    process::{Command, Stdio},
    string::FromUtf8Error,
    time::Duration,
};
use thiserror::Error;
use tokio::task::JoinError;
use tokio::sync::mpsc;


type Result<T> = std::result::Result<T, FirebaseInterfaceError>;

pub struct FirebaseInterface {
    hash_map: HashMap<String, String>,
    alias: String,
}


impl FirebaseInterface {
    pub fn new(hash_map: HashMap<String, String>, alias: String) -> Self {
        Self { hash_map, alias }
    }

    fn set_project(&self) -> Result<()> {
        println!("Setting project alias to {}...", &self.alias);
        let child = Command::new("firebase")
            .arg("use")
            .arg(&self.alias)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| FirebaseInterfaceError::io_error("could not run firebase command, check if firebase-tools are installed in your machine", err))?;

        let stdout_output = child
            .wait_with_output()
            .map_err(|err| FirebaseInterfaceError::io_error("debug", err))?;

        let output = String::from_utf8(stdout_output.stdout)?;

        if output.contains("Error") {
            return Err(FirebaseInterfaceError::firebase_error(output));
        };
        println!("Project is set to {}", &self.alias);
        Ok(())
    }
    fn set_secret(key: &str, value: &str) -> Result<()> {
        let mut child = Command::new("firebase")
            .arg("functions:secrets:set")
            .arg(key)
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|err| FirebaseInterfaceError::io_error("could not run firebase command, check if firebase-tools are installed in your machine", err))?;

        write!(child.stdin.as_mut().unwrap(), "{value}")
            .map_err(|err| FirebaseInterfaceError::io_error("debug", err))?;


        let stdout_output = child
            .wait_with_output()
            .map_err(|err| FirebaseInterfaceError::io_error("debug", err))?;

        let output = String::from_utf8(stdout_output.stdout)?;

        if output.contains("Error") {
            return Err(FirebaseInterfaceError::secret_set_error(
                key.into(),
                value.into(),
                output,
            ));
        };
        Ok(())
    }
    pub async fn set_secrets(self) -> Result<()> {
        self.set_project()?;
        let bar = ProgressBar::new(1);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner} {msg}")?
        );
        bar.enable_steady_tick(Duration::from_millis(10));
        bar.set_message("Setting secrets...");

        //mspc channel approach
        let (tx, mut rx)  = mpsc::channel(100);

        self.hash_map
            .into_iter()
            .for_each(|(k,v)| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    tx.send(Self::set_secret(&k, &v)).await.unwrap();
                });
            });

        drop(tx);

        while let Some(res) = rx.recv().await {
            res?;
        };

        bar.finish_with_message("All secrets are set !");
        Ok(())
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
    RuntimeError(JoinError),
    IndicatifError(TemplateError),
}

impl From<FromUtf8Error> for FirebaseInterfaceError {
    fn from(value: FromUtf8Error) -> Self {
        Self::StringParse(value)
    }
}

impl From<JoinError> for FirebaseInterfaceError {
    fn from(value: JoinError) -> Self {
        Self::RuntimeError(value)
    }
}

impl From<TemplateError> for FirebaseInterfaceError {
    fn from(value: TemplateError) -> Self {
        Self::IndicatifError(value)
    }
}

impl FirebaseInterfaceError {
    pub fn io_error(msg: &'static str, source: io::Error) -> Self {
        Self::Io { msg, source }
    }
    pub fn firebase_error(msg: String) -> Self {
        Self::FirebaseError(msg)
    }
    pub fn secret_set_error(key: String, value: String, message: String) -> Self {
        Self::SecretSetError {
            key,
            value,
            message,
        }
    }
}

impl Display for FirebaseInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirebaseInterfaceError::Io { msg, source } => {
                write!(f, "{msg}, source: {source}")
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
                write!(f, "error parsing firebase output: {err}")
            }
            FirebaseInterfaceError::RuntimeError(err) => {
                write!(f, "tokio runtime error: {err}")
            },
            FirebaseInterfaceError::IndicatifError(err) => {
                write!(f, "indicatif error: {err}")
            },
        }
    }
}
