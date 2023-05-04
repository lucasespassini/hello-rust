use rand::Rng;

fn main() {
    let mut array: [i16; 10] = [0; 10];

    for i in 0..array.len() {
        array[i] = i as i16;
    }

    println!(" R: {}\n", ex1_soma_array(array));

    //////////////////////////////////////////////////////////////////

    let mut notas: [u8; 4] = [0; 4];

    notas[0] = 8;
    notas[1] = 7;
    notas[2] = 5;
    notas[3] = 9;

    println!(" R: {}\n", ex2_ta_aprovado(notas));

    //////////////////////////////////////////////////////////////////

    ex3_multiplo_2();
}

fn ex1_soma_array(array: [i16; 10]) -> i16 {
    println!("1 - Faça um programa que tenha uma função que recebe um array de inteiros com sinal (aceite números negativos) e devolva a soma dos valores deste array e exiba no console.");

    let mut soma: i16 = 0;

    for v in array {
        soma += v;
    }

    soma
}

fn ex2_ta_aprovado(notas: [u8; 4]) -> bool {
    println!("2 - Faça um programa que calcule a média entre quatro notas e informe se foi aprovado ou não e a média, para ser aprovado a média deve ser maior ou igual a 7.");

    let mut soma: u8 = 0;

    for nota in notas {
        soma += nota;
    }

    let media = soma / notas.len() as u8;

    if media >= 7 {
        return true;
    }

    false
}

fn ex3_multiplo_2() {
    println!("3 - Faça um programa que percorra um vetor com valores inteiros e verifique quais múltiplos de 2.");

    let mut rng = rand::thread_rng();
    let mut array: [u16; 20] = [0; 20];

    for i in 0..array.len() {
        array[i] = rng.gen::<u16>();
    }

    array.map(|v| {
        if v % 2 == 0 {
            println!("{v}")
        }
    });
}
