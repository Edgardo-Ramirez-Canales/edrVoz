# Security Anti-Patterns

## ❌ Secrets en el código

```csharp
// MAL
var key = "MiClaveSecreta123"; // hardcodeado
```

## ❌ SQL Injection

```csharp
// MAL: concatenación directa
var query = $"SELECT * FROM Users WHERE Name = '{name}'";

// BIEN: parámetros
var query = "SELECT * FROM Users WHERE Name = @name";
```

## ❌ Confiar en el cliente

Nunca confiar en datos que vienen del frontend sin validar en backend.

## ❌ Autorización faltante

```csharp
// MAL: solo autenticación, sin validar si el usuario puede acceder al recurso
[Authorize]
public async Task<IActionResult> GetInvoice(int id) 
{
    return Ok(await _repo.GetById(id)); // cualquier usuario autenticado puede ver cualquier factura
}
```

## ❌ Passwords en logs

```csharp
// MAL
_logger.LogInformation("Login attempt: user={user}, pass={pass}", email, password);
```
