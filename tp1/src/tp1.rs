use anyhow::{Context, Result};
use std::io::{self, BufRead, BufReader, Write};
use std::fs::{self, File};
use std::error::Error;

/// Représente un produit avec nom et quantité.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Produit {
    pub nom: String,
    pub quantite: u32,
}

/// Inventaire contenant plusieurs produits.
#[derive(Debug, Default)]
pub struct Inventory {
    pub items: Vec<Produit>,
}

impl Inventory {
    /// Crée un nouvel inventaire vide.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Ajoute ou met à jour un produit.
    pub fn add_or_update_product(&mut self, nom: String, quantite: u32) {
        if let Some(p) = self.items.iter_mut().find(|p| p.nom == nom) {
            p.quantite = quantite;
        } else {
            self.items.push(Produit { nom, quantite });
        }
    }

    /// Supprime un produit par nom. Retourne true si supprimé.
    pub fn remove_product(&mut self, nom: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|p| p.nom == nom) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    /// Affiche la liste des produits.
    pub fn list_products(&self) {
        if self.items.is_empty() {
            println!("Aucun produit dans l'inventaire.");
        } else {
            println!("Inventaire:");
            for p in &self.items {
                println!("- {} : {}", p.nom, p.quantite);
            }
        }
    }

    /// Sauvegarde l'inventaire dans un fichier texte ("nom,quantité" par ligne).
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let mut file = File::create(path)
            .with_context(|| format!("Impossible de créer le fichier {}", path))?;
        for p in &self.items {
            writeln!(file, "{},{}", p.nom, p.quantite)
                .with_context(|| "Erreur lors de l'écriture du fichier")?;
        }
        Ok(())
    }

    /// Charge l'inventaire depuis un fichier texte.
    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)
            .with_context(|| format!("Impossible d'ouvrir le fichier {}", path))?;
        let reader = BufReader::new(file);
        self.items.clear();
        for line in reader.lines() {
            let line = line.with_context(|| "Erreur lors de la lecture d'une ligne")?;
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                if let Ok(q) = parts[1].parse::<u32>() {
                    self.items.push(Produit {
                        nom: parts[0].to_string(),
                        quantite: q,
                    });
                }
            }
        }
        Ok(())
    }

    // // Sérialise la collection de produits en JSON et l'écrit dans un fichier
    // fn sauvegarder(&self) -> Result<(), Box<dyn Error>> {
    //     let json = serde_json::to_string(&self.items)?;
    //     let mut file = File::create("inventaire.json")?;
    //     file.write_all(json.as_bytes())?;
    //     Ok(())
    // }

    // // Charge la bibliothèque depuis le fichier JSON ou crée une nouvelle si le fichier n'existe pas
    // fn charger() -> Result<Self, Box<dyn Error>> {
    //     match fs::read_to_string("inventaire.json") {
    //         Ok(contenu) => {
    //             let items: Vec<Produit> = serde_json::from_str(&contenu)?;
    //             Ok(Self { items })
    //         }
    //         Err(_) => Ok(Self::new())
    //     }
    // }
}

/// Lit une ligne depuis stdin avec un prompt.
pub fn read_line(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    print!("{}: ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input)
}