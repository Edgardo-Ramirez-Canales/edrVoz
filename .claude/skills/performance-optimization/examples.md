# Performance Examples

## Evitar N+1 con Include

```csharp
// MAL: N+1 queries
var orders = await _db.Orders.ToListAsync();
foreach (var order in orders)
    Console.WriteLine(order.Customer.Name); // query por cada order

// BIEN: eager loading
var orders = await _db.Orders
    .Include(o => o.Customer)
    .ToListAsync();
```

## Paginación

```csharp
public async Task<PagedResult<InvoiceDto>> GetPagedAsync(int page, int pageSize)
{
    var query = _db.Invoices.AsNoTracking();
    var total = await query.CountAsync();
    var items = await query
        .Skip((page - 1) * pageSize)
        .Take(pageSize)
        .Select(i => new InvoiceDto { Id = i.Id, Total = i.Total })
        .ToListAsync();
    return new PagedResult<InvoiceDto>(items, total, page, pageSize);
}
```

## Caché en memoria

```csharp
public async Task<List<TenantDto>> GetTenantsAsync()
{
    return await _cache.GetOrCreateAsync("tenants", async entry =>
    {
        entry.AbsoluteExpirationRelativeToNow = TimeSpan.FromMinutes(10);
        return await _db.Tenants.Select(t => new TenantDto(t)).ToListAsync();
    });
}
```
