use tracing_core::LevelFilter;

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct TracingConfig {
    #[serde(deserialize_with = "deserialize_level_filter")]
    pub level: LevelFilter,
    pub tempo_url: String,
    pub tempo_port: u32,
    pub service_name: String,
}

pub fn deserialize_level_filter<'de, D>(deserializer: D) -> Result<LevelFilter, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct StringLevelFilter;

    impl<'de> serde::de::Visitor<'de> for StringLevelFilter {
        type Value = LevelFilter;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("string")
        }

        fn visit_str<E>(self, value: &str) -> Result<LevelFilter, E>
        where
            E: serde::de::Error,
        {
            match value {
                "OFF" => Ok(LevelFilter::OFF),
                "ERROR" => Ok(LevelFilter::ERROR),
                "WARN" => Ok(LevelFilter::WARN),
                "INFO" => Ok(LevelFilter::INFO),
                "DEBUG" => Ok(LevelFilter::DEBUG),
                "TRACE" => Ok(LevelFilter::TRACE),
                _ => Err(serde::de::Error::custom(format!("unknown log level: {value}"))),
            }
        }
    }

    deserializer.deserialize_any(StringLevelFilter)
}
