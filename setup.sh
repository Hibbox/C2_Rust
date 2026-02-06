#!/bin/bash

echo "=== ZARC2 Setup ==="

# Vérifier si .env existe ?
if [ -f .env ]; then
    echo ".env already exists. Skipping..."
    exit 0
fi

# Demander le mot de passe ou en générer un
read -p "Enter a database password (leave empty to auto-generate): " password

if [ -z "$password" ]; then
    password=$(openssl rand -base64 16)
    echo "Generated password: $password"
fi

# Créer le fichier .env
echo "DB_PASSWORD=$password" > .env

# Créer le fichier db_secret.txt
echo -n "$password" > db_secret.txt

echo ""
echo "✅ Setup complete!"
echo "Run: docker compose up --build"