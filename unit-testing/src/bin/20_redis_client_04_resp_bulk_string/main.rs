use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn resp_parse(reader: impl Read) -> Result<String> {
    let mut reader = BufReader::new(reader);

    let mut resp_type = [0; 1];
    reader.read_exact(&mut resp_type)?;

    match resp_type[0] {
        b'+' => {
            let mut data = String::new();
            reader.read_line(&mut data)?;
            let data = data.trim_end().to_string();
            Ok(data)
        }
        b'$' => {
            let mut len_buf = String::new();
            reader.read_line(&mut len_buf)?;
            let data_length = len_buf.trim().parse()?;

            let mut data = vec![0; data_length];
            reader.read_exact(data.as_mut_slice())?;
            reader.read_exact(&mut [0; 2])?; // Throw away terminating "\r\n"

            Ok(format!("{}", String::from_utf8(data)?))
        }
        _ => Err(format!("Illegal RESP: {}", resp_type[0] as char).into()),
    }
}

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    stream.write(b"INFO\r\n")?;

    let reply = resp_parse(stream)?;

    print!("{}", reply);

    Ok(())
}
