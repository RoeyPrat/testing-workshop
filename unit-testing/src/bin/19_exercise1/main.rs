use std::error::Error;

fn resp_parse(data: &[u8]) -> Result<&[u8], Box<dyn Error>> {
    match data {
        [b'+', string.., b'\r', b'\n'] => Ok(string),
        //[b'$', ..] => Ok(&data[1..]),
        _ => Err("invalid data".into()),
    }
}

#[test]
fn test_resp_parse() {
    assert_eq!(resp_parse(b"+hello\r\n").unwrap(), b"hello");
}