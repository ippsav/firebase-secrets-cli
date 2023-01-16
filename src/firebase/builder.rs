use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read};
use thiserror::Error;

use super::FirebaseInterface;

pub struct FirebaseInterfaceBuilder {
    hash_map: HashMap<String, String>,
    alias: Option<String>,
}

impl FirebaseInterfaceBuilder {
    pub fn builder() -> Self {
        Self {
            hash_map: HashMap::new(),
            alias: None,
        }
    }

    pub fn from_source(&mut self, source: impl Read) -> Result<(), BuilderError> {
        let buf_reader = BufReader::new(source);

        for line in buf_reader.lines() {
            let value = line?;
            if value.is_empty() || value.trim_start().starts_with('#') {
                continue;
            };
            self.add_secret(value)?;
        }

        Ok(())
    }

    pub fn set_alias(&mut self, alias: String) {
        self.alias = Some(alias);
    }

    pub fn add_secret(&mut self, secret: String) -> Result<(), BuilderError> {
        match secret.split_once('=') {
            Some((key, value)) => {
                let raw_value = match value.split_once('#') {
                    Some((val, _)) => val.trim(),
                    None => value.trim(),
                };
                if raw_value.starts_with('"') && raw_value.ends_with('"') {
                    self.hash_map
                        .insert(key.into(), raw_value[1..raw_value.len() - 1].into());
                } else {
                    self.hash_map.insert(key.into(), raw_value.into());
                };
            }
            None => {
                return Err(BuilderError::InvalidSecretFormat(secret.into()));
            }
        };
        Ok(())
    }

    pub fn build(self) -> Result<FirebaseInterface, BuilderError> {
        let alias = match self.alias {
            Some(v) => v,
            None => {
                return Err(BuilderError::ProjectAliasNotSetError);
            }
        };
        Ok(FirebaseInterface::new(self.hash_map, alias))
    }
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("error reading from source file")]
    Io(#[from] io::Error),
    #[error("secret `{0}` is in an invalid format")]
    InvalidSecretFormat(String),
    #[error("error project alias not set")]
    ProjectAliasNotSetError,
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
KEY3=VALUE=3=
"#
        .as_bytes();

        assert!(builder.from_source(source).is_ok());

        let mut hash_map = HashMap::new();
        hash_map.insert("KEY1".into(), "VALUE1".into());
        hash_map.insert("KEY2".into(), "VALUE2".into());
        hash_map.insert("KEY3".into(), "VALUE=3=".into());

        assert_eq!(hash_map, builder.hash_map);
    }

    #[test]
    fn add_secrets_from_source_export_from_firebase() {
        let mut builder = FirebaseInterfaceBuilder::builder();
        let source = r#"KEY1="VALUE1"
KEY2=VALUE2
#### COMMENT
KEY3="VAL3"                 # comment
"#
        .as_bytes();

        assert!(builder.from_source(source).is_ok());

        let mut hash_map = HashMap::new();
        hash_map.insert("KEY1".into(), "VALUE1".into());
        hash_map.insert("KEY2".into(), "VALUE2".into());
        hash_map.insert("KEY3".into(), "VAL3".into());

        assert_eq!(hash_map, builder.hash_map);
    }
}
