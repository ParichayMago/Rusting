mod transportation;

fn main() {
    
    eat_at_restaurant();

    println!("Deciding on transportation");


    transportation::transport::public_transport::metro();
    crate::payment_method::cash(12)

}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist")
        }
    }
}

pub fn eat_at_restaurant() {
    println!("destincation decided");
    crate::front_of_house::hosting::add_to_waitlist();
}

mod payment_method {
    mod upi {
        fn paytm() {
            println!("rupay prapt houa")
        }
        fn phone_pay(){
            println!("davio aur sujano payment prapt houa")
        }
    }
    pub fn cash(money: u32){
        println!("{} payed with cash", money)
    }
}