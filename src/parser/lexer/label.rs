use regex::Regex;

pub struct Label {
    pub name: String,
}
impl Label {
    pub fn new(str: &str) -> Result<Option<Self>, String> {
        let re = Regex::new(r"^[a-z_0-9]*:").unwrap();
        if re.captures(str).is_some() {
            let name = str.split(":").collect::<Vec<&str>>()[0].to_string();
            return Ok(Some(Label { name }));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label() {
        let new_instance = Label::new("label_test_2:");

        assert!(new_instance.is_ok());
        assert_eq!(new_instance.unwrap().unwrap().name, "label_test_2");

        let new_instance = Label::new("halt");

        assert!(new_instance.is_ok());
        assert!(new_instance.unwrap().is_none());
    }    
}
