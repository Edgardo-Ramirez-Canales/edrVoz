# Security Examples

## JWT Validation (ASP.NET)

```csharp
services.AddAuthentication(JwtBearerDefaults.AuthenticationScheme)
    .AddJwtBearer(options =>
    {
        options.TokenValidationParameters = new TokenValidationParameters
        {
            ValidateIssuer = true,
            ValidateAudience = true,
            ValidateLifetime = true,
            ValidateIssuerSigningKey = true,
            ValidIssuer = config["Jwt:Issuer"],
            ValidAudience = config["Jwt:Audience"],
            IssuerSigningKey = new SymmetricSecurityKey(
                Encoding.UTF8.GetBytes(config["Jwt:Key"]))
        };
    });
```

## Secrets en variables de entorno

```json
// appsettings.json — sin valores sensibles
{
  "ConnectionStrings": {
    "Default": ""
  }
}
```

```bash
# .env o variables del sistema
ConnectionStrings__Default=Server=...;Password=...
```

## Autorización por rol

```csharp
[Authorize(Roles = "Admin,Manager")]
[HttpDelete("{id}")]
public async Task<IActionResult> Delete(int id) { ... }
```
