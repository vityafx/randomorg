extern crate randomorg;

fn main() {
    use randomorg::Random;

    let r = Random::new("API KEY HERE").unwrap();
    println!("Result: {:?}", r.generate_integers(-100, 100, 15, true));
}
