use std::io;
use std::io::Write;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  println!("\n=== Générateur de mot de passe ===");
    let length_str = read_line("Longueur souhaitée")?;
    let length = length_str.trim().parse::<usize>().unwrap_or(12);
    let exclude = read_line("Caractères à exclure (ex: l1O0) ou entrer pour aucun")?;
    let exclude = if exclude.trim().is_empty() {
        None
    } else {
        Some(exclude.trim().to_string())
    };

    let password = generate_password(length, exclude);
    println!("Mot de passe généré : {}", password);
    Ok(())
}


/// Lit une ligne depuis stdin avec un prompt.
pub fn read_line(prompt: &str) -> io::Result<String> {
  let mut input = String::new();
  print!("{}: ", prompt);
  io::stdout().flush()?;
  io::stdin().read_line(&mut input)?;
  Ok(input)
}

use rand::{distributions::Alphanumeric, Rng};

/// Génère un mot de passe de longueur `length`, en excluant éventuellement certains caractères.
pub fn generate_password(length: usize, exclude: Option<String>) -> String {
    // Construit un jeu de caractères de base: lettres, chiffres et spéciaux
    let mut charset: Vec<char> = ('!'..='~').collect(); // ASCII imprimables
    // Applique l'exclusion
    if let Some(ex) = exclude {
        charset.retain(|c| !ex.contains(*c));
    }
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect()
}

