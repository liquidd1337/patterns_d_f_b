use crate::factory::transport::{Airplane, Bicycle, Car, Transport};

pub fn transport_car() -> impl TransportFactory {
    CarFactory
}

pub fn transport_bicycle() -> impl TransportFactory {
    BicycleFactory
}

pub fn transport_airplane() -> impl TransportFactory {
    AirplaneFactory
}

pub trait TransportFactory {
    type TransportDelivery: Transport;

    fn create_transport(&self) -> Self::TransportDelivery;
}

struct CarFactory;

impl TransportFactory for CarFactory {
    type TransportDelivery = Car;
    fn create_transport(&self) -> Self::TransportDelivery {
        Car
    }
}

struct BicycleFactory;

impl TransportFactory for BicycleFactory {
    type TransportDelivery = Bicycle;
    fn create_transport(&self) -> Self::TransportDelivery {
        Bicycle
    }
}

pub struct AirplaneFactory;

impl TransportFactory for AirplaneFactory {
    type TransportDelivery = Airplane;
    fn create_transport(&self) -> Self::TransportDelivery {
        Airplane
    }
}
pub mod transport {
    pub trait Transport {
        fn deliver(&self) -> String;
    }

    pub struct Car;

    impl Transport for Car {
        fn deliver(&self) -> String {
            "Delivering by car".into()
        }
    }

    pub struct Bicycle;

    impl Transport for Bicycle {
        fn deliver(&self) -> String {
            "Delivering by bicycle".into()
        }
    }

    pub struct Airplane;

    impl Transport for Airplane {
        fn deliver(&self) -> String {
            "Delivering by airplane".into()
        }
    }
}
