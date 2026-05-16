# DevOps Examples

## Dockerfile multi-stage (ASP.NET)

```dockerfile
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src
COPY . .
RUN dotnet publish -c Release -o /app/publish

FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS runtime
WORKDIR /app
COPY --from=build /app/publish .
HEALTHCHECK --interval=30s --timeout=5s \
  CMD curl -f http://localhost:80/health || exit 1
ENTRYPOINT ["dotnet", "MyApp.dll"]
```

## Variables por ambiente

```yaml
# docker-compose.override.yml (dev)
services:
  api:
    environment:
      - ASPNETCORE_ENVIRONMENT=Development
      - ConnectionStrings__Default=Server=localhost;...
```

## GitHub Actions básico

```yaml
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: dotnet build
      - run: dotnet test
```
