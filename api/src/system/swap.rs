use super::SysInfo;

pub trait Swap {
    fn get_swap(&self,)->f64;
    fn get_total_swap(&self,)->u64;
}

impl Swap for SysInfo {
    fn get_swap(&self) -> f64 {
        self.system.used_swap() as f64 / self.system.total_swap() as f64 * 100.
    }

    fn get_total_swap(&self) -> u64 {
        self.system.total_swap() / Self::GIB
    }
}


#[cfg(test)]
mod test {
    use shared::error::AppResult;

    use crate::system::{swap::Swap, SysInfo};


    #[test]
    fn swap() -> AppResult<()> {
        let si = SysInfo::new();
        let swap = si.get_swap();
        println!("{swap}");
        let total_swap = si.get_total_swap();
        assert_ne!(0,total_swap);
        Ok(())
    }
}
