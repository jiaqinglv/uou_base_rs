use std::{error::Error, net::SocketAddr};

pub mod axum;

// Server
pub trait Server
where
    Self: Sized + Send + Sync,
{
    fn bind(addr: SocketAddr) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

// Server集合
pub trait Servers
where
    Self: Sized + Send + Sync + Send,
{
}

#[allow(dead_code)]
// 服务器地址
pub enum ServerAddress {
    /// SocketAddr 类型
    SocketAddr(SocketAddr),
    // String 转 SocketAddr
    StringToSocketAddr(String),
}
