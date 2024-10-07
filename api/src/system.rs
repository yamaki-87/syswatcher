use std::collections::HashMap;

use sysinfo::{Networks, Pid, Process, System};

const UNKONW: &str = "unkonw";


pub struct SysInfo{
    system:System,
}

impl SysInfo {
    fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self { system: sys }
    }

    #[inline]
    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
    }

    #[inline]
    fn refresh_cpu(&mut self) {
        self.system.refresh_cpu_usage();
    }

    #[inline]
    fn refresh_mem(&mut self) {
        self.system.refresh_memory();
    }

    #[inline]
    pub fn get_memory(&self) -> f64 {
        self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.
    }

    #[inline]
    pub fn get_cpu(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn get_cpu_2(&self) -> f32 {
        self.system.cpus().iter().map(|c| c.cpu_usage()).sum()
    }

    pub fn get_processes(&self) -> &HashMap<Pid, Process> {
        self.system.processes()
    }
}

impl Default for SysInfo {
    fn default() -> Self {
        SysInfo::new()
    }
}
pub struct SysData{
    host:String,
    cpu_arch:String,
    boot_time:u64,
    uptime:u64,
    long_os_ver:String,
    kernel_ver:String,
}
impl SysData {
    fn new()->Self {
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
    Networks::new_with_refreshed_list()
}
