use std::io::Write;

pub fn auth_guard(buffer: &[u8]) -> bool {
    // Simule une vérification d'authentification
    // Dans une application réelle, vérifiez les en-têtes de session ou les jetons JWT
    let auth_header = b"Authorization: Bearer token";
    buffer.windows(auth_header.len()).any(|window| window == auth_header)
}

pub fn handle_unauthorized(stream: &mut impl Write) {
    let response = "HTTP/1.1 401 Unauthorized\r\nContent-Type: text/plain\r\n\r\nUnauthorized";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}