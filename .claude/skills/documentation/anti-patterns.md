# Documentation Anti-Patterns

## ❌ README vacío o inexistente

Un repo sin README obliga a leer todo el código para entender qué hace.

## ❌ Documentación desactualizada

Peor que no tener docs es tener docs incorrectos. Actualizar al mismo tiempo que el código.

## ❌ Comentarios que explican el QUÉ en lugar del POR QUÉ

```csharp
// MAL: explica lo obvio
// Incrementar contador
counter++;

// BIEN: explica por qué
// Oracle no soporta SEQUENCE en este contexto, usamos MAX+1
var nextId = await _db.Invoices.MaxAsync(i => (int?)i.Id) + 1 ?? 1;
```

## ❌ Secrets en la documentación

No poner valores reales de passwords, keys ni tokens en ningún documento del repo.

## ❌ Documentación fuera del repositorio

Si los docs viven solo en Confluence o Notion y el repo no tiene nada, eventualmente quedan desincronizados.
