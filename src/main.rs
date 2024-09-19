use dynamic::DynamicLoadBalancer;
use iphash::IpHashLoadBalancer;
use lcg::LcgRng;
use leastconnection::LeastConnectionLoadBalancer;
use roundrobin::RoundRobinLoadBalancer;
use weightedroundrobin::WeightedRoundRobinLoadBalancer;

mod dynamic;
mod iphash;
mod lcg;
mod leastconnection;
mod roundrobin;
mod weightedroundrobin;
fn main() {
    println!("Round Robin Load Balancer starts ");

    let mut roundrobinloadbalancer = RoundRobinLoadBalancer::new();

    roundrobinloadbalancer.add_server("Server1".to_string());
    roundrobinloadbalancer.add_server("Server2".to_string());
    roundrobinloadbalancer.add_server("Server3".to_string());

    println!("{:?}", roundrobinloadbalancer.next_server());
    println!("{:?}", roundrobinloadbalancer.next_server());
    println!("{:?}", roundrobinloadbalancer.next_server());

    println!("Least Connection Load Balancer starts");

    let mut leastconnectionloadbalancer = LeastConnectionLoadBalancer::new();

    leastconnectionloadbalancer.add_server("Server1".to_string());
    leastconnectionloadbalancer.add_server("Server2".to_string());
    leastconnectionloadbalancer.add_server("Server3".to_string());

    println!("{:?}", leastconnectionloadbalancer.next_server());
    println!("{:?}", leastconnectionloadbalancer.next_server());
    println!("{:?}", leastconnectionloadbalancer.next_server());

    println!("Weighted Round Robin Load Balancer Starts ");

    let mut weightedroundrobinbalancer = WeightedRoundRobinLoadBalancer::new();

    weightedroundrobinbalancer.add_server("Server1".to_string(), 3);
    weightedroundrobinbalancer.add_server("Server2".to_string(), 1);
    weightedroundrobinbalancer.add_server("Server3".to_string(), 2);

    for i in 0..10 {
        if let Some(server) = weightedroundrobinbalancer.next_server() {
            println!("Request {} directed to: {}", i + 1, server);
        }
    }

    println!("Ip Hash Load Balancer Starts ");
    let mut iphashloadbalancer = IpHashLoadBalancer::new();

    // Add servers with their IP addresses
    iphashloadbalancer.add_server("192.168.1.1".to_string(), "Server1".to_string());
    iphashloadbalancer.add_server("192.168.1.2".to_string(), "Server2".to_string());

    // Retrieve servers based on IP address
    let ip = "192.168.1.1";
    match iphashloadbalancer.get_server(ip) {
        Some(server) => println!("IP {} is directed to: {}", ip, server),
        None => println!("No server found for IP {}", ip),
    }

    println!("Dynamic Load Balancer Starts ");

    let mut dynamicloadbalancer = DynamicLoadBalancer::new();

    dynamicloadbalancer.add_server("Server1".to_string());
    dynamicloadbalancer.add_server("Server2".to_string());
    dynamicloadbalancer.add_server("Server3".to_string());

    let mut lcgrng = LcgRng::new();

    for i in 0..10 {
        let load = lcgrng.lcg_rand();

        if let Some(server) = dynamicloadbalancer.next_server(load) {
            println!("Request {} directed to: {}", i + 1, server);
        }
    }
}
