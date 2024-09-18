use leastconnection::LeastConnectionLoadBalancer;
use roundrobin::RoundRobinLoadBalancer;
use weightedroundrobin::WeightedRoundRobinLoadBalancer;

mod iphash;
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
}
