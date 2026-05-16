# Monitoring & Observability Rules

- Logging estructurado en JSON — nunca logs de texto plano.
- Nunca logear datos sensibles (passwords, tokens, PII).
- RequestId/TraceId en cada request para correlación de logs.
- Health endpoint (`/health`) en todas las APIs.

> Niveles de log, formato y retención: `00_AI_CORE/GLOBAL_STANDARDS/logging.md`
