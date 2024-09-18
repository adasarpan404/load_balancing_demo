#[derive(Debug)]
struct Server {
    name: String,
    weight: usize,
}

pub struct WeightedRoundRobinLoadBalancer {
    servers: Vec<Server>,
    current_server_index: usize,
    gcd: usize,
    current_weight: usize,
    max_weight: usize,
}

impl WeightedRoundRobinLoadBalancer {
    pub fn new() -> Self {
        WeightedRoundRobinLoadBalancer {
            servers: Vec::new(),
            current_server_index: 0,
            gcd: 0,
            current_weight: 0,
            max_weight: 0,
        }
    }

    fn get_gcd(&self, a: usize, b: usize) -> usize {
        let mut a = a;
        let mut b = b;

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn get_max_weight(&self) -> usize {
        self.servers.iter().map(|s| s.weight).max().unwrap_or(0)
    }

    pub fn add_server(&mut self, name: String, weight: usize) {
        let server = Server { name, weight };

        if self.servers.is_empty() {
            self.gcd = server.weight;
        } else {
            self.gcd = self.get_gcd(self.gcd, server.weight);
        }
        self.max_weight = self.get_max_weight();
        self.servers.push(server);
    }

    pub fn next_server(&mut self) -> Option<String> {
        if self.servers.is_empty() {
            return None;
        }

        loop {
            self.current_server_index = (self.current_server_index + 1) % self.servers.len();
            if self.current_server_index == 0 {
                self.current_weight = self.current_weight.saturating_sub(self.gcd);
                if self.current_weight <= 0 {
                    self.current_weight = self.max_weight;
                    if self.current_weight == 0 {
                        return None;
                    }
                }
            }

            if self.servers[self.current_server_index].weight >= self.current_weight {
                return Some(self.servers[self.current_server_index].name.clone());
            }
        }
    }
}
