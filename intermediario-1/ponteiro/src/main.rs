fn main() {
    let mut a: u8 = 10;
    let b: &mut u8 = &mut a;
    *b = 20;
    println!("Valor de a: {}", a);
}
