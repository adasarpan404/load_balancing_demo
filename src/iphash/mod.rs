use std::collections::HashMap;

struct IpHashLoadBalancer {
    server_map: HashMap<String, String>,
}

impl IpHashLoadBalancer {
    pub fn new() -> Self {
        IpHashLoadBalancer {
            server_map: HashMap::new(),
        }
    }

    pub fn add_server(&mut self, ip_address: String, server_name: String) {
        self.server_map.insert(ip_address, server_name);
    }

    pub fn get_server(&mut self, ip_address: &str) -> Option<&String> {
        self.server_map.get(ip_address)
    }
}
