# Étape 1 : build de l'application
FROM rust:1.74 as builder

WORKDIR /app

# Copier le manifest d'abord pour utiliser le cache Docker plus intelligemment
COPY Cargo.toml .
COPY src ./src

# Build en release
# RUN cargo build

# Commande par défaut
# CMD ["./target/release/docker_rust_project"]
CMD ["tail", "-f", "/dev/null"]