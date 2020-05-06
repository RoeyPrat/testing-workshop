#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::str;

#[derive(Debug, PartialEq)]
struct RespError<'a> {
    explanation: &'a str
}

fn parse_bulk(data: &[u8]) -> Result<&[u8], RespError> {
    let mut num_chars: usize = 0;
    let mut last_digit_index = 0;

    if (data[1] as char) == '-' && (data[2] as char) == '1' {
        return Ok(b"");
    }

    for (i, ch) in data[1..(data.len() - 1)].iter().enumerate() {
        if (*ch as char).is_digit(10) || (*ch as char) == '-' {
            num_chars *= 10;
            num_chars += (*ch as char).to_digit(10).unwrap() as usize;
            last_digit_index = i
        } else {
            break;
        }
    }
    if num_chars == 0 {
        return Ok(b"");
    }
    return Ok(&data[last_digit_index + 4..last_digit_index + 4 + num_chars]);
}

fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
    match data[0] {
        b'+' => {
            Ok(&data[1..data.len() - 2])
        }
        b'$' => {
            parse_bulk(data)
        }
        _ => Err(RespError { explanation: "not valid because of first char" })
    }
}

// Empty bulk string `b"$0\r\n\r\n"`
#[test]
fn test_empty_bulk_string() {
    assert_eq!(b"", resp_parse(b"$0\r\n\r\n").unwrap());
}

// Null Bulk Strings (`b"$-1\r\n"`)
#[test]
fn test_null_bulk_string() {
    assert_eq!(b"", resp_parse(b"$-1\r\n").unwrap());
}


// embedded with \r\n b"$12\r\rnhello\r\nworld\r\n"
#[test]
fn test_complex() {
    let expected = b"hello\r\nworld";
    let actual = resp_parse(b"$12\r\nhello\r\nworld\r\n").unwrap();

    assert_eq!(expected, actual, "{} != {}", str::from_utf8(expected).unwrap(), str::from_utf8(actual).unwrap());
}

#[quickcheck]
fn test_invalid_input(expected: String) -> bool {
    //
    let unparsed = format!("-{}\r\n", expected);
    Err(RespError { explanation: "not valid because of first char" }) == resp_parse(unparsed.as_bytes())
}

#[quickcheck]
fn simple_string(expected: String) -> bool {
    let unparsed = format!("+{}\r\n", expected);
    let actual = resp_parse(unparsed.as_bytes()).unwrap();
    expected == str::from_utf8(actual).unwrap()
}