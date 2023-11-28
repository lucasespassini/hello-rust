use chrono::prelude::*;

struct Cliente {
    nome: String,
    ano_de_nascimento: u16,
    documento: String,
}

impl Cliente {
    fn new(nome: String, ano_de_nascimento: u16, documento: String) -> Self {
        Self {
            nome,
            ano_de_nascimento,
            documento,
        }
    }

    fn idade(&self) {
        self.ano_de_nascimento;
         // Obtendo a data e hora local atual
        let local: DateTime<Local> = Local::now();

        // Formatando a data e hora como uma string
        let formatted_date = local.format("%Y-%m-%d %H:%M:%S").to_string();

        // Exibindo a string formatada
        println!("Data e hora atual: {}", formatted_date);
    }
}

fn main() {
    let cliente = Cliente::new(String::from("Lucas"), 1999, String::from("18134482724"));

    cliente.idade();
}
