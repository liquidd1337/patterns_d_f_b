use patterns::factory::transport::Transport;
use patterns::factory::TransportFactory;
fn main() {
    let car_factory = patterns::factory::transport_car();
    let bicycle_factory = patterns::factory::transport_bicycle();
    let airplane_factory = patterns::factory::transport_airplane();
    print_delivery(car_factory);
    print_delivery(bicycle_factory);
    print_delivery(airplane_factory);
}

fn print_delivery(factory: impl TransportFactory) {
    let transport = factory.create_transport();
    println!("{}", transport.deliver())
}
