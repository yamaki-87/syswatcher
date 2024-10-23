use std::collections::HashMap;

use sysinfo::{Pid, Process};

use super::SysInfo;


pub trait SysProcess{
    fn get_processes(&self) -> &HashMap<Pid, Process>;
}

impl SysProcess for SysInfo {
    
    fn get_processes(&self) -> &HashMap<Pid, Process> {
        self.system.processes()
    }
}

#[cfg(test)]
mod test{
    use shared::error::AppResult;

    use crate::system::SysInfo;

    use super::SysProcess;

    #[test]
    fn test_get_processes()->AppResult<()>{
        let si = SysInfo::new();
        let processes = si.get_processes();

        assert_ne!(0,processes.len());
        Ok(())
    }
}