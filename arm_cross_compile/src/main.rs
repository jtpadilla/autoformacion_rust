use arm_cross_compile::qr_reader::QrReader;
use arm_cross_compile::tcp_server::TcpServer;

#[tokio::main]
async fn main() -> tokio_serial::Result<()> {

    env_logger::init();

    let mut _reader = QrReader::new()?;

    let _tcp_server = TcpServer::new(_reader);

    Ok(())

}
