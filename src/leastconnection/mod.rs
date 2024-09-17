#[derive(Debug)]
struct Server {
    connection_count: usize,
    name: String,
}

impl Server {
    pub fn new(name: String) -> Self {
        Server {
            connection_count: 0,
            name,
        }
    }
}

pub struct LeastConnectionLoadBalancer {
    servers: Vec<Server>,
}

impl LeastConnectionLoadBalancer {
    pub fn new() -> Self {
        LeastConnectionLoadBalancer {
            servers: Vec::new(),
        }
    }

    pub fn add_server(&mut self, server_name: String) {
        let server = Server::new(server_name);
        self.servers.push(server);
    }

    pub fn next_server(&mut self) -> Option<String> {
        if self.servers.is_empty() {
            return None;
        }

        let mut min_index = 0;
        let mut min_connections = self.servers[0].connection_count;

        for (i, server) in self.servers.iter().enumerate() {
            if server.connection_count < min_connections {
                min_connections = server.connection_count;
                min_index = i;
            }
        }

        self.servers[min_index].connection_count += 1;
        Some(self.servers[min_index].name.clone())
    }
}
