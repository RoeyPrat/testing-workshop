
#[derive(Debug, PartialEq)]
struct RespError{
}

fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
    if data[0] !=  b'+' && data[0] !=  b'$'{
        return Err(RespError{})
    }
    return Ok(data)
}

#[test]
fn test_invalid_input() {
    assert_eq!(resp_parse(b"-not valid\r\n"), Err(RespError{}));
}