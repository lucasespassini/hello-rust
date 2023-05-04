fn main() {
    let mut meu_texto = String::from("Lucas");
    printa_string(&meu_texto);
    adiciona_texto(&mut meu_texto);
    println!("{}", meu_texto);
}

fn printa_string(string: &String) {
    println!("{}", string);
}

fn adiciona_texto(string: &mut String) {
    string.push_str(" Mendes");
}