use std::fs;
// use tokio::fs;
use std::string::String;

use serde::{de::DeserializeOwned, Deserialize};
use tracing::log::error;

/// 配置文件类型
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub enum ConfigType {
    JSON(String),
    TOML(String),
    YAML(String),
}

/// 配置
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Config<T>
where
    T: ServerConfig,
{
    pub source: ConfigType,
    pub source_data: String,
    pub data: Option<T>,
}

/// 服务器配置文件-必须要有一个默认配置,方便错误时使用
pub trait ServerConfig: DeserializeOwned {
    fn default() -> Self
    where
        Self: Sized + Sync;
}

/// 配置信息
impl<T> Config<T>
where
    T: ServerConfig,
{
    pub fn new(source: ConfigType) -> Result<Config<T>, Box<dyn std::error::Error>>
    where
        T: ServerConfig + Sync + Send,
    {
        // 获取配置位置
        match &source {
            ConfigType::JSON(config_path) => {
                let config_file: Result<String, std::io::Error> =
                    fs::read_to_string(config_path.as_str());
                let config_file_data = match config_file {
                    Ok(source_data) => source_data,
                    Err(err) => {
                        error!("Couldn't read config file, {}", err);
                        return Err(Box::new(err));
                    }
                };
                let source_data = config_file_data;
                let mut conf: Config<T> = Config {
                    source,
                    data: Some(T::default()),
                    source_data,
                };

                let config = serde_json::from_str(&conf.source_data);
                conf.data = match config {
                    Ok(config) => config,
                    Err(err) => {
                        error!("configure load error:{}", err);
                        Some(T::default())
                    }
                };

                return Ok(conf);
            }
            ConfigType::TOML(_path) => {
                panic!("the config type is not supported");
            }
            ConfigType::YAML(config_path) => {
                let config_file: Result<String, std::io::Error> =
                    fs::read_to_string(config_path.as_str());
                let config_file_data = match config_file {
                    Ok(source_data) => source_data,
                    Err(err) => {
                        error!("Couldn't read config file, {}", err);
                        return Err(Box::new(err));
                    }
                };
                let source_data = config_file_data;
                let mut conf: Config<T> = Config {
                    source,
                    data: Some(T::default()),
                    source_data,
                };

                let config = serde_yaml::from_str(&conf.source_data);
                conf.data = match config {
                    Ok(config) => config,
                    Err(err) => {
                        error!("configure load error:{}", err);
                        Some(T::default())
                    }
                };

                return Ok(conf);
            }
        };
    }
}
