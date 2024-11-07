pub fn parse_number<T: num::Integer + std::str::FromStr>(str: &str) -> Option<T> {
    if let Some(hex_number) = str.strip_prefix('$') {
        if let Ok(num) = T::from_str_radix(hex_number, 16) {
            Some(num)
        } else {
            None
        }
    } else if let Ok(num) = str.parse::<T>() {
        Some(num)
    } else {
        None
    }
}

pub fn trim_line(str: &str) -> &str {
    let mut str = str;
    if let Some(comment_index) = str.find(";") {
         str = &str[0..comment_index];
    }
    str.trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_parsing() {
        // Hex parsing
        let new_number = parse_number::<u16>("$FffF");
        assert!(new_number.is_some());
        assert_eq!(new_number.unwrap(), 65535);

        // Decimal parsing
        let new_number = parse_number::<u8>("53");
        assert!(new_number.is_some());
        assert_eq!(new_number.unwrap(), 53);
        
        // Decimal overflow
        let new_number = parse_number::<u8>("1024");
        assert!(new_number.is_none());

        // Hex overflow
        let new_number = parse_number::<u8>("$FFFF");
        assert!(new_number.is_none());

        // Hex as decimal
        let new_number = parse_number::<u8>("ab");
        assert!(new_number.is_none());
    }
}
