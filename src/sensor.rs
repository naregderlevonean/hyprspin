use anyhow::Result;
use zbus::{proxy, Connection};

#[proxy(
    interface = "net.hadess.SensorProxy",
    default_service = "net.hadess.SensorProxy",
    default_path = "/net/hadess/SensorProxy"
)]
pub trait SensorProxy {
    fn claim_accelerometer(&self) -> zbus::Result<()>;
    
    #[zbus(property)]
    fn accelerometer_orientation(&self) -> zbus::Result<String>;
}

pub async fn get_proxy() -> Result<SensorProxyProxy<'static>> {
    let connection = Connection::system().await?;
    let proxy = SensorProxyProxy::new(&connection).await?;
    
    proxy.claim_accelerometer().await?;
    Ok(proxy)
}
