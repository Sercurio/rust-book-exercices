use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let nombre_secret = rand::thread_rng().gen_range(0..100);

    // println!("Le nombre secret est: {}", nombre_secret);

    println!("Il faut deviner le nombre !");

    loop {
        let mut supposition = String::new();
        println!("Veuillez rentrer un nombre: ");

        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };

        match supposition.cmp(&nombre_secret) {
            Ordering::Equal => {
                println!("Bravo, vous avez trouvé !");
                break;
            }
            Ordering::Less => println!("Plus grand"),

            Ordering::Greater => println!("Plus petit"),
        }
    }
}
