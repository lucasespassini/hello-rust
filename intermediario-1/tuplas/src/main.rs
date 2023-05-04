fn main() {
    let tuple: (u8, i32, String, char) = (10, 25, String::from("Rust4Noobs"), 'a');
    println!("Valor i32: {}", tuple.1);
}

// fn main() {
//     let texto1: String = String::from("4Noobs");
//     let texto2 = String::from("4Noobs");

//     let (mut devolve1, mut devolve2) = printa_duas_strings(texto1, texto2);

//     devolve1.push_str(" qualquercoisasópraterexemplo");
//     devolve2.push_str(" sérionaotiveideianenhuma");

//     println!("{}", devolve1);
//     println!("{}", devolve2);
// }

// fn printa_duas_strings(texto1: String, texto2: String) -> (String, String) {
//     println!("{}", texto1);
//     println!("{}", texto2);
//     (texto1, texto2)
// }
