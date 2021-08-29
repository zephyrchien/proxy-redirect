use regex::Regex;

#[inline]
pub fn is_socks5(buf: &[u8]) -> bool { buf[0] == 0x05 }

#[inline]
pub fn is_http(buf: &[u8]) -> bool {
    if let Ok(method) = std::str::from_utf8(buf) {
        let expr = Regex::new(r"(^get|^post|^head|^connect)").unwrap();
        return expr.is_match(&method.to_lowercase());
    };
    false
}
