/*!

Parses HTTP headers

*/

export parse, parse_error;

import result::{result, ok, err};
import response_headers::{ResponseHeader, ResponseHeaderBlock};

/**
The result of parsing the headers out of a u8 buffer is a set of HTTP
headers plus whatever following bytes were not part of the header
*/
type ParseResult = {
    header_block: ResponseHeaderBlock,
    rest: ~[u8]
};

enum ParseError {
    /// Returned when the header block isn't terminated by a double-
    /// newline, i.e. we need to keep receiving data
    IncompleteHeader,
    BadUnicode
}

fn parse(data: &[u8]) -> result<ParseResult, ParseError> {

    // Get a slice of all headers through the end of the
    // double-newline that begins the payload

    let mut slice_size = 0;

    let header_slice = get_header_slice(data);
    let header_str = header_slice.chain( |header_slice| {
        slice_size = header_slice.len();
        get_header_str(header_slice)
    });
    let headers = header_str.chain(parse_headers);
    let result = headers.chain( |headers| {
        ok({
            header_block: {
                headers: headers
            },
            rest: ~[]
        })
    });
    ret result;
}

// FIXME: Would prefer to return a slice here but am getting
// compiler errors
fn get_header_slice(data: &[u8]) -> result<~[u8], ParseError> {
    let double_newline = &[0x0D, 0x0A, 0x0D, 0x0A];

    if data.len() < double_newline.len() {
        ret err(IncompleteHeader);
    }

    for uint::range(0, data.len() - double_newline.len()) |i| {
        let subslice = vec::view(data, i, i + double_newline.len());
        if subslice == double_newline {
            ret ok(data.slice(0, i + double_newline.len()));
        }
    }

    ret err(IncompleteHeader);
}

#[test]
fn get_header_slice_should_return_incomplete_header_if_the_data_is_not_large_enough_for_a_double_newline() {
    // This isn't even enough bytes for a double newline
    let data = &[65, 65, 65];
    assert get_header_slice(data) == err(IncompleteHeader);
}

#[test]
fn get_header_slice_should_return_incomplete_header_if_it_does_not_encounter_a_double_newline() {
    // This isn't even enough bytes for a double newline
    let data = &[65, 65, 65, 65, 65, 65];
    assert get_header_slice(data) == err(IncompleteHeader);
}

fn get_header_str(data: &[u8]) -> result<str, ParseError> {
    if str::is_utf8(data) {
        // FIXME: from_bytes requires a unique vec
        let data = data.slice(0, data.len());
        ok(str::from_bytes(data))
    } else {
        err(BadUnicode)
    }
}

#[test]
fn get_header_str_should_return_bad_unicode_error_if_data_is_not_utf8() {
    let double_newline = ~[0x0D, 0x0A, 0x0D, 0x0A];
    let data = ~[0xFF, 0xFF, 0xFF, 0xFF] + double_newline;
    assert get_header_str(data) == err(BadUnicode);
}

#[test]
fn get_header_str_should_convert_to_str() {
    let double_newline = ~[0x0D, 0x0A, 0x0D, 0x0A];
    let data = ~[65, 65, 65, 65] + double_newline;
    assert get_header_str(data) == ok("AAAA\u000D\u000A\u000D\u000A");
}

fn parse_headers(&&text: str) -> result<~[ResponseHeader], ParseError> {
    fail
}
