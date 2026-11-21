use std::{env, path::PathBuf, sync::LazyLock};
use dotenv::dotenv;
use serde::{Deserialize,Serialize};


#[derive(Deserialize,Debug, Serialize)]
pub(crate) struct Config {
    pub(crate) port: u16,
    pub(crate) database_file: PathBuf, // stores all in here using json
}
pub(crate) static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    dotenv().ok();

    Config{
    port: env::var("PORT").expect("PORT must be set for the initialization to happen.").parse().expect("Port must be a number"),
    database_file: env::var("DB_FILE")
        .expect("DB_FILE must be set for the initialization to happen.")
        .into()
    }
});
