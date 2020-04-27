use ::byte_strings::concat_bytes;

#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[derive(Debug, PartialEq)]
struct RespError {}

fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
    match data[0] {
        b'+' => {
            Ok(&data[1..data.len() - 2])
        }
        b'$' => {
            Ok(data)
        }
        _ => Err(RespError {})
    }
}

#[test]
fn test_invalid_input() {
    assert_eq!(Err(RespError {}), resp_parse(b"-not valid because of first char\r\n"));
}

#[test]
fn test_simple_string() {
    let expected: &[u8] = b"so simple";
    let unparsed = concat_bytes!(b"+", expected, b"\r\n");
    let actual = resp_parse(unparsed).unwrap();
    assert_eq!(expected.len(), actual.len(), "Arrays don't have the same length");
    assert!(expected.iter().zip(actual.iter()).all(|(a, b)| a == b), "Arrays are not equal");
}