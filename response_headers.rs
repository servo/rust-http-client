enum ResponseHeader {
    Unknown(~str)
}

struct ResponseHeaderBlock {
    headers: ~[ResponseHeader]
}