use std::collections::HashMap;
use std::io::Write;
use crate::db::user::{create_user, authenticate_user};
use crate::db::connection::get_conn;
use std::str;
use form_urlencoded;

pub fn handle_register<W: Write>(stream: &mut W, buffer: &[u8]) {
    println!("Handling register request");
    let request = str::from_utf8(buffer).unwrap_or_default();
    println!("Request body: {:?}", request);

    // Extract body from request
    if let Some(body) = request.split("\r\n\r\n").nth(1) {
        let form_data: HashMap<_, _> = form_urlencoded::parse(body.as_bytes()).into_owned().collect();
        let name = form_data.get("name").unwrap_or(&"".to_string()).to_string();
        let email = form_data.get("email").unwrap_or(&"".to_string()).to_string();
        let password = form_data.get("password").unwrap_or(&"".to_string()).to_string();

        println!("Extracted values: name = {}, email = {}, password = {}", name, email, password);

        if name.is_empty() || email.is_empty() || password.is_empty() {
            respond_with_message(stream, "400 Bad Request", "All fields are required");
            return;
        }

        println!("Creating user with name: {}, email: {}, password: {}", name, email, password);

        let mut conn = get_conn();
        match create_user(&mut conn, &name, &email, &password) {
            Ok(_) => respond_with_message(stream, "200 OK", "User registered successfully"),
            Err(e) => {
                eprintln!("Error creating user: {:?}", e);
                respond_with_message(stream, "500 Internal Server Error", "Failed to register user")
            },
        }
    } else {
        respond_with_message(stream, "400 Bad Request", "Invalid request body");
    }
}

fn respond_with_message<W: Write>(stream: &mut W, status: &str, message: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type, Authorization\r\n\r\n{}",
        status,
        message.len(),
        message
    );
    stream.write_all(response.as_bytes()).unwrap();
}

/**
 * Gère l'authentification des utilisateurs.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
pub fn handle_login<W: Write>(stream: &mut W, buffer: &[u8]) {
    // Extraction des données de la requête
    let request_body = std::str::from_utf8(buffer).unwrap();
    let params: Vec<&str> = request_body.split('&').collect();
    let mut email = "";
    let mut password = "";
    
    for param in params {
        let pair: Vec<&str> = param.split('=').collect();
        match pair[0] {
            "email" => email = pair[1],
            "password" => password = pair[1],
            _ => {}
        }
    }

    // Appel de la fonction authenticate_user pour vérifier les informations d'identification
    match authenticate_user(&mut get_conn(), email, password) {
        Some(user) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                serde_json::to_string(&user).unwrap()
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
        None => {
            let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Invalid email or password\"}";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    stream.flush().unwrap();
}

/**
 * Vérifie l'authentification de l'utilisateur.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param _buffer Le tampon contenant les données de la requête (non utilisé).
 */
pub fn handle_check_auth<W: Write>(stream: &mut W, _buffer: &[u8]) {
    // Logique de vérification de l'authentification (à implémenter)
    // Pour l'instant, nous supposons que l'utilisateur est authentifié
    let is_authenticated = true;

    if is_authenticated {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nAuthenticated";
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: text/plain\r\n\r\nUnauthorized";
        stream.write_all(response.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}