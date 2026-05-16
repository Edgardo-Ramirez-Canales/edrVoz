# Git Anti-Patterns

## ❌ Commit directo a main

Siempre trabajar en ramas y mergear via Pull Request.

## ❌ Commits masivos con todo mezclado

```
# MAL: un commit con todo
git commit -m "changes"  # 47 archivos modificados
```

## ❌ Mensajes sin contexto

```
# MAL
git commit -m "fix"
git commit -m "asdf"
git commit -m "wip"
```

## ❌ .gitignore faltante

Sin .gitignore se suben node_modules, bin/, obj/, .env y secrets.

## ❌ Force push a ramas compartidas

Reescribir historia en ramas que otros usan genera conflictos imposibles.
