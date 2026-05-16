# Performance Anti-Patterns

## ❌ Optimizar sin medir

Nunca refactorizar por "intuición de que es lento". Medir primero.

## ❌ SELECT * en queries

```sql
-- MAL
SELECT * FROM Invoices WHERE TenantId = @id

-- BIEN
SELECT Id, Number, Total, CreatedAt FROM Invoices WHERE TenantId = @id
```

## ❌ Lazy loading activo con colecciones grandes

Cada acceso a una propiedad lazy genera una query. En loops = desastre.

## ❌ Cargar toda la tabla para filtrar en memoria

```csharp
// MAL: carga todo y filtra en C#
var activos = (await _db.Users.ToListAsync())
    .Where(u => u.IsActive);

// BIEN: filtra en la query
var activos = await _db.Users.Where(u => u.IsActive).ToListAsync();
```

## ❌ Operaciones síncronas bloqueantes en I/O

```csharp
// MAL: bloquea el thread
var result = _db.Invoices.ToList();

// BIEN
var result = await _db.Invoices.ToListAsync();
```
