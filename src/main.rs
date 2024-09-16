use roundrobin::RoundRobinLoadBalancer;

mod roundrobin;
fn main() {
    let mut roundrobinloadbalancer = RoundRobinLoadBalancer::new();

    roundrobinloadbalancer.add_server("Server1".to_string());
    roundrobinloadbalancer.add_server("Server2".to_string());
    roundrobinloadbalancer.add_server("Server3".to_string());

    println!("{:?}", roundrobinloadbalancer.next_server());
    println!("{:?}", roundrobinloadbalancer.next_server());
    println!("{:?}", roundrobinloadbalancer.next_server());
}
