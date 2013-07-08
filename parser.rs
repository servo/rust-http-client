// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Higher-level Rust constructs for http_parser

use http_parser;
use std::vec::raw::from_buf_raw;
use std::libc::{c_int, c_void, c_char, size_t};
use std::ptr::{null, to_unsafe_ptr};
use http_parser::{http_parser_settings, HTTP_RESPONSE};
use http_parser::{http_parser_init, http_parser_execute};

pub type HttpCallback = @fn() -> bool;
pub type HttpDataCallback = @fn(data: ~[u8]) -> bool;

pub struct ParserCallbacks {
    on_message_begin: HttpCallback,
    on_url: HttpDataCallback,
    on_header_field: HttpDataCallback,
    on_header_value: HttpDataCallback,
    on_headers_complete: HttpCallback,
    on_body: HttpDataCallback,
    on_message_complete: HttpCallback
}

pub struct Parser {
    http_parser: http_parser::http_parser,
    settings: http_parser_settings
}

pub fn Parser() -> Parser {
    let http_parser = http_parser::struct_http_parser {
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

    unsafe {
        http_parser_init(&http_parser, HTTP_RESPONSE);
    }

    let settings = http_parser::struct_http_parser_settings {
        on_message_begin: on_message_begin,
        on_url: on_url,
        on_header_field: on_header_field,
        on_header_value: on_header_value,
        on_headers_complete: on_headers_complete,
        on_body: on_body,
        on_message_complete: on_message_complete
    };

    Parser {
        http_parser: http_parser,
        settings: settings
    }
}

impl Parser {
    pub fn execute(&mut self, data: &[u8], callbacks: &ParserCallbacks) -> uint {
        unsafe {
            self.http_parser.data = to_unsafe_ptr(callbacks) as *c_void;
            do data.as_imm_buf |buf, _| {
                http_parser_execute(&self.http_parser,
                                    &self.settings,
                                    buf as *c_char,
                                    data.len() as size_t) as uint
            }
        }
    }

    pub fn status_code(&self) -> uint {
        self.http_parser.status_code as uint
    }
}

fn callbacks(http_parser: *http_parser::http_parser) -> *ParserCallbacks {
    unsafe {
        assert!((*http_parser).data.is_not_null());
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
