use bytes::Buf;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
pub async fn main() -> Result<()> {

    let mut buf = &b"hello world"[..];

    assert_eq!(b'h', buf.get_u8());
    assert_eq!(b'e', buf.get_u8());
    assert_eq!(b'l', buf.get_u8());

    let mut rest = [0; 8];
    buf.copy_to_slice(&mut rest);

    assert_eq!(&rest[..], &b"lo world"[..]);

    println!("ok!");

    Ok(())

}
