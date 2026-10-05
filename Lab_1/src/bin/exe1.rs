fn prim(x: i32) -> bool {
    let mut count = 0;
    for i in 1..=x {
        if x % i == 0 {
            count += 1;
        }
    }
    if count == 2 { true } else { false }
}

fn main() {
    for x in 0..=100 {
        if x == 0 || x == 1 {
            continue;
        }
        if prim(x) {
            println!("{x} is prime");
        }
    }
}
