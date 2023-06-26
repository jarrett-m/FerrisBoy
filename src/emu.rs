use std::thread;
use std::time::Duration;
mod cart;


pub struct Emulator <'a>{
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
    cart: cart::Cart<'a>
}

impl Emulator<'_>{

    pub fn new() -> Emulator<'static>{
        return Emulator {paused: false, running: false, ticks: 0, cart: cart::Cart::new()};

    }

    pub fn run(&mut self, cart_name: &str) -> Result<(), String>{

        self.cart.load_cart(cart_name);

        self.running = true;
        self.paused = false;
        self.ticks = 0;

        self.cart.print_data();

        while self.running{
            if self.paused{
                Self::delay(10);
                continue;
            }
            self.ticks += 1;
        }

        println!("Cart Loaded!");

        /*
        match cpu::cpu_step(){
            Err(error: String) => {
                return Err(error);
            }
            Ok(()) ={}
        } 
        */

        return Ok(())
    }
    
    fn delay(ms: u64){
        thread::sleep(Duration::from_millis(ms));
    }


}


