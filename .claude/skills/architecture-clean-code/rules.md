# Architecture Rules

- No lógica de negocio en controllers — va en servicios o application layer.
- Dependencias hacia adentro: `domain` no conoce `infrastructure` ni `presentation`.
- DTOs obligatorios entre capas — nunca exponer entidades de dominio directamente.
- Interfaces para desacoplar implementaciones (repositorios, servicios externos).
- Cada módulo es independiente y reutilizable (alta cohesión, bajo acoplamiento).

> Estructura de carpetas detallada: `00_AI_CORE/GLOBAL_STANDARDS/folder-structures.md`
