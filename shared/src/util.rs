use std::{ffi::OsStr, fmt};


#[test]
fn t(){
    let s = r#""test""#.to_string();
    println!("s = {}",&s);

    let ss = s.trim_matches('"');

    println!("new s = {}",ss);
}

pub struct DisplayOsStr<'a>(&'a OsStr);

impl<'a> DisplayOsStr<'a> {
    pub fn new(str:&'a OsStr,)->Self{
        Self(str)
    }
}

impl<'a> fmt::Display for DisplayOsStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.0.to_string_lossy())
    }
}