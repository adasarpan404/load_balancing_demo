use leastconnection::LeastConnectionLoadBalancer;
use roundrobin::RoundRobinLoadBalancer;

mod leastconnection;
mod roundrobin;
fn main() {
    println!("Round Robin Load Balancer");

    let mut roundrobinloadbalancer = RoundRobinLoadBalancer::new();

    roundrobinloadbalancer.add_server("Server1".to_string());
    roundrobinloadbalancer.add_server("Server2".to_string());
    roundrobinloadbalancer.add_server("Server3".to_string());

    println!("{:?}", roundrobinloadbalancer.next_server());
    println!("{:?}", roundrobinloadbalancer.next_server());
    println!("{:?}", roundrobinloadbalancer.next_server());

    println!("Least Connection Load Balancer");

    let mut leastconnectionloadbalancer = LeastConnectionLoadBalancer::new();

    leastconnectionloadbalancer.add_server("Server1".to_string());
    leastconnectionloadbalancer.add_server("Server2".to_string());
    leastconnectionloadbalancer.add_server("Server3".to_string());

    println!("{:?}", leastconnectionloadbalancer.next_server());
    println!("{:?}", leastconnectionloadbalancer.next_server());
    println!("{:?}", leastconnectionloadbalancer.next_server());
}
