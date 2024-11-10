# Use alpine Node image as base
FROM node:bookworm-slim

# Install nginx & supervisor
RUN apt update && apt install -y nginx supervisor && apt clean && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy files
COPY artifacts/svelte_build_output/ /app/ui/
COPY artifacts/axum_build_binary/axum-api /app/

# Change ownership of Working Directory to UID 1000
RUN chown -R 1000:1000 /app

# Copy Start Script
COPY ./docker-context/docker_entrypoint.sh /etc/app/init.sh
RUN chmod +x /etc/app/init.sh /app/axum-api

# Copy nginx config
COPY ./docker-context/nginx.conf /etc/nginx/nginx.conf

# Copy supervisord config
COPY ./docker-context/supervisord.conf /etc/supervisord.conf

# Start supervisord
ENTRYPOINT ["/etc/app/init.sh"]