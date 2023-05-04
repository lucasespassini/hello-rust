fn main() {
    let mut string = String::new();
    println!("Entre com o seu texto: ");
    std::io::stdin().read_line(&mut string).unwrap();
    println!("Você digitou {}", string);
}
