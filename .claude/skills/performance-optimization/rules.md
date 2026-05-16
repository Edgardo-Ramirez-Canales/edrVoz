# Performance Optimization Rules

- Medir antes de optimizar — nunca actuar sin datos (profiler, logs, métricas).
- Queries N+1 son el problema más común en ORMs — revisar siempre con eager loading.
- Paginación obligatoria en todos los listados — nunca traer colecciones completas.
- Async/await en toda operación I/O (BD, HTTP, archivos).

> Proceso completo de diagnóstico y optimización: `00_AI_CORE/GLOBAL_PROMPTS/optimize-performance.md`
