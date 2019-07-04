fn main() {
    let num = 10;
    println!("{} plus one is {}!", num, add_one::add_one(num));
    println!("random is {}", add_one::get_rand());
}

