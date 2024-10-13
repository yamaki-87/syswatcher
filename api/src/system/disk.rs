use sysinfo::Disks;
use shared::util::DisplayOsStr;
use super::SysInfo;

pub trait Disk {
    fn refresh_disks(&mut self);
    fn get_disks_info(&self) -> Vec<String>;
}

impl Disk for SysInfo {
    fn refresh_disks(&mut self) {
        self.disks = Disks::new_with_refreshed_list();
    }

    fn get_disks_info(&self) -> Vec<String> {
        self.disks
            .list()
            .iter()
            .map(|d| {
                let available_usage = d.available_space() / Self::GIB;
                let total_usage = d.total_space() / Self::GIB;
                format!(
                    "{}:{}:{}:{}/{} GB",
                    DisplayOsStr::new(d.name()),
                    d.mount_point().display(),
                    DisplayOsStr::new(d.file_system()),
                    available_usage,
                    total_usage,
                )
            })
            .collect()
    }
}
