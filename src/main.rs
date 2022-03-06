extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o número!"); // exibe no terminal o texto

    let numero_secreto = rand::thread_rng().gen_range(1,101); // atribui a "numero_secreto", números gerados de 1-101

    loop { // loop para que a pessoa possa ter múltiplas tentativas
        println!("Digite o seu palpite!"); // exibe no terminal o texto

        let mut palpite = String::new(); // cria uma variável "palpite" que espera um valor do tipo String
        io::stdin().read_line(&mut palpite) // Lê o que foi digitado no terminal e atribui a "palpite"
            .expect("Falha ao ler entrada"); // se houver algum erro ele sai do programa com essa mensagem de erro

        let palpite: u32 = match palpite.trim().parse() { // converte o valor de "palpite" para um número inteiro
            Ok(num) => num, // se for um número, Ok
            Err(_) => continue, // se não for, Continue, para que possa ser digitado novamente.
        };

        println!("Você disse: {}", palpite); // exibe no terminal o texto

        match palpite.cmp(&numero_secreto){ // compara o valor de "palpite" com o valor de "numero_secreto", e se for Equal, sai do programa
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater =>  println!("Muito alto!"),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }
}
