# Self-Hosted AI Chat App

A simple, self-hosted chat application that lets you interact with AI APIs while keeping your conversation data on your own machine.

## 🚀 Quick Start & Installation Example

- **Easy Deployment**: Quick setup via **Docker Compose** or **docker run**, making deployment on your local machine or server a breeze.

### Option 1: Using Docker Compose
This option sets up both the BeyondChat application and PostgreSQL database containers together.
1. Create a `.env` file:

```env
POSTGRES_USER=beyondchat
POSTGRES_PASSWORD=your_secure_password
POSTGRES_DB=beyondchat_db
```

2. Create a `docker-compose.yml`:

```yaml
volumes:
  postgres-data:
  beyondchat-data:

services:
  postgres:
    image: postgres:latest
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    volumes:
      - postgres-data:/var/lib/postgresql/data # Mount volume to persist PostgreSQL data where chats are stored

  beyond-chat:
    image: ghcr.io/beyond-chat/selfhosted:main
    environment:
      # Dynamically create the POSTGRES_DB_URL using the variables
      POSTGRES_DB_URL: 'postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}'
      APP_URL: http://localhost:3000
      # APP_URL: http://192.168.0.40:3000
      # APP_URL: http://my-beyondchat.site
    volumes:
      - beyondchat-data:/app/.data/llm # Mount volume to persist LLM data like API keys
    container_name: beyond-chat
    ports:
      - '3000:80'
      # - '192.168.0.40:3000:80'
      # - 'my-beyondchat.site:80:80'
    depends_on:
      - postgres
```
3. Start the application:

```bash
docker compose up -d
```

### Option 2: Using Single Docker Container
If you already have a PostgreSQL database, you can run BeyondChat as a standalone container.

Run the container:
```bash
docker run -d \
  --name beyond-chat \
  -p 3000:80 \
  -e POSTGRES_DB_URL='postgres://user:password@your-db-host:5432/dbname' \
  -e APP_URL='http://localhost:3000' \
  -v beyondchat-data:/app/.data/llm \
  ghcr.io/beyond-chat/selfhosted:main
```

### Access App at http://localhost:3000/ or APP_URL

## 📝 Environment Variables

## 📝 Environment Variables

|
 Variable 
|
 Description 
|
 Example 
|
|
----------
|
-------------
|
---------
|
|
 POSTGRES_DB_URL 
|
 PostgreSQL connection URL 
|
`postgres://user:password@host:5432/dbname`
|
|
 APP_URL 
|
 Public URL where the app will be accessed 
|
`http://localhost:3000`
|

## 🔒 Privacy Features

- **Full Control Over Data**: You are in full control of the server and the data it handles, allowing you to avoid using cloud-based services if preferred.
- **Your Data Storage**: All conversations are stored ONLY in your local database

## ⚠️ Privacy Notice

- **API Data Usage**: While your chat history stays on your machine, be aware that AI providers (like OpenAI, Anthropic, etc.) may receive and process the messages you send through their APIs
- **Best Practice**: Review the privacy policies of any AI APIs you connect to

## 🛟 Support

If you encounter any issues or need help, please:
1. Check our [Issues](https://github.com/beyond-chat/selfhosted/issues) page
2. Open a new issue if you can't find a solution
3. As with any application, bugs can happen. If you find one, please report it via GitHub Issues.