use clap::Parser;
use dotenv;
use std::env;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

/// Q&A web service API
#[derive(Deserialize, Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Config{
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "error")]
    pub log_level: String,
    /// Which PORT the server is listening to
    #[clap(short, long, default_value = "8080")]
    pub port: u16,
    /// Database user
    #[clap(long, default_value = "postgres")]
    pub db_user: String,
    /// Database user
    #[clap(long, default_value = "123456")]
    pub db_password: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub db_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub db_port: u16,
    /// Database name
    #[clap(long, default_value = "jobdb")]
    pub db_name: String,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        // //test
        // let config_path = "config.dev.yaml".to_string();
        // let mut file = File::open(config_path)?;
        // let mut contents = String::new();
        // file.read_to_string(&mut contents);
        // let config: Config = serde_yaml::from_str(&contents)?;


        dotenv::dotenv().ok();
        let config = Config::parse();

        let port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.port))
            .map_err(|e| handle_errors::Error::ParseError(e))?;

        let db_user =
            env::var("POSTGRES_USER").unwrap_or(config.db_user.to_owned());
        let db_password = env::var("POSTGRES_PASSWORD").unwrap();
        let db_host =
            env::var("POSTGRES_HOST").unwrap_or(config.db_host.to_owned());
        let db_port = env::var("POSTGRES_PORT")
            .unwrap_or(config.db_port.to_string());
        let db_name =
            env::var("POSTGRES_DB").unwrap_or(config.db_name.to_owned());

        Ok(Config {
            log_level: config.log_level,
            port,
            db_user,
            db_password,
            db_host,
            db_port: db_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            db_name,
        })
    }
}