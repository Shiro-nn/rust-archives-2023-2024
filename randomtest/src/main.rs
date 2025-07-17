use rand::Rng;

fn main() {
    let mut i = 100;
    while i > 0 {
        print();
        i -= 1;
    }
}

fn print() {
    let random_number = rand::thread_rng().gen_range(0..100);
    println!("rand: {random_number}");
}