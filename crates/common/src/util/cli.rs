pub use clap::Parser;

use serde::{
    de::{Deserializer, Error},
    Deserialize,
};

pub fn from_file_or_const<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if let Some(s) = s.strip_prefix("file:") {
        let content = std::fs::read_to_string(s).map_err(Error::custom)?;
        content.parse().map_err(Error::custom)
    } else {
        s.parse().map_err(Error::custom)
    }
}

#[derive(Parser, Debug)]
pub struct Args<T: serde::de::DeserializeOwned + Clone + Send + Sync + 'static> {
    #[arg(short, long, env, value_parser = toml_from_file::<T>)]
    pub config: T,
}

fn toml_from_file<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    // toml::from_str(&std::fs::read_to_string(path).expect("Something went wrong reading the file"))
    //     .expect("failed to parse config file")

    // The above does not work as we can not deserialize enums from toml when they are not the only
    //  table in the file without first converting to JSON as a workaround.
    //  Example Error: thread 'main' panicked at 'failed to parse config file: Error { inner: ErrorInner { kind: Wanted { expected: "exactly 1 table", found: "more than 1 table" }, line: None, col: 0, at: Some(0), message: "", key: [] } }',
    let toml = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config_json = toml::from_slice::<serde_json::Value>(toml.as_bytes()).map_err(|e| e.to_string())?;
    let config = serde_json::from_value(config_json).map_err(|e| e.to_string())?;
    Ok(config)
}

impl<T> Args<T>
where
    T: serde::de::DeserializeOwned + Clone + Send + Sync + 'static,
{
    pub fn new(config: T) -> Self {
        Self { config }
    }
}
