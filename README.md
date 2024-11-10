# Self-Hosted AI Chat App

A simple, self-hosted chat application that lets you interact with AI APIs while keeping your conversation data on your own machine.

## ⚠️ Privacy Notice

- **Your Data Storage**: All conversations are stored ONLY in your local database
- **API Data Usage**: While your chat history stays on your machine, be aware that AI providers (like OpenAI, Anthropic, etc.) may receive and process the messages you send through their APIs
- **Best Practice**: Review the privacy policies of any AI APIs you connect to

## 🚀 Quick Start

### Prerequisites

- Docker and Docker Compose

### Installation Example

Create a `docker-compose.yml`:

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
    image: ghcr.io/beyond-chat/selfhosted:latest
    environment:
      # Dynamically create the DATABASE_URL using the variables
      DATABASE_URL: 'postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}'
    volumes:
      - beyondchat-data:/app/.data/llm # Mount volume to persist LLM data like API keys
    container_name: beyond-chat
    ports:
      - '3000:80'
    depends_on:
      - postgres
```

## Start App
docker compose up-d
### Access App at http://localhost:3000/
