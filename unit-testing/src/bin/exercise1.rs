#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::str;

#[derive(Debug, PartialEq)]
struct RespError <'a>{
    explanation: &'a str
}

fn parse_bulk(data: &[u8]) -> Result<&[u8], RespError> {
    let mut num_chars: u64 = 0;
    for ch in data[1..(data.len() - 1)] {
        if ch.is_digit(10) {
            num_chars *= 10;
            num_chars += ch.to_digit().unwrap();
        } else {
            break;
        }
    }
    if num_chars == 0 {
        return Err(RespError { explanation: "could not parse num of chars" })
    }
}

fn resp_parse(data: &[u8]) -> Result<&[u8], RespError> {
    match data[0] {
        b'+' => {
            Ok(&data[1..data.len() - 2])
        }
        b'$' => {
            Ok(data)
        }
        _ => Err(RespError { explanation: "not valid because of first char" })
    }
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