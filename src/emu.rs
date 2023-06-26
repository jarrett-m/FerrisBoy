use std::thread;
use std::time::Duration;
mod cart;

pub struct Emulator {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
    cart: cart::Cart,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            paused: false,
            running: false,
            ticks: 0,
            cart: cart::Cart::new(),
        }
    }

    pub fn run(&mut self, cart_name: String) -> Result<(), String> {
        let _ = self.cart.load_cart(cart_name);

        self.running = true;
        self.paused = false;
        self.ticks = 0;

        self.cart.print_data();

        while self.running {
            if self.paused {
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

        Ok(())
    }

    fn delay(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }
}
