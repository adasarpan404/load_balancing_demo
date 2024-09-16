pub struct RoundRobinLoadBalancer {
    server: Vec<String>,
    current_server_index: usize,
}

impl RoundRobinLoadBalancer {
    pub fn new() -> Self {
        RoundRobinLoadBalancer {
            server: Vec::new(),
            current_server_index: 0,
        }
    }

    pub fn add_server(&mut self, server: String) {
        self.server.push(server);
    }

    pub fn next_server(&mut self) -> Option<String> {
        if self.server.is_empty() {
            return None;
        }

        let next_server = self.server[self.current_server_index].clone();
        self.current_server_index = (self.current_server_index + 1) % self.server.len();
        Some(next_server)
    }
}
