use mysql::PooledConn;
use mysql::prelude::Queryable;
use crate::models::user::User;
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use mysql::params;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("Bcrypt error: {0}")]
    BcryptError(#[from] BcryptError),
    #[error("MySQL error: {0}")]
    MySQLError(#[from] mysql::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/**
 * Vérifie si un mot de passe est valide.
 *
 * @param password Le mot de passe à vérifier.
 * @return Un booléen indiquant si le mot de passe est valide.
 */
fn is_valid_password(password: &str) -> bool {
    if password.is_empty() || password.len() < 8 || password.len() > 128 {
        return false;
    }
    password.chars().all(|c| c.is_ascii() && !c.is_control())
}

/**
 * Nettoie le mot de passe en supprimant les caractères non désirés.
 *
 * @param password Le mot de passe à nettoyer.
 * @return Le mot de passe nettoyé.
 */
fn clean_password(password: &str) -> String {
    password.chars().filter(|&c| c.is_ascii() && !c.is_control()).collect()
}

/**
 * Insère un nouvel utilisateur dans la base de données.
 *
 * @param conn La connexion à la base de données.
 * @param name Le nom de l'utilisateur.
 * @param email L'email de l'utilisateur.
 * @param password Le mot de passe de l'utilisateur.
 * @return Un résultat indiquant le succès ou l'échec de l'opération.
 */
pub fn create_user(conn: &mut PooledConn, name: &str, email: &str, password: &str) -> Result<(), UserError> {
    let cleaned_password = clean_password(password);

    if !is_valid_password(&cleaned_password) {
        return Err(UserError::InvalidInput("Invalid password format".to_string()));
    }

    let hashed_password = hash(&cleaned_password, DEFAULT_COST)?;

    conn.exec_drop(
        "INSERT INTO users (name, email, password) VALUES (:name, :email, :password)",
        params! {
            "name" => name,
            "email" => email,
            "password" => hashed_password,
        },
    )?;
    Ok(())
}



/**
 * Récupère un utilisateur par son ID.
 *
 * @param conn La connexion à la base de données.
 * @param user_id L'ID de l'utilisateur.
 * @return Une `Option` contenant l'utilisateur ou `None` si l'utilisateur n'est pas trouvé.
 */
pub fn get_user_by_id(conn: &mut PooledConn, user_id: i32) -> Option<User> {
    let sql = "SELECT id, name, email, password, created_at FROM users WHERE id = :id";

    let row: mysql::Row = match conn.exec_first(sql, params!{"id" => user_id}).unwrap() {
        Some(row) => row,
        None => return None,
    };

    Some(User::from_row(row))
}

/**
 * Met à jour les informations d'un utilisateur.
 *
 * @param conn La connexion à la base de données.
 * @param user_id L'ID de l'utilisateur.
 * @param name Le nouveau nom de l'utilisateur.
 * @param email Le nouvel email de l'utilisateur.
 * @return Un `Result` indiquant le succès ou l'échec de l'opération.
 */
pub fn update_user(conn: &mut PooledConn, user_id: i32, name: &str, email: &str) -> Result<(), mysql::Error> {
    conn.exec_drop(
        "UPDATE users SET name = :name, email = :email WHERE id = :id",
        params! {
            "id" => user_id,
            "name" => name,
            "email" => email,
        },
    )?;
    Ok(())
}

/**
 * Supprime un utilisateur par son ID.
 *
 * @param conn La connexion à la base de données.
 * @param user_id L'ID de l'utilisateur.
 * @return Un `Result` indiquant le succès ou l'échec de l'opération.
 */
pub fn delete_user(conn: &mut PooledConn, user_id: i32) -> Result<(), mysql::Error> {
    conn.exec_drop(
        "DELETE FROM users WHERE id = :id",
        params! {
            "id" => user_id,
        },
    )?;
    Ok(())
}

/**
 * Vérifie les informations d'identification de l'utilisateur.
 *
 * @param conn La connexion à la base de données.
 * @param email L'email de l'utilisateur.
 * @param password Le mot de passe de l'utilisateur.
 * @return Une `Option` contenant l'utilisateur si les informations sont correctes, sinon `None`.
 */
pub fn authenticate_user(conn: &mut PooledConn, email: &str, password: &str) -> Option<User> {
    let sql = "SELECT id, name, email, password, created_at FROM users WHERE email = :email";

    let row: mysql::Row = match conn.exec_first(sql, params!{"email" => email}).unwrap() {
        Some(row) => row,
        None => return None,
    };

    let user = User::from_row(row);

    if verify(password, &user.password).unwrap() {
        Some(user)
    } else {
        None
    }
}