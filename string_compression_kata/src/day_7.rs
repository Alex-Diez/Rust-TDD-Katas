pub fn compress(src: &str) -> String {
    if src.is_empty() {
        src.to_owned()
    } else {
        let mut compressed = String::new();
        let mut chars = src.chars().peekable();
        while let Some(c) = chars.peek().cloned() {
            let mut counter = 0;
            while let Some(n) = chars.peek().cloned() {
                if c == n {
                    counter += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            compressed.push_str(counter.to_string().as_str());
            compressed.push(c);
        }
        compressed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_empty_string() {
        assert_eq!(compress(""), "");
    }

    #[test]
    fn compress_unique_chars_string() {
        assert_eq!(compress("abc"), "1a1b1c");
    }

    #[test]
    fn compress_doubled_chars_string() {
        assert_eq!(compress("aabbcc"), "2a2b2c");
    }
}
