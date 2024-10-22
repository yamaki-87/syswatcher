
use sysinfo::{IpNetwork, MacAddr};

use super::SysInfo;

pub struct IpAddr{
    pub ipv4:String,
    pub ipv6:String,
}

impl IpAddr {
    fn new(ipv4:String,ipv6:String,)->Self{
        //Self{ipv4:Arc::new(ipv4),ipv6:Arc::new(ipv6)}
        Self{ipv4,ipv6}
    }

}
pub struct NetworkData{
    name:String,
    ip_addr:Option<IpAddr>,
    mac_addr:MacAddr,
}

impl NetworkData {
    pub fn get_name(&self)->&String{
        &self.name
    }

    pub fn get_ip_addr(&self)->&Option<IpAddr>{
        &self.ip_addr
    }

    pub fn get_mac_addr(&self)->MacAddr{
        self.mac_addr
    }
}

pub trait Networks {
    const HAVE_IPV6_IPV4:usize = 2;
    
    fn refresh_networks(&mut self);
    fn get_networks_info(&self)->Vec<NetworkData>;
}

impl Networks for SysInfo {

    fn refresh_networks(&mut self) {
        self.networks.refresh_list();
    }

    fn get_networks_info(&self)->Vec<NetworkData> {
        self.networks.iter().map(|(name,net)| {
            let ip_network = net.ip_networks();
            let mut ip_network_opt = None;
            if ip_network.len() == Self::HAVE_IPV6_IPV4{
                let temp = IpAddr::new(format!("\tIPv6:{}",ip_network[0].addr), format!("\tIPv4:{}",ip_network[1].addr));
                ip_network_opt = Some(temp);
            }

            NetworkData { name: name.clone().to_owned(), ip_addr: ip_network_opt,mac_addr: net.mac_address()}
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use shared::error::AppResult;

    use crate::system::SysInfo;

    use super::Networks;

    #[test]
    fn test_get_networks_info()->AppResult<()>{
        let si = SysInfo::new();
        let network_infos = si.get_networks_info();
        network_infos.iter().for_each(|e| {
            assert_ne!("",e.get_name());
            assert_ne!("",e.get_mac_addr().to_string());

            if let Some(ip_addr) = e.get_ip_addr(){
                assert_ne!("",ip_addr.ipv4);
                assert_ne!("",ip_addr.ipv6);
            }
        });
        Ok(())
    }
}