export build_request;

import std::net::url;
import std::net::url::url;

fn build_request(url: url) -> ~str {

    let host = url.host;
    let path = if url.path.is_not_empty() { url.path } else { ~"/" };

    let request_header = #fmt("GET %s HTTP/1.0\u000D\u000AHost: %s\u000D\u000A\u000D\u000A",
                              path, host);

    ret request_header;
}

#[test]
fn should_request_slash_when_path_is_empty() {
    let url = url::from_str(~"http://host").get();
    assert url.path.is_empty();
    let headers = build_request(url);
    assert headers.contains(~"GET / ");
}