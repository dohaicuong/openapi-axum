use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub package: Package,
}

pub fn init() -> Result<Config, figment::Error> {
    Figment::new()
        .merge(Toml::file("Cargo.toml"))
        .merge(Env::prefixed("CARGO_"))
        .extract()
}
