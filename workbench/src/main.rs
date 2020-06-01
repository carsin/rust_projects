use rand::Rng;

fn main() {
    let num = gen_number();

    println!("{}", num);
}

fn gen_number() -> u32 {
    let x = rand::thread_rng().gen_range(0, 5);

    x
}
