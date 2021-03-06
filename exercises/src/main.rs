#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::str;

use quickcheck::{Arbitrary, Gen};

use rand::{
    self,
    distributions::{Distribution, Uniform},
}; // 0.6.4


const NEWLINE: &[u8] = b"\r\n";

#[derive(Debug, PartialEq)]
enum RespError {
    MissingLength,
    InvalidLength,
    InvalidData,
    MissingEndOfLine,
    NotEnoughData {
        required_len: usize,
        actual_len: usize,
    },
}

#[derive(Debug, PartialEq)]
enum RedisValue<'data> {
    SimpleString(&'data [u8]),
    BulkString(&'data [u8]),
    Integer(i64),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
enum RedisValueOwned {
    SimpleString(Vec<u8>),
    BulkString(Vec<u8>),
    Integer(i64),
    Null,
}

fn resp_parse(data: &[u8]) -> Result<RedisValue, RespError> {
    match &data {
        [b'+', data @ ..] => parse_simple_string(data),
        [b'$', data @ ..] => parse_bulk_string(data),
        [b':', data @ ..] => parse_integer(data),
        _ => Err(RespError::InvalidData),
    }
}

fn parse_simple_string(data: &[u8]) -> Result<RedisValue, RespError> {
    match split_line(data) {
        (Some(line), _) => Ok(RedisValue::SimpleString(line)),
        (None, _) => Err(RespError::MissingEndOfLine),
    }
}

fn parse_bulk_string(data: &[u8]) -> Result<RedisValue, RespError> {
    match split_line(data) {
        (Some(length), data) => {
            let length = str::from_utf8(length).map_err(|_| RespError::InvalidLength)?;
            let length: isize = length.parse().map_err(|_| RespError::InvalidLength)?;

            let length = if length == -1 {
                // Null bulk string
                return Ok(RedisValue::Null);
            } else {
                length as usize
            };

            let required_len = length + NEWLINE.len();
            let actual_len = data.len();

            if actual_len < required_len {
                Err(RespError::NotEnoughData {
                    required_len,
                    actual_len,
                })
            } else {
                let data = &data[..length];
                Ok(RedisValue::BulkString(data))
            }
        }
        (None, _) => Err(RespError::MissingLength),
    }
}

fn parse_integer(data: &[u8]) -> Result<RedisValue, RespError> {
    match split_line(data) {
        (Some(line), _) => {
            let string = str::from_utf8(line).map_err(|_| RespError::InvalidData)?;
            let value: i64 = string.parse().map_err(|_| RespError::InvalidData)?;
            Ok(RedisValue::Integer(value))
        }
        (None, _) => Err(RespError::MissingEndOfLine),
    }
}

fn split_line(data: &[u8]) -> (Option<&[u8]>, &[u8]) {
    find_subsequence(data, NEWLINE)
        .map(|i| {
            let line = &data[..i];
            let rest = &data[i + NEWLINE.len()..];
            (Some(line), rest)
        })
        .unwrap_or((None, data))
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

impl Arbitrary for RedisValueOwned {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let mut rng = rand::thread_rng();
        let die_range = Uniform::new_inclusive(0, 3);
        let die = die_range.sample(&mut rng);
        match die {
            0 => {
                let data: Vec<u8> = Arbitrary::arbitrary(g);
                RedisValueOwned::BulkString(data)
            }
            1 => {
                let data: Vec<u8> = Arbitrary::arbitrary(g);
                RedisValueOwned::SimpleString(data)
            }
            2 => RedisValueOwned::Null,
            _ => {
                let data: i64 = Arbitrary::arbitrary(g);
                RedisValueOwned::Integer(data)
            }
        }
    }
}

#[quickcheck]
fn qc_roundtrip_simple_string(input: RedisValueOwned) -> bool {
    match input {
        RedisValueOwned::BulkString(input_data) => {
            let mut resp = vec![];
            resp.extend_from_slice(b"$");
            resp.extend_from_slice(input_data.len().to_string().as_bytes());
            resp.extend_from_slice(b"\r\n");
            resp.extend_from_slice(input_data.as_slice());
            resp.extend_from_slice(b"\r\n");

            eprintln!("Testing Bulk String with length {}", input_data.len());
            eprintln!("RESP: {:?}", String::from_utf8_lossy(&resp));

            let value = resp_parse(resp.as_slice()).expect("valid RESP data");

            match value {
                RedisValue::BulkString(parsed_data) => parsed_data == input_data.as_slice(),
                _ => false,
            }
        }
        RedisValueOwned::SimpleString(input_data) => {
            let mut resp = vec![];
            resp.extend_from_slice(b"+");
            resp.extend_from_slice(input_data.as_slice());
            resp.extend_from_slice(b"\r\n");
            eprintln!("Testing Simple String with length {}", input_data.len());
            eprintln!("RESP: {:?}", String::from_utf8_lossy(&resp));
            let value = resp_parse(resp.as_slice()).expect("valid RESP data");
            match value {
                RedisValue::SimpleString(parsed_data) => parsed_data == input_data.as_slice(),
                _ => false,
            }
        }
        RedisValueOwned::Integer(input_data) => {
            let mut resp = vec![];
            resp.extend_from_slice(b":");
            resp.extend_from_slice(input_data.to_string().as_str().as_bytes());
            resp.extend_from_slice(b"\r\n");
            eprintln!("Testing Integer");
            eprintln!("RESP: {:?}", String::from_utf8_lossy(&resp));
            let value = resp_parse(resp.as_slice()).expect("valid RESP data");
            match value {
                RedisValue::Integer(parsed_data) => parsed_data == input_data,
                _ => false,
            }
        },
        _ => {
            let value = resp_parse(b"$-1\r\n").expect("valid RESP data");
            RedisValue::Null == value
        },
    }
}

fn assert_parse_eq(input: &[u8], expected: &RedisValue) {
    let parsed = &resp_parse(input).unwrap();

    let expected_str: String = match expected {
        RedisValue::SimpleString(s) => String::from(str::from_utf8(s).unwrap()),
        RedisValue::BulkString(s) => String::from(str::from_utf8(s).unwrap()),
        RedisValue::Integer(i) => format!("{}", i),
        RedisValue::Null => String::from("(nil)"),
    };

    let parsed_str = match parsed {
        RedisValue::SimpleString(s) => String::from(str::from_utf8(s).unwrap()),
        RedisValue::BulkString(s) => String::from(str::from_utf8(s).unwrap()),
        RedisValue::Integer(i) => format!("{}", i),
        RedisValue::Null => String::from("(nil)"),
    };

    assert_eq!(
        parsed, expected,
        "expected: '{}', got: '{}'",
        expected_str, parsed_str,
    );
}

fn assert_parse_error(input: &[u8], error: &RespError) {
    match resp_parse(input) {
        Err(ref e) => assert_eq!(e, error),
        r => panic!("got unexpected result: {:?}", r),
    }
}