# Use the official Caddy image from Docker Hub
FROM caddy:latest

# Copy the Caddyfile into the container
COPY ./Caddyfile /etc/caddy/Caddyfile

# Expose ports 80 and 443 for HTTP and HTTPS traffic
EXPOSE 80
EXPOSE 443

# Start Caddy when the container is run
CMD ["caddy", "run", "--config", "/etc/caddy/Caddyfile"]