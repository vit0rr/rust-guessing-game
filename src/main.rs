extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o número!");
    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    let mut contador = 0;

    loop {
        contador += 1;
        println!("tentativa: {:?}", contador);
        println!("Digite o seu palpite!");
        let mut palpite = String::new();
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Acertou! Você jogou por {} vezes", contador);
                break;
            }
        }
    }
}
