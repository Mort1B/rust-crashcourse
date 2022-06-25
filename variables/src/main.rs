const STARTING_MISSILES: i32 = 8;
const READY_AMOUT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUT);
    println!("firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
