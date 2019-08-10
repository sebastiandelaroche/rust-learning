extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let _b: bool = rng.gen();
}
