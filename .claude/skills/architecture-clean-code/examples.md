# Architecture Examples

## Estructura de capas (Clean Architecture)

```
src/
├── Domain/          # Entidades, interfaces, reglas de negocio puras
├── Application/     # Casos de uso, DTOs, servicios de aplicación
├── Infrastructure/  # Repositorios, DB, externos
└── Presentation/    # Controllers, ViewModels, APIs
```

## Ejemplo de servicio bien estructurado

```csharp
// Correcto: lógica en servicio, controller delgado
public class InvoiceService : IInvoiceService
{
    public async Task<InvoiceDto> CreateAsync(CreateInvoiceRequest request)
    {
        // validación + lógica aquí
    }
}

[ApiController]
public class InvoiceController : ControllerBase
{
    public async Task<IActionResult> Create([FromBody] CreateInvoiceRequest request)
        => Ok(await _service.CreateAsync(request));
}
```
