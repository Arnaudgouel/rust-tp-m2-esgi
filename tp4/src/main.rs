use anyhow::{Context, Result};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::io::Read;

#[derive(Debug, Deserialize)]
struct SmtpConfig {
    smtp_server: String,
    smtp_port: u16,
    username: String,
    password: String,
}

impl SmtpConfig {
    /// Charge la configuration SMTP depuis le fichier .env
    fn from_env_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Impossible de lire le fichier {}", path))?;
        let mut cfg = SmtpConfig {
            smtp_server: String::new(),
            smtp_port: 587,
            username: String::new(),
            password: String::new(),
        };
        for line in content.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0] {
                    "SMTP_SERVER" => cfg.smtp_server = parts[1].to_string(),
                    "SMTP_PORT" => cfg.smtp_port = parts[1].parse().unwrap_or(587),
                    "SMTP_USERNAME" => cfg.username = parts[1].to_string(),
                    "SMTP_PASSWORD" => cfg.password = parts[1].to_string(),
                    _ => (),
                }
            }
        }
        Ok(cfg)
    }
}

fn main() -> Result<()> {
    // Charge la configuration
    let config = SmtpConfig::from_env_file(".env")?;

    println!("=== Envoi d'e-mail ===");
    let from = read_line("Adresse expéditeur (from)")?;
    let to = read_line("Adresse destinataire (to)")?;
    let subject = read_line("Sujet")?;
    let body = read_multiline("Corps du message (CTRL+D pour terminer)")?;

    // Construit le message
    let email = Message::builder()
        .from(from.parse().context("Adresse expéditeur invalide")?)
        .reply_to(from.parse().unwrap())
        .to(to.parse().context("Adresse destinataire invalide")?)
        .subject(subject)
        .body(body)
        .context("Erreur lors de la construction du message")?;

    // Transport SMTP
    let creds = Credentials::new(config.username, config.password);
    let mailer = SmtpTransport::relay(&config.smtp_server)?
        .port(config.smtp_port)
        .credentials(creds)
        .build();

    // Envoi
    match mailer.send(&email) {
        Ok(_) => println!("E-mail envoyé avec succès !"),
        Err(e) => eprintln!("Erreur lors de l'envoi de l'e-mail: {}", e),
    }

    Ok(())
}

/// Lit une ligne avec un prompt
fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Lit plusieurs lignes jusqu'à EOF (Ctrl+D)
fn read_multiline(prompt: &str) -> io::Result<String> {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}
