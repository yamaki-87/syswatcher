use std::collections::HashMap;

use shared::error::{AppError, AppResult};
use sysinfo::{Disks, Networks, Pid, Process, System};

pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod prelude;

const UNKONW: &str = "unkonw";

pub struct SysInfo {
    system: System,
    disks: Disks,
    networks: Networks,
}

impl SysInfo {
    const BYTE: u64 = 1024;
    const GIB: u64 = 1024 * 1024 * 1024;

    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();

        let networks = Networks::new_with_refreshed_list();
        Self {
            system: sys,
            disks,
            networks,
        }
    }

    #[inline]
    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
    }

    pub fn get_processes(&self) -> &HashMap<Pid, Process> {
        self.system.processes()
    }

    pub fn get_swap(&self) -> f64 {
        self.system.used_swap() as f64 / self.system.total_swap() as f64 * 100.
    }

    pub fn get_total_swap(&self) -> u64 {
        self.system.total_swap() / Self::GIB
    }
}

impl Default for SysInfo {
    fn default() -> Self {
        SysInfo::new()
    }
}
pub struct SysData {
    host: String,
    cpu_arch: String,
    boot_time: u64,
    uptime: u64,
    long_os_ver: String,
    kernel_ver: String,
}
impl SysData {
    fn new() -> Self {
        Self {
            host: get_host(),
            cpu_arch: get_cpu_arch(),
            boot_time: get_boot_time(),
            uptime: get_uptime(),
            long_os_ver: get_long_os_ver(),
            kernel_ver: get_kernel_ver(),
        }
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_cpu_arch(&self) -> &str {
        &self.cpu_arch
    }

    pub fn get_boot_time(&self) -> u64 {
        self.boot_time
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime
    }

    pub fn get_os_long_ver(&self) -> &str {
        &self.long_os_ver
    }

    pub fn get_kernel_ver(&self) -> &str {
        &self.kernel_ver
    }
}

impl Default for SysData {
    fn default() -> Self {
        SysData::new()
    }
}

fn get_os() -> String {
    if let Some(os) = System::name() {
        os
    } else {
        UNKONW.into()
    }
}

fn get_host() -> String {
    if let Some(host) = System::host_name() {
        host
    } else {
        UNKONW.into()
    }
}

fn get_os_version() -> String {
    if let Some(ver) = System::os_version() {
        ver
    } else {
        UNKONW.into()
    }
}

fn get_cpu_arch() -> String {
    if let Some(arch) = System::cpu_arch() {
        arch
    } else {
        UNKONW.into()
    }
}

fn get_boot_time() -> u64 {
    System::boot_time() / 3600
}

fn get_uptime() -> u64 {
    System::uptime() / 3600
}

fn get_long_os_ver() -> String {
    if let Some(ver) = System::long_os_version() {
        ver
    } else {
        UNKONW.into()
    }
}

fn get_kernel_ver() -> String {
    if let Some(k_ver) = System::kernel_version() {
        k_ver
    } else {
        UNKONW.into()
    }
}

pub fn get_networks_data() -> Networks {
    let net = Networks::new_with_refreshed_list();
    for (str, n) in net.list() {}
    net
}

pub fn supported() -> AppResult<()> {
    if sysinfo::IS_SUPPORTED_SYSTEM {
        Ok(())
    } else {
        Err(AppError::NoSupported)
    }
}

#[cfg(test)]
mod test {
    use shared::error::AppResult;
    use sysinfo::{Networks, System};

    #[test]
    fn swap() -> AppResult<()> {
        let mut sys = System::new_all();
        sys.refresh_all();
        println!("free: {}", &sys.free_swap() / 1024 * 1024 * 1024);
        println!("used: {}", &sys.used_swap());
        println!("total: {}", &sys.total_swap() / (1024 * 1024 * 1024));

        Ok(())
    }

    #[test]
    fn network() -> AppResult<()> {
        let network = Networks::new_with_refreshed_list();
        for n in network.list() {
            println!("{:?}", n);
            println!("ip address = {:?}", n.1.ip_networks());
            println!("mac address = {}", n.1.mac_address());
        }
        Ok(())
    }

    #[test]
    fn disk() -> AppResult<()> {
        use sysinfo::Disks;
        let disks = Disks::new_with_refreshed_list();
        for d in disks.list() {
            println!("{:?}", d);
        }
        Ok(())
    }
}
