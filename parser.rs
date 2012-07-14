//! Higher-level Rust constructs for http_parser

export HttpCallback, HttpDataCallback;
export ParserCallbacks, Parser;

import vec::unsafe::from_buf;
import libc::{c_int, c_void, c_char, size_t};
import ptr::{null, addr_of};
import http_parser::{
    http_parser, http_parser_settings, HTTP_REQUEST
};
import http_parser::bindgen::{http_parser_init, http_parser_execute};

type HttpCallback = fn@() -> bool;
type HttpDataCallback = fn@(+~[u8]) -> bool;

type ParserCallbacks = {
    on_message_begin: HttpCallback,
    on_url: HttpDataCallback,
    on_header_field: HttpDataCallback,
    on_header_value: HttpDataCallback,
    on_headers_complete: HttpCallback,
    on_body: HttpDataCallback,
    on_message_complete: HttpCallback
};

class Parser {
    let mut http_parser: http_parser;
    let settings: http_parser_settings;
    let callbacks: ParserCallbacks;

    new(callbacks: ParserCallbacks) {
        self.callbacks = callbacks;

        self.http_parser = {
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

        http_parser_init(addr_of(self.http_parser), HTTP_REQUEST);
        self.http_parser.data = addr_of(self.callbacks) as *c_void;

        self.settings = {
            on_message_begin: on_message_begin,
            on_url: on_url,
            on_header_field: on_header_field,
            on_header_value: on_header_value,
            on_headers_complete: on_headers_complete,
            on_body: on_body,
            on_message_complete: on_message_complete
        };
    }

    fn execute(data: &[u8]) {
        do vec::as_buf(data) |buf| {
            http_parser_execute(addr_of(self.http_parser),
                                addr_of(self.settings),
                                buf as *c_char, data.len() as size_t);
        }
    }
}

fn callbacks(http_parser: *http_parser) -> *ParserCallbacks {
    unsafe {
        assert (*http_parser).data.is_not_null();
        ret (*http_parser).data as *ParserCallbacks;
    }
}

extern fn on_message_begin(http_parser: *http_parser) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_message_begin() as c_int)
    }
}

extern fn on_url(http_parser: *http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_url(from_buf(at, length as uint)) as c_int)
    }
}

extern fn on_header_field(http_parser: *http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_header_field(from_buf(at, length as uint)) as c_int)
    }
}

extern fn on_header_value(http_parser: *http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_header_value(from_buf(at, length as uint)) as c_int)
    }
}

extern fn on_headers_complete(http_parser: *http_parser) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_headers_complete() as c_int)
    }
}

extern fn on_body(http_parser: *http_parser, at: *u8, length: size_t) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_body(from_buf(at, length as uint)) as c_int)
    }
}

extern fn on_message_complete(http_parser: *http_parser) -> c_int {
    unsafe {
        !((*callbacks(http_parser)).on_message_complete() as c_int)
    }
}