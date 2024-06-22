use std::fmt::Debug;

use self::{app::App, data::DataSource, server::Servers};
use crate::core::config::{Config, ConfigType, ServerConfig};

pub mod app;
pub mod config;
pub mod data;
pub mod logs;
pub mod request;
pub mod response;
pub mod server;

/// 初始化axum http 服务器 并初始化配置、日志以及数据库信息
pub async fn load_config<C>(conf: ConfigType) -> Result<C, Box<dyn std::error::Error>>
where
    C: ServerConfig + Debug + Sync + Send,
{
    // 默认初始配置
    let default_conf = C::default();
    // 配置文件初始化
    let config_type = conf;
    // 获取程序初始配置
    let boot_confg: C = match Config::new(config_type) {
        Ok(config_data) => config_data.data.unwrap_or(default_conf),
        Err(err) => {
            println!("Server BootConfig Error: {}", err);
            default_conf
        }
    };

    Ok(boot_confg)
}

/// 创建应用
pub fn new_app<C, D, S, L>(
    conf: C,
    data: D,
    servers: S,
    logger: L,
    id: &'static str,
    name: &'static str,
    version: &'static str,
) -> App<C, D, S, L>
where
    C: ServerConfig + Sized + Send + Sync,
    D: DataSource + Send + Sync,
    S: Servers,
    L: logs::Logger + Send + Sync,
{
    return App {
        id: id.to_string(),
        name: name.to_string(),
        version: version.to_string(),
        conf,
        data: Some(data),
        servers: Some(servers),
        logger: Some(logger)
    };
}
