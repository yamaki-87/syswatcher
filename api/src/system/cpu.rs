use super::SysInfo;
pub trait Cpu {
    fn refresh_cpu(&mut self,);
    fn get_cpu(&self)->f32;

}

impl Cpu for SysInfo {
    
    #[inline]
    fn refresh_cpu(&mut self) {
        self.system.refresh_cpu_usage();
    }

    #[inline]
    fn get_cpu(&self) -> f32 {
        self.system.global_cpu_usage()
    }
}

#[cfg(test)]
mod test{
    use shared::error::AppResult;

    use crate::system::{cpu::Cpu, SysInfo};

    #[test]
    fn test_get_test()->AppResult<()>{
        let sysinfo= SysInfo::new();
        assert_ne!(0.,sysinfo.get_cpu());
        Ok(())
    }
}