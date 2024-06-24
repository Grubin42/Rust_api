use std::io::Write;

/**
 * Gère les requêtes OPTIONS.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 * @param _buffer Le tampon contenant les données de la requête (non utilisé).
 */
pub fn handle_options_request<W: Write>(stream: &mut W, _buffer: &[u8]) {
    let response = "HTTP/1.1 200 OK\r\nAllow: OPTIONS, GET, POST, PUT, DELETE\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

/**
 * Gère les requêtes non reconnues.
 *
 * @param stream Le flux de sortie pour écrire les réponses.
 */
pub fn handle_request<W: Write>(stream: &mut W) {
    let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nThe requested resource was not found.";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}