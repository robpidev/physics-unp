# Registro de notas para el laboratorio de Física

## 1. Técnologías utilizadas

* Base de datos: SurrealDB
* Backend: Rust (Actix Web)
* Frontend: Svelte + SvelteKit

## 2. Guía de uso

### 2.1 Profesores

A continuación se muestra la guía de uso de la sección de profesores

### 2.1.1 Login

El profesor puede ingresar con el lógin de la siguiente manera:

De click en uno de los siguientes botones:

![pantalla principal](user_guide/f1.png) "pantalla principal"

Llenando el formulario:

* El código por lo general al momento de registro será el DNI
* Y la contraseña mínimo de 8 caractres

![Login](user_guide/f2.png)

### 2.1.2 Registro

El profesor puede registrase haciendo click en el siguiente botón:

![Link de registro](user_guide/f3.png) "link de registro"

Llenando el formulario:

![Registro](user_guide/f4.png)

Luego de registrarse ir a iniciar seción:

![Inicio de sesión](user_guide/f5.png)

### 2.1.3 Cursos

Una vez iniciada sesión el profesor puede acceder a a la lista de cursos asignados

![Cursos](user_guide/f6.png)

Al dar click en un curso le aparecerá los alumnos que pertenecen
y los detalles del curso:

![Detalles del curso](user_guide/f7.png)

Y si se da click en ver (como se muestra en la parte superior)
si es profesor de práctica podrá modificar las notas, sino, solo verlas.

![Ver notas](user_guide/f8.png)
