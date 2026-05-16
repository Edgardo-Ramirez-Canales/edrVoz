# Architecture Anti-Patterns

## ❌ Lógica de negocio en el controller

```csharp
// MAL: controller con lógica pesada
public async Task<IActionResult> CreateInvoice([FromBody] InvoiceRequest req)
{
    var invoice = new Invoice();
    invoice.Total = req.Items.Sum(x => x.Price * x.Qty);
    invoice.Tax = invoice.Total * 0.16m;
    // 50 líneas más...
    await _db.SaveChangesAsync();
    return Ok(invoice);
}
```

## ❌ Acceso directo a DB desde el controller

```csharp
// MAL: DbContext inyectado directo en controller
public InvoiceController(AppDbContext db) { _db = db; }
```

## ❌ Clases de miles de líneas

Una clase = una responsabilidad. Si supera 300 líneas, probablemente hace demasiado.

## ❌ Copiar y pegar código

Extraer a un método o servicio compartido.
