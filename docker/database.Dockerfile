# Use the official MariaDB image from Docker Hub
FROM mariadb:latest

# Copy the database initialization script into the container
COPY ./database/init.sql /docker-entrypoint-initdb.d/init.sql
COPY ./database/user.sql /docker-entrypoint-initdb.d/