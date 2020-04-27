# Exercises

## Unit tests

Implement a parser for a subset of [the RESP protocol](https://redis.io/topics/protocol).

In the first stage, we will parse _Simple Strings_ and _Bulk Strings_.
We use the notation `b"..."` (valid both in Python and Rust) to emphasize that these are _binary_ strings, not Unicode strings.
Redis doesn't know or care what data you put into the strings, so they can be e.g. UTF-8 or any binary data.
Therefore it's important not to make any assumptions not permitted by the protocol explicitly.

### Examples

_Simple Strings_:

    +OK

    Raw: b"+OK\r\n"

_Bulk Strings_:

    $6
    foobar

    Raw: b"$6\r\nfoobar\r\n"


### Steps

1. Write a function named `resp_parse`, that takes a sequence of bytes (byte string) as an argument,
   and returns either a byte string or an error value (`Result` in Rust, specific exception in Python).
   
   Python (syntax is Python 3):
   
   ```
   def resp_parse(data: bytes) -> bytes:
       pass
   ```

   Rust:
   
   ```
   fn resp_parse(data: &[u8]) -> &[u8] {
   }
   ```
    

   Rust implementation details:
   - Use an `enum` to represent the valid values.
   - Return a `Result<String, Error>` that contains either the string value or an error.

2. Write a test that verifies correct parsing of the above examples.
