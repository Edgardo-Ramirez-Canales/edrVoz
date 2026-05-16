# Debugging Examples

## Flujo de diagnóstico recomendado

1. Leer el error completo (mensaje + stack trace).
2. Identificar en qué capa ocurre (controller, service, repo, DB).
3. Revisar logs del servidor/aplicación.
4. Reproducir en ambiente local.
5. Agregar breakpoint o log temporal en el punto exacto.
6. Validar datos de entrada.
7. Corregir causa raíz.
8. Verificar que no rompe otros flujos.

## Herramientas útiles

- Logs estructurados con Serilog / NLog
- dotnet watch run para recarga rápida
- Postman para reproducir llamadas API
- SQL Profiler para queries lentas
