use std::collections::BTreeMap;
use std::collections::HashMap;

use shared::util::DisplayOsStr;
use sysinfo::{Pid, Process};

use super::SysInfo;

pub trait SysProcess {
    fn get_processes(&self) -> String;
    fn get_processes_map(&self) -> HashMap<Pid, Process>;
}

///TODO Process画面が崩れる
impl SysProcess for SysInfo {
    fn get_processes(&self) -> String {
        let temp = BTreeMap::from_iter(
            self.system
                .processes()
                .iter()
                .map(|(pid, process)| (pid.as_u32(), process)),
        );

        temp.iter().map(|(pid,process) |{
            format!("{}\t{}\n",pid,DisplayOsStr::new(process.name()))
        }).collect()
    }

    fn get_processes_map(&self) -> HashMap<Pid, Process> {
        todo!()
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
