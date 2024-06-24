use mysql::Row;
use serde::{Serialize, Deserialize};

/**
 * Représente un utilisateur dans la base de données.
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: String,
}

impl User {
    /**
     * Crée une instance de `User` à partir d'une ligne de la base de données.
     *
     * @param row Une ligne de la base de données contenant les données de l'utilisateur.
     * @return Une instance de `User`.
     */
    pub fn from_row(mut row: Row) -> Self {
        User {
            id: row.take("id").unwrap(),
            name: row.take("name").unwrap(),
            email: row.take("email").unwrap(),
            password: row.take("password").unwrap(),
            created_at: row.take("created_at").unwrap(),
        }
    }
}