enum ResponseHeader {
    Unknown(~str)
}

type ResponseHeaderBlock = {
    headers: ~[ResponseHeader]
};