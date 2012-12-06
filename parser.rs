//! Higher-level Rust constructs for http_parser

use vec::raw::from_buf_raw;
use libc::{c_int, c_void, c_char, size_t};
use ptr::{null, to_unsafe_ptr};
use http_parser::{http_parser_settings, HTTP_RESPONSE};
use http_parser::bindgen::{http_parser_init, http_parser_execute};

pub type HttpCallback = fn@() -> bool;
pub type HttpDataCallback = fn@(data: ~[u8]) -> bool;

pub type ParserCallbacks = {
    on_message_begin: HttpCallback,
    on_url: HttpDataCallback,
    on_header_field: HttpDataCallback,
    on_header_value: HttpDataCallback,
    on_headers_complete: HttpCallback,
    on_body: HttpDataCallback,
    on_message_complete: HttpCallback
};

pub struct Parser {
    mut http_parser: http_parser::http_parser,
    settings: http_parser_settings
}

pub fn Parser() -> Parser {
    let http_parser = {
        _type_flags: 0,
        state: 0,
        header_state: 0,
        index: 0,
        nread: 0,
        content_length: 0,
        http_major: 0,
        http_minor: 0,
        status_code: 0,
        method: 0,
        http_errno_upgrade: 0,
        data: null()
    };

    http_parser_init(to_unsafe_ptr(&http_parser), HTTP_RESPONSE);

    let settings = {
        on_message_begin: on_message_begin,
        on_url: on_url,
        on_header_field: on_header_field,
        on_header_value: on_header_value,
        on_headers_complete: on_headers_complete,
        on_body: on_body,
        on_message_complete: on_message_complete
    };

    Parser {
        http_parser: move http_parser,
        settings: move settings
    }
}

impl Parser {
    fn execute(data: &[u8], callbacks: &ParserCallbacks) -> uint {
        self.http_parser.data = to_unsafe_ptr(callbacks) as *c_void;
        do vec::as_imm_buf(data) |buf, _i| {
            http_parser_execute(to_unsafe_ptr(&self.http_parser),
                                to_unsafe_ptr(&self.settings),
                                buf as *c_char, data.len() as size_t) as uint
        }
    }

    fn status_code() -> uint {
        self.http_parser.status_code as uint
    }
}

fn callbacks(http_parser: *http_parser::http_parser) -> *ParserCallbacks {
    unsafe {
        assert (*http_parser).data.is_not_null();
        return (*http_parser).data as *ParserCallbacks;
    }
}

extern fn on_message_begin(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_message_begin)()) as c_int
    }
}

extern fn on_url(http_parser: *http_parser::http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        (!(((*callbacks(http_parser)).on_url)(from_buf_raw(at, length as uint)))) as c_int
    }
}

extern fn on_header_field(http_parser: *http_parser::http_parser, at: *u8, length: size_t) ->
        c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_header_field)(from_buf_raw(at, length as uint))) as c_int
    }
}

extern fn on_header_value(http_parser: *http_parser::http_parser, at: *u8, length: size_t) ->
        c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_header_value)(from_buf_raw(at, length as uint))) as c_int
    }
}

extern fn on_headers_complete(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_headers_complete)()) as c_int
    }
}

extern fn on_body(http_parser: *http_parser::http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_body)(from_buf_raw(at, length as uint))) as c_int
    }
}

extern fn on_message_complete(http_parser: *http_parser::http_parser) -> c_int {
    unsafe {
        (!((*callbacks(http_parser)).on_message_complete)()) as c_int
    }
}
