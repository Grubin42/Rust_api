// use std::io::Write;
// use crate::db::connection::establish_connection;
// use crate::db::user::fetch_users;
// use serde_json;

// /// Gère les requêtes GET pour les utilisateurs.
// ///
// /// Cette fonction établit une connexion à la base de données, récupère tous les utilisateurs,
// /// et écrit la réponse HTTP avec les données des utilisateurs en format JSON.
// ///
// /// # Arguments
// ///
// /// * `stream` - Un flux de type `impl Write` pour écrire la réponse HTTP.
// pub fn handle_get_users(stream: &mut impl Write) {
//     let mut connection = establish_connection();
//     let users = fetch_users(&mut connection);
    
//     let response = format!(
//         "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
//         serde_json::to_string(&users).unwrap()
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }