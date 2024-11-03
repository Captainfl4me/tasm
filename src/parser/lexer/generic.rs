pub fn parse_number<T: num::Integer + std::str::FromStr>(str: &str) -> Option<T> {
    if let Some(hex_number) = str.strip_prefix('$') {
        Some(T::from_str_radix(hex_number, 16).ok().unwrap())
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
