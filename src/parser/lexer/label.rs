use regex::Regex;

pub struct Label<'a> {
    pub name: &'a str,
}
impl<'a> Label<'a> {
    pub fn new(str: &'a str) -> Result<Option<Self>, String> {
        let re = Regex::new(r"^[a-z_0-9]*:").unwrap();
        if re.captures(str).is_some() {
            let name = str.split(":").collect::<Vec<&str>>()[0];
            return Ok(Some(Label { name }));
        }
        Ok(None)
    }
}
