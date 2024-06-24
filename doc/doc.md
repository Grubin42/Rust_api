
# Documentation du Projet Backend

## Structure du Projet

```
/backend
|-- /src
|   |-- /db
|   |   |-- mod.rs
|   |   |-- connection.rs
|   |   |-- user.rs
|   |   |-- game.rs
|   |-- /handlers
|   |   |-- mod.rs
|   |   |-- user.rs
|   |   |-- common.rs
|   |   |-- game.rs
|   |-- /models
|   |   |-- mod.rs
|   |   |-- user.rs
|   |   |-- game.rs
|   |-- /routes
|   |   |-- mod.rs
|   |   |-- user.rs
|   |   |-- game.rs
|   |-- main.rs
|-- Cargo.toml
|-- .env
```

## Description des Fichiers

### `src/main.rs`

Le point d'entrée principal de l'application.

- Charge les variables d'environnement à partir du fichier `.env`.
- Crée un `TcpListener` pour écouter les connexions entrantes sur le port 8000.
- Démarre le serveur en appelant la fonction `run` dans le module `routes`.

### `src/db/mod.rs`

Module qui regroupe les sous-modules `connection`, `user`.

### `src/db/connection.rs`

Contient la fonction `establish_connection` pour établir une connexion à la base de données.

### `src/db/user.rs`

Contient la fonction `fetch_users` pour récupérer tous les utilisateurs de la base de données.

### `src/handlers/mod.rs`

Module qui regroupe les sous-modules `user`, `common`.

### `src/handlers/user.rs`

Contient la fonction `handle_get_users` pour gérer les requêtes GET des utilisateurs.

### `src/handlers/common.rs`

Contient les fonctions communes de gestion des requêtes :

- `handle_options_request` : Gère les requêtes OPTIONS pour les utilisateurs.
- `handle_request` : Gère les requêtes non reconnues (404 Not Found).

### `src/models/mod.rs`

Module qui regroupe les sous-modules `user`.

### `src/models/user.rs`

Définit la structure `User` représentant un utilisateur.

### `src/routes/mod.rs`

Module qui regroupe les sous-modules `user`.

- Contient la fonction `run` qui démarre le serveur et délègue les requêtes aux modules appropriés en fonction de l'URL.

### `src/routes/user.rs`

Contient la fonction `route_user_requests` pour router les requêtes liées aux utilisateurs.
