# Monitoring Anti-Patterns

## ❌ Logs sin estructura

```csharp
// MAL: string concatenado
_logger.LogInformation("El usuario " + userId + " hizo login");

// BIEN: estructurado
_logger.LogInformation("User {UserId} logged in", userId);
```

## ❌ Todo en nivel Error

Usar Error solo para situaciones que requieren atención inmediata. Si todo es Error, nada lo es.

## ❌ Logear passwords o tokens

```csharp
// MAL
_logger.LogDebug("Auth request: user={u}, pass={p}", email, password);
```

## ❌ Sin health endpoint

Una API sin /health no se puede monitorear en Docker ni en orquestadores.

## ❌ Ignorar logs en producción

Los logs solo tienen valor si alguien los lee o hay alertas configuradas.
