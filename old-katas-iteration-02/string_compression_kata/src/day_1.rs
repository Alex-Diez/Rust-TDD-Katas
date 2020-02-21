pub fn compress(src: &str) -> String {
    if src.is_empty() {
        src.to_owned()
    } else {
        let mut compressed = String::new();
        let mut i = 1;
        let len = src.len();
        let mut chars = src.chars();
        let mut c = chars.next().unwrap();
        let mut count = 1;
        while i < len {
            count = 1;
            while let Some(n) = chars.next() {
                i += 1;
                if c == n {
                    count += 1;
                } else {
                    compressed.push_str(count.to_string().as_str());
                    compressed.push(c);
                    c = n;
                    break;
                }
            }
        }
        compressed.push_str(count.to_string().as_str());
        compressed.push(c);
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
    fn compress_single_char_string() {
        assert_eq!(compress("a"), "1a");
    }

    #[test]
    fn compress_string_of_unique_chars() {
        assert_eq!(compress("abc"), "1a1b1c");
    }

    #[test]
    fn compress_string_with_doubled_chars() {
        assert_eq!(compress("aabbcc"), "2a2b2c");
    }
}
