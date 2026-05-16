# Git Workflow Examples

## Flujo de trabajo estándar

```bash
# Crear rama para nueva funcionalidad
git checkout -b feature/invoice-module

# Commits atómicos con mensaje descriptivo
git add src/Invoice/
git commit -m "add invoice creation endpoint"

git add tests/Invoice/
git commit -m "add unit tests for invoice service"

# Sincronizar con main antes de PR
git fetch origin
git rebase origin/main

# Subir rama
git push origin feature/invoice-module
# Abrir Pull Request en GitHub
```

## Formato de mensaje de commit

```
<tipo>: <descripción breve>

Tipos: feat, fix, docs, chore, refactor, test, style
Ejemplos:
  feat: add multi-tenant invoice filtering
  fix: resolve null reference in billing service
  chore: update nuget packages
```

## Estrategia de ramas

```
main          ← producción estable
  └── develop ← integración
        ├── feature/xxx
        ├── fix/xxx
        └── hotfix/xxx
```
