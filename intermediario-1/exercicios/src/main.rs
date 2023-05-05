fn main() {
    ex1_grau_do_numero();
    ex2_add_string();
}

fn ex1_grau_do_numero() {
    println!("1 - Faça um programa que receba um número de no máximo 255 como entrada do usuário e informe em qual grau o numero esta encaixado, conforme as especificações:
1° grau entre 0 e 50
2° grau entre 51 e 120
3° grau entre 121 e 200
4° grau acima de 200\n");

    let mut string = String::new();
    println!("Escolha seu número: ");
    std::io::stdin().read_line(&mut string).unwrap();
    let num: u8 = string.trim().parse().unwrap();

    if num <= 50 {
        println!("1° grau");
    } else if num <= 120 {
        println!("2° grau");
    } else if num <= 200 {
        println!("3° grau");
    } else {
        println!("4° grau");
    }
}

fn ex2_add_string() {
    println!("\n2 - Faça um programa que tenha a String 'Rust4Noobs' e receba do usuário outra String que sera adicionada a String já existente. E escreva no console.");

    let mut text: String = String::from("Rust4Noobs");
    let mut string = String::new();
    println!("O que será adicionado a string {}? ", text);
    std::io::stdin().read_line(&mut string).unwrap();
    text.push_str(&string);
    println!("{text}")
}
