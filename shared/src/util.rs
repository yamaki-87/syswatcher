use std::{ffi::OsStr, fmt};

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

#[cfg(test)]
mod test{
    use std::ffi::OsStr;

    use crate::{error::AppResult, util::DisplayOsStr};

    #[test]
    fn test_displayosstr()->AppResult<()>{
        let path = OsStr::new("C:\\Users\\user\\Documents");
        let output = format!("{}",DisplayOsStr(path));
        assert_eq!("C:\\Users\\user\\Documents",output);

        Ok(())
    }
}