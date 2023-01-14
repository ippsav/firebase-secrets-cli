use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read};
use thiserror::Error;

use super::FirebaseInterface;

pub struct FirebaseInterfaceBuilder {
    hash_map: HashMap<String, String>,
}

impl FirebaseInterfaceBuilder {
    pub fn builder() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    pub fn from_source(&mut self, source: impl Read) -> Result<(), BuilderError> {
        let buf_reader = BufReader::new(source);

        for line in buf_reader.lines() {
            self.add_secret(line?)?;
        }

        Ok(())
    }

    pub fn add_secret(&mut self, secret: String) -> Result<(), BuilderError> {
        match secret.split_once('=') {
            Some((key, value)) => {
                self.hash_map.insert(key.into(), value.into());
            }
            None => {
                return Err(BuilderError::InvalidSecretFormat(secret.into()));
            }
        };
        Ok(())
    }

    pub fn build(self) -> FirebaseInterface {
        dbg!(&self.hash_map);
        FirebaseInterface::new(self.hash_map)
    }
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("error reading from source file")]
    Io(#[from] io::Error),
    #[error("secret `{0}` is in an invalid format")]
    InvalidSecretFormat(String),
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn add_valid_secret() {
        let mut builder = FirebaseInterfaceBuilder::builder();
        let secret = "KEY=VALUE".to_owned();

        assert!(builder.add_secret(secret).is_ok());

        let mut hash_map = HashMap::new();
        hash_map.insert("KEY".to_owned(), "VALUE".to_owned());

        assert_eq!(hash_map, builder.hash_map)
    }

    #[test]
    fn add_invalid_secret() {
        let mut builder = FirebaseInterfaceBuilder::builder();
        let secret = "INVALID_FORMAT".to_owned();
        let res = builder.add_secret(secret.clone());
        assert!(res.is_err());

        let error_msg = format!("secret `{secret}` is in an invalid format");
        let error = res.unwrap_err();

        assert_eq!(error.to_string(), error_msg);
    }

    #[test]
    fn add_secrets_from_source() {
        let mut builder = FirebaseInterfaceBuilder::builder();
        let source = r#"KEY1=VALUE1
KEY2=VALUE2
KEY3=VALUE=3="#
            .as_bytes();

        assert!(builder.from_source(source).is_ok());

        let mut hash_map = HashMap::new();
        hash_map.insert("KEY1".into(), "VALUE1".into());
        hash_map.insert("KEY2".into(), "VALUE2".into());
        hash_map.insert("KEY3".into(), "VALUE=3=".into());

        assert_eq!(hash_map, builder.hash_map);
    }
}
