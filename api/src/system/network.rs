use super::SysInfo;


pub trait Networks {
    #[warn(non_upper_case_globals)]
    const HAVE_IPv6_IPv4:usize = 2;
    fn refresh_networks(&mut self);
    fn get_networks_info(&self)->Vec<String>;
}

impl Networks for SysInfo {

    fn refresh_networks(&mut self) {
        self.networks.refresh_list();
    }

    fn get_networks_info(&self)->Vec<String> {
        self.networks.iter().map(|(name,net)| {
            let ip_network = net.ip_networks();
            let mut network_info = format!("{}",name,);
            if ip_network.len() == Self::HAVE_IPv6_IPv4{
                network_info.push_str(&format!("\n\tIPv6:{}",ip_network[0].addr));
                network_info.push_str(&format!("\n\tIPv4:{}",ip_network[1].addr));
            }

            network_info.push_str(&format!("\n\tMAC Addr:{}",net.mac_address()));

            network_info
        }).collect()
    }
}