use super::generic::parse_number;

pub enum Flag {
    Org(u16),
}

impl Flag {
    pub fn new(str: &str) -> Result<Option<Self>, String> {
        if str.starts_with('.') {
            let line_splited = str.split(" ").collect::<Vec<&str>>();
            let keyword = &line_splited[0][1..];
            match keyword {
                "org" => {
                    if let Some(addr) = parse_number::<u16>(line_splited[1]) {
                        Ok(Some(Flag::Org(addr)))
                    } else {
                        Err(format!("Cannot parse address: {}", line_splited[1]))
                    }
                },
                _ => { Err(format!("Unknow flag keyword: .{}", keyword)) }
            }
        } else {
            Ok(None)
        }
    }
}
