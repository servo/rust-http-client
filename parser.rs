/*!

Parses HTTP headers

*/

export parse, parse_error;

import result::result;
import response_headers::ResponseHeaders;

/**
The result of parsing the headers out of a u8 buffer is a set of HTTP
headers plus whatever following bytes were not part of the header
*/
type ParseResult = {
    headers: ResponseHeaders,
    rest: ~[u8]
};

enum ParseError {
    /// Returned when the header block isn't terminated by a double-
    /// newline, i.e. we need to keep receiving data
    IncompleteHeader,
}

fn parse(data: &[u8]) -> result<ParseResult, ParseError> {
    fail
}
