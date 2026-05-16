# DevOps Anti-Patterns

## ❌ Deploy manual sin proceso

Deployar copiando archivos a mano genera errores inconsistentes.

## ❌ Sin healthcheck

Un contenedor sin healthcheck puede estar "corriendo" pero completamente roto.

## ❌ Mismas variables para todos los ambientes

Dev y Prod deben tener configuraciones separadas e independientes.

## ❌ Sin rollback plan

Antes de cada deploy a producción: ¿cómo revertimos si algo sale mal?

## ❌ Imagen Docker con todo el SDK en producción

Usar multi-stage: el runtime final no debe incluir el SDK de compilación.
