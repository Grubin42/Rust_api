use std::net::TcpListener;
use std::io::Read;
use crate::handlers::common::handle_request;
use crate::routes::user;

/**
 * Démarre le serveur et écoute les connexions entrantes.
 *
 * Cette fonction accepte un `TcpListener` configuré pour écouter les connexions entrantes.
 * Pour chaque connexion entrante, elle lit les données de la requête et délègue la requête
 * à la fonction appropriée pour traitement en fonction de l'URL de la requête.
 *
 * @param listener Un `TcpListener` configuré pour écouter les connexions entrantes.
 *
 * @return Un `std::io::Result<()>` indiquant le succès ou l'échec de l'opération.
 */
pub fn run(listener: TcpListener) -> std::io::Result<()> {
    println!("COUCOU");

    for stream in listener.incoming() {
        println!("COUCOU1: Connection established");
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        //println!("Request received: {:?}", std::str::from_utf8(&buffer).unwrap_or("Invalid UTF-8"));
                        if buffer.starts_with(b"GET /api")
                            || buffer.starts_with(b"POST /api")
                            || buffer.starts_with(b"PUT /api")
                            || buffer.starts_with(b"DELETE /api")
                            || buffer.starts_with(b"OPTIONS /api") {
                            println!("COUCOU2: Handling API request");
                            user::route_user_requests(&mut stream, &buffer);
                        } else {
                            println!("COUCOU3: Handling non-API request");
                            handle_request(&mut stream);
                        }
                    }
                    Err(e) => {
                        println!("Failed to read from connection: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to establish connection: {:?}", e);
            }
        }
    }

    Ok(())
}