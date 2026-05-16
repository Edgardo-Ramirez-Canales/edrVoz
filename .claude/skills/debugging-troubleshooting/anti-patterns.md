# Debugging Anti-Patterns

## ❌ Modificar código sin entender el error

Cambiar cosas al azar hasta que "funcione" genera deuda técnica y errores ocultos.

## ❌ Ignorar el stack trace

El stack trace muestra exactamente dónde ocurrió el error. Leerlo completo.

## ❌ Asumir sin datos

"Creo que es la base de datos" sin revisar logs no es debugging, es adivinanza.

## ❌ Debuggear en producción

Siempre reproducir en local o staging antes de tocar producción.

## ❌ No verificar git blame

Muchos bugs vienen de cambios recientes. `git log` y `git blame` son tus amigos.
