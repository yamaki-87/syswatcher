use super::SysInfo;
use std::io::prelude::*;
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