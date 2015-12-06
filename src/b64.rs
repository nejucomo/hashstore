use serialize::base64::{CharacterSet, Config, Newline, ToBase64};


/* Like ToBase64 but hardwired with our preferred format: url safe,
 * no whitespace, no padding.
 */
pub trait ToB64 {
    fn to_b64(&self) -> String;
}


impl ToB64 for [u8] {
    fn to_b64(&self) -> String {
        self.to_base64(Config {
            char_set: CharacterSet::UrlSafe,
            newline: Newline::LF,
            pad: false,
            line_length: None,
        })
    }
}


#[cfg(test)]
mod tests {
    use b64::ToB64;

    #[test]
    fn empty_u8() {
        let arr: [u8; 0] = [0; 0];
        assert_eq!("", arr.to_b64());
    }

    #[test]
    fn one_u8() {
        let arr: [u8; 1] = [42];
        assert_eq!("Kg", arr.to_b64());
    }
}
