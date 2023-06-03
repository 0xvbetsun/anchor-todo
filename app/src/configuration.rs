use serde::{Deserialize, Deserializer};

pub enum Storage {
    Solana,
    InMemory
}

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
    pub storage: Storage,
    pub keypair_file: String,
}

impl<'de> Deserialize<'de> for Storage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "solana" => Storage::Solana,
            "in-memory" => Storage::InMemory,
            &_ => todo!()
        })
    }
}
pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "config.toml",
            config::FileFormat::Toml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}