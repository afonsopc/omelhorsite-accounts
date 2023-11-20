use envconfig::Envconfig;
use lazy_static::lazy_static;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "SERVER_HOST")]
    pub server_host: String,

    #[envconfig(from = "PICTURES_DIRECTORY")]
    pub pictures_directory: String,

    #[envconfig(from = "DEFAULT_PICTURE")]
    pub default_picture: String,

    #[envconfig(from = "HANDLE_MAX_LENGTH")]
    pub handle_max_length: usize,

    #[envconfig(from = "NAME_MAX_LENGTH")]
    pub name_max_length: usize,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::init_from_env().unwrap();
}
