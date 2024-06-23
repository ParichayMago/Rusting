pub mod transport {

    pub mod car_service {

        pub fn uber() {
            println!("calling an uber")
        }

        pub fn ola() {
            println!("booking ola")
        }
    }

    pub mod public_transport {

        pub fn metro() {
            println!("buying metro ticket")
        }
    }
}
