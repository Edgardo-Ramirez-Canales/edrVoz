# Security Rules

- Secrets nunca en código — siempre en `.env` y documentados en `.env.example`.
- Validar y sanitizar todos los inputs en application layer antes de procesar.
- JWT: validar firma, expiración y claims obligatorios en cada request protegido.
- HTTPS obligatorio en producción — sin excepciones.
- Principio de mínimo privilegio en cuentas, roles y conexiones a BD.

> Checklist completo OWASP: `00_AI_CORE/GLOBAL_PROMPTS/security-review.md`
