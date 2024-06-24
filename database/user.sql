USE mydatabase;

CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Ajoutez des utilisateurs de test
INSERT INTO users (name, email, password) VALUES
('John Doe', 'john@example.com', '$2y$12$KIXTOGrlQQ0V1UnQFvN3BebVE2uIR/G.1w64FnJtI4VZyQ9vxkPuq'), -- password: password1
('Jane Doe', 'jane@example.com', '$2y$12$eRVzZc2E5Zx5G9F8Rx2bI.8LSMZ1/at5Ypi8oZXGyqH/3rTPKdm9W'), -- password: password2
('Alice Smith', 'alice@example.com', '$2y$12$9D1Q7GRBvnQyt6zq29Nlu.H/aj.PT5NTwPU3iFPkFvX1XlOPdY7L2'); -- password: password3