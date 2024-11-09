use std::collections::HashMap;

use shared::util::DisplayOsStr;
use sysinfo::{Pid, Process};

use super::SysInfo;

pub trait SysProcess {
    fn get_processes(&self) -> Vec<String>;
}

impl SysProcess for SysInfo {
    fn get_processes(&self) -> Vec<String> {
        self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                format!("{}\t{}\n", pid.as_u32(), DisplayOsStr::new(process.name()))
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use shared::error::AppResult;

    use crate::system::SysInfo;

    use super::SysProcess;

    #[test]
    fn test_get_processes() -> AppResult<()> {
        let si = SysInfo::new();
        let processes = si.get_processes();

        assert_ne!(0, processes.len());
        Ok(())
    }
}
