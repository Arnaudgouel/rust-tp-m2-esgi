mod tp1;

use anyhow::Result;
use tp1::Inventory;

fn main() -> Result<()> {
    let mut inventory = Inventory::new();
    // Charge l'inventaire existant si le fichier existe
    if let Err(e) = inventory.load_from_file("inventory.txt") {
        eprintln!("Impossible de charger l'inventaire: {}", e);
    }

    loop {
        println!("\n=== Menu Inventaire ===");
        println!("1. Ajouter ou mettre à jour un produit");
        println!("2. Supprimer un produit");
        println!("3. Afficher l'inventaire");
        println!("4. Sauvegarder et quitter");
        println!("5. Quitter sans sauvegarder");

        let choice = tp1::read_line("Choix")?;
        match choice.trim() {
            "1" => {
                let nom = tp1::read_line("Nom du produit")?;
                let quantite_str = tp1::read_line("Quantité")?;
                match quantite_str.trim().parse::<u32>() {
                    Ok(q) => {
                        inventory.add_or_update_product(nom.trim().to_string(), q);
                        println!("Produit ajouté/mis à jour.");
                    }
                    Err(_) => println!("Quantité invalide."),
                }
            }
            "2" => {
                let nom = tp1::read_line("Nom du produit à supprimer")?;
                if inventory.remove_product(nom.trim()) {
                    println!("Produit supprimé.");
                } else {
                    println!("Produit non trouvé.");
                }
            }
            "3" => inventory.list_products(),
            "4" => {
                inventory.save_to_file("inventory.txt")?;
                println!("Inventaire sauvegardé. Au revoir !");
                break;
            }
            "5" => {
                println!("Au revoir sans sauvegarder !");
                break;
            }
            _ => println!("Option invalide."),
        }
    }

    Ok(())
}