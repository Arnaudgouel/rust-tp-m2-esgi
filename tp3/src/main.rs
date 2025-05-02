use rand::Rng;
use std::io;

fn main() {
  println!("Bienvenue dans le jeu du nombre à deviner !");
  println!("Je choisis un nombre entre 1 et 100. À vous de deviner...");

  let secret = rand::thread_rng().gen_range(1..=100);

  loop {
      println!("Veuillez entrer votre proposition :");

      let mut guess = String::new();
      io::stdin()
          .read_line(&mut guess)
          .expect("Échec de la lecture de l'entrée");

      let guess: u32 = match guess.trim().parse() {
          Ok(num) if num >= 1 && num <= 100 => num,
          _ => {
              println!("Entrée invalide. Veuillez entrer un nombre entre 1 et 100.");
              continue;
          }
      };

      if guess < secret {
          println!("Trop petit !");
      } else if guess > secret {
          println!("Trop grand !");
      } else {
          println!("Bravo ! Vous avez deviné le nombre {} !", secret);
          break;
      }
  }
}