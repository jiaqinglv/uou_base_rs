use super::{
    config::ServerConfig,
    data::DataSource,
    logs,
    server::Servers,
};

/// 服务器应用 C: 配置信息 S: 服务器 L: 日志
pub struct App<C, D, S, L>
where
    C: ServerConfig + Sized + Send + Sync,
    D: DataSource + Send + Sync,
    S: Servers + Send + Sync,
    L: logs::Logger + Send + Sync,
{
    // 应用ID
    pub id: String,
    // 名称
    pub name: String,
    // 版本
    pub version: String,
    // 自定义服务
    pub servers: Option<S>,
    // 配置文件
    pub conf: C,
    // 数据源
    pub data: Option<D>,

    pub logger: Option<L>
}

/// 应用信息
impl<C, D, S, L> App<C, D, S, L>
where
    C: ServerConfig + Sized + Sync + Send,
    D: DataSource + Send + Sync,
    S: Servers + Send + Sync,
    L: logs::Logger + Send + Sync,
{
    pub fn default_log(self, level: log::Level) -> Result<Self, Box<dyn std::error::Error>> {
        logs::new_default_log(level).expect("init default log error");
        return Ok(self);
    }

    /// 日志初始化
    pub fn init_log(self, level: log::Level) -> Result<Self, Box<dyn std::error::Error>> {
        logs::new_default_log(level).expect("init default log error");
        return Ok(self);
    }

    /// 日志初始化
    pub fn init_opentelemetry_log(self, level: log::Level) -> Result<Self, Box<dyn std::error::Error>> {
        logs::DefaultLogger::new_tracing_opentelemetry_jaeger(level, self.name.clone()).expect("init opentelemetry log error");
        return Ok(self);
    }
}
