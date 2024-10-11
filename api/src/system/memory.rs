use super::SysInfo;

pub trait Memory {
    fn refresh_mem(&mut self);
    fn get_memory(&self)->f64;
    fn get_total_memory(&self)->u64;
}

impl Memory for SysInfo {
    #[inline]
    fn refresh_mem(&mut self) {
        self.system.refresh_memory();
    }

    #[inline]
    fn get_memory(&self) -> f64 {
        self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.
    }

    fn get_total_memory(&self)->u64{
        self.system.total_memory() / Self::GIB
    }

}