# Documentation Examples

## README mínimo

```markdown
# Nombre del Proyecto

Descripción breve del proyecto.

## Stack

- Backend: ASP.NET Core 8
- Frontend: Angular 17
- Base de datos: SQL Server 2022
- Contenedores: Docker

## Setup rápido

```bash
git clone ...
cd proyecto
cp .env.example .env
docker-compose up -d
```

## Estructura

```
src/
├── backend/
├── frontend/
└── docker/
```

## Variables de entorno

Ver `.env.example`
```

## .env.example

```bash
# Base de datos
DB_SERVER=localhost
DB_NAME=mi_app
DB_USER=sa
DB_PASSWORD=

# JWT
JWT_KEY=
JWT_ISSUER=https://miapp.com
JWT_EXPIRY_HOURS=8

# Ambiente
ASPNETCORE_ENVIRONMENT=Development
```
