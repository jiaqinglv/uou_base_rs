use std::net::SocketAddr;

use axum::Router;

use super::Server;

pub struct AxumServer {
    // server: Option<Builder<hyper::AddrIncoming, Exec>>
    addr: Option<SocketAddr>,
}

impl Server for AxumServer {
    fn bind(addr: SocketAddr) -> Result<AxumServer, Box<dyn std::error::Error>> {
        Ok(AxumServer { addr: Some(addr) })
    }
}

impl AxumServer {
    #[allow(dead_code)]
    // 服务器监听运行
    pub async fn listen(self, router: Router) -> Result<(), Box<dyn std::error::Error>> {
        let server = match self.addr {
            None => panic!("Axum Server addr is none"),
            Some(server) => tokio::net::TcpListener::bind(server).await.unwrap(),
        };

        axum::serve(server, router.into_make_service()).await?;

        Ok(())
    }
}
