use super::generic::parse_number;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Flag {
    Org(u16),
    Include(String),
    Label((String, u16)),
}

impl Flag {
    pub fn new(str: &str) -> Result<Option<Self>, String> {
        if str.starts_with('.') {
            let line_splited = str.split(" ").collect::<Vec<&str>>();
            let keyword = &line_splited[0][1..];
            match keyword {
                "org" => {
                    if line_splited.len() == 2 {
                        if let Some(addr) = parse_number::<u16>(line_splited[1]) {
                            Ok(Some(Flag::Org(addr)))
                        } else {
                            Err(format!("Cannot parse address: {}", line_splited[1]))
                        }
                    } else {
                        Err("Argument does not match should be: .org <ADDR>".to_string())
                    }
                }
                "include" => {
                    if line_splited.len() == 2 {
                        if line_splited[1].starts_with('"') && line_splited[1].ends_with('"') {
                            let path_str = &line_splited[1][1..line_splited[1].len() - 1];
                            Ok(Some(Flag::Include(path_str.to_string())))
                        } else {
                            Err("File path need to be string format!".to_string())
                        }
                    } else {
                        Err("Argument does not match should be: .include \"<PATH>\"".to_string())
                    }
                }
                "label" => {
                    if line_splited.len() == 3 {
                        let re = Regex::new(r"^[a-z_0-9]+$").unwrap();
                        if re.is_match(line_splited[1]) {
                            if let Some(addr) = parse_number::<u16>(line_splited[2]) {
                                Ok(Some(Flag::Label((line_splited[1].to_string(), addr))))
                            } else {
                                Err(format!("Cannot parse address: {}", line_splited[2]))
                            }
                        } else {
                            Err(format!("Label is not correct (only [a-z_0-9]): {}", line_splited[1]))
                        }
                    } else {
                        Err("Argument does not match should be: .label <NAME> <ADDR>".to_string())
                    }
                }
                _ => Err(format!("Unknow flag keyword: .{}", keyword)),
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_flag() {
        let new_instance = Flag::new(". $8000");
        assert!(new_instance.is_err());

        let new_instance = Flag::new(".");
        assert!(new_instance.is_err());
    }

    #[test]
    fn test_org_flag() {
        let new_instance = Flag::new(".org $8000");
        assert!(new_instance.is_ok());
        assert_eq!(new_instance.unwrap().unwrap(), Flag::Org(32768));

        let new_instance = Flag::new(".org #5");
        assert!(new_instance.is_err());

        let new_instance = Flag::new(".org");
        assert!(new_instance.is_err());
    }

    #[test]
    fn test_include_flag() {
        let new_instance = Flag::new(".include \"./test/test.tasm\"");
        assert!(new_instance.is_ok());
        assert_eq!(
            new_instance.unwrap().unwrap(),
            Flag::Include("./test/test.tasm".to_string())
        );

        let new_instance = Flag::new(".include ./test/test.tasm");
        assert!(new_instance.is_err());

        let new_instance = Flag::new(".include");
        assert!(new_instance.is_err());
    }

    #[test]
    fn test_label_flag() {
        let new_instance = Flag::new(".label test 0");
        assert!(new_instance.is_ok());
        assert_eq!(
            new_instance.unwrap().unwrap(),
            Flag::Label(("test".to_string(), 0))
        );

        let new_instance = Flag::new(".label test_0 $8100");
        assert!(new_instance.is_ok());
        assert_eq!(
            new_instance.unwrap().unwrap(),
            Flag::Label(("test_0".to_string(), 33024))
        );

        let new_instance = Flag::new(".label");
        assert!(new_instance.is_err());

        let new_instance = Flag::new(".label test");
        assert!(new_instance.is_err());

        let new_instance = Flag::new(".label test aze");
        assert!(new_instance.is_err());

        let new_instance = Flag::new(".label wr-ong 0");
        assert!(new_instance.is_err());
    }
}
