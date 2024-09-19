struct Server {
    name: String,
    load: f64,
}

pub struct DynamicLoadBalancer {
    servers: Vec<Server>,
}

impl DynamicLoadBalancer {
    pub fn new() -> Self {
        DynamicLoadBalancer {
            servers: Vec::new(),
        }
    }

    pub fn add_server(&mut self, server_name: String) {
        let server = Server {
            name: server_name,
            load: 0.0,
        };

        self.servers.push(server);
    }

    pub fn next_server(&mut self, load: f64) -> Option<String> {
        if self.servers.is_empty() {
            return None;
        }

        let mut min_index = 0;
        let mut min_load = self.servers[0].load;

        for (i, server) in self.servers.iter().enumerate().skip(1) {
            if server.load < min_load {
                min_load = server.load;
                min_index = i;
            }
        }

        self.servers[min_index].load += load;

        Some(self.servers[min_index].name.clone())
    }
}
