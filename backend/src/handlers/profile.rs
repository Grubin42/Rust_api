use std::io::Write;
use crate::db::user::{get_user_by_id, update_user, delete_user};
use crate::db::connection::get_conn;

/**
 * Simule l'extraction de l'ID utilisateur à partir d'un token.
 *
 * @param buffer Le tampon contenant les données de la requête.
 * @return Une Option contenant l'ID utilisateur, ou None si le token est invalide.
 */
fn extract_user_id_from_token(buffer: &[u8]) -> Option<i32> {
    // Simule l'extraction d'un token à partir des en-têtes HTTP
    let headers = std::str::from_utf8(buffer).unwrap();
    if headers.contains("Authorization: Bearer valid_token") {
        // Simule un token valide qui contient l'ID utilisateur 1
        Some(1)
    } else {
        None
    }
}

/**
 * Gère la requête pour afficher le profil de l'utilisateur.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
pub fn handle_profile<W: Write>(stream: &mut W, buffer: &[u8]) {
    let user_id = extract_user_id_from_token(buffer).unwrap_or_else(|| {
        // Répondre avec une erreur si l'utilisateur n'est pas authentifié
        let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Unauthorized\"}";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        0 // ID utilisateur par défaut pour arrêter le traitement
    });

    if user_id == 0 {
        return;
    }

    match get_user_by_id(&mut get_conn(), user_id) {
        Some(user) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                serde_json::to_string(&user).unwrap()
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
        None => {
            let response = "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\n\r\n{\"message\": \"User not found\"}";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    stream.flush().unwrap();
}

/**
 * Gère la requête pour éditer le profil de l'utilisateur.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
pub fn handle_edit_profile<W: Write>(stream: &mut W, buffer: &[u8]) {
    let user_id = extract_user_id_from_token(buffer).unwrap_or_else(|| {
        // Répondre avec une erreur si l'utilisateur n'est pas authentifié
        let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Unauthorized\"}";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        0 // ID utilisateur par défaut pour arrêter le traitement
    });

    if user_id == 0 {
        return;
    }

    // Extraction des données de la requête
    let request_body = std::str::from_utf8(buffer).unwrap();
    let params: Vec<&str> = request_body.split('&').collect();
    let mut name = "";
    let mut email = "";

    for param in params {
        let pair: Vec<&str> = param.split('=').collect();
        match pair[0] {
            "name" => name = pair[1],
            "email" => email = pair[1],
            _ => {}
        }
    }

    match update_user(&mut get_conn(), user_id, name, email) {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Profile updated successfully\"}";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {
            let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Failed to update profile\"}";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    stream.flush().unwrap();
}

/**
 * Gère la requête pour supprimer le profil de l'utilisateur.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
pub fn handle_delete_profile<W: Write>(stream: &mut W, buffer: &[u8]) {
    let user_id = extract_user_id_from_token(buffer).unwrap_or_else(|| {
        // Répondre avec une erreur si l'utilisateur n'est pas authentifié
        let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Unauthorized\"}";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        0 // ID utilisateur par défaut pour arrêter le traitement
    });

    if user_id == 0 {
        return;
    }

    match delete_user(&mut get_conn(), user_id) {
        Ok(_) => {
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Profile deleted successfully\"}";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(_) => {
            let response = "HTTP/1.1 500 Internal Server Error\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Failed to delete profile\"}";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    stream.flush().unwrap();
}