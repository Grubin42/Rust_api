use std::io::Write;
use crate::handlers::auth::{handle_register, handle_login, handle_check_auth};
use crate::handlers::profile::{handle_profile, handle_edit_profile, handle_delete_profile};
use crate::handlers::common::{handle_options_request, handle_request};
use crate::middleware::auth::{auth_guard, handle_unauthorized};

/**
 * Route les requêtes utilisateur en fonction de l'URL de la requête.
 *
 * Cette fonction utilise une table de routage pour associer les chemins d'URL aux gestionnaires
 * correspondants. Si l'URL correspond à un gestionnaire enregistré, la requête est traitée par
 * ce gestionnaire. Sinon, une réponse par défaut est envoyée.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
pub fn route_user_requests<W: Write>(stream: &mut W, buffer: &[u8]) {
   //println!("Request received in route_user_requests: {:?}", std::str::from_utf8(buffer).unwrap_or("Invalid UTF-8"));
    let routes: Vec<(&[u8], fn(&mut W, &[u8]))> = vec![
        (b"POST /api/register HTTP/1.1\r\n", handle_register),
        (b"POST /api/login HTTP/1.1\r\n", handle_login),
        (b"GET /api/check-auth HTTP/1.1\r\n", handle_check_auth),
        (b"GET /api/profile HTTP/1.1\r\n", handle_profile_wrapper),
        (b"PUT /api/profile/edit HTTP/1.1\r\n", handle_edit_profile_wrapper),
        (b"DELETE /api/profile/delete HTTP/1.1\r\n", handle_delete_profile_wrapper),
        (b"OPTIONS /api HTTP/1.1\r\n", handle_options_request_wrapper),
    ];

    for (route, handler) in routes {
        if buffer.starts_with(route) {
            println!("Handling route: {:?}", std::str::from_utf8(route).unwrap_or("Invalid UTF-8"));
            handler(stream, buffer);
            return;
        }
    }

    println!("No matching route found");
    handle_request(stream);
}

/**
 * Enveloppe la fonction `handle_profile` avec une vérification d'authentification.
 *
 * Cette fonction vérifie si l'utilisateur est authentifié avant d'appeler la fonction `handle_profile`.
 * Si l'utilisateur n'est pas authentifié, une réponse non autorisée est envoyée.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
fn handle_profile_wrapper<W: Write>(stream: &mut W, buffer: &[u8]) {
    if auth_guard(buffer) {
        handle_profile(stream, buffer);
    } else {
        handle_unauthorized(stream);
    }
}

/**
 * Enveloppe la fonction `handle_edit_profile` avec une vérification d'authentification.
 *
 * Cette fonction vérifie si l'utilisateur est authentifié avant d'appeler la fonction `handle_edit_profile`.
 * Si l'utilisateur n'est pas authentifié, une réponse non autorisée est envoyée.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
fn handle_edit_profile_wrapper<W: Write>(stream: &mut W, buffer: &[u8]) {
    if auth_guard(buffer) {
        handle_edit_profile(stream, buffer);
    } else {
        handle_unauthorized(stream);
    }
}

/**
 * Enveloppe la fonction `handle_delete_profile` avec une vérification d'authentification.
 *
 * Cette fonction vérifie si l'utilisateur est authentifié avant d'appeler la fonction `handle_delete_profile`.
 * Si l'utilisateur n'est pas authentifié, une réponse non autorisée est envoyée.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
fn handle_delete_profile_wrapper<W: Write>(stream: &mut W, buffer: &[u8]) {
    if auth_guard(buffer) {
        handle_delete_profile(stream, buffer);
    } else {
        handle_unauthorized(stream);
    }
}

/**
 * Enveloppe la fonction `handle_options_request` pour qu'elle corresponde à la signature attendue.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param buffer Le tampon contenant les données de la requête.
 */
fn handle_options_request_wrapper<W: Write>(stream: &mut W, buffer: &[u8]) {
    handle_options_request(stream, buffer);
}