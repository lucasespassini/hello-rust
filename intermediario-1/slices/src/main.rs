fn main() {
    let texto: String = String::from("Rust 4 Noobs");
    let slice = primeira_palavra(&texto);
    println!("{}", slice);
}

fn primeira_palavra(texto: &String) -> &str {
    let bytes = texto.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //b' ' transforma o espaço em um byte
            return &texto[0..i];
        }
    }

    &texto[..] //caso não ache retorna a string inteira como um slice
}
