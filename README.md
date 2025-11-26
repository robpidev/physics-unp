# Registro de notas para el laboratorio de Física

## 1. Técnologías utilizadas

* Base de datos: SurrealDB
* Backend: Rust (Actix Web)
* Frontend: Svelte + SvelteKit

## 2. Guía de uso

En la pantalla principal se
podrán ver los avisos y
los links de inicio de sesión
tanto para alumnos y profesores es el (mismo)

![Pantalla principal](user_guide/f1.png)


Tambien se pueden ver
avisos que aya

![Avisos](user_guide/a1.png)

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

## 2.2 Panel de administracion
Cuando inicie sesión el administrador podrá hacer
click en su nombre para acceder al panel de administración

![Panel de administración](user_guide/admin1.png)

### 2.2.1 Facultades

Aqui se puede crear y ver
las facultades existentes

![Facultades](user_guide/admin2.png)

### 2.2.2 Escuelas

si se da click en una facultad
se podrán ver las escuelas existentes y tambien agregar nuevas

![Escuelas](user_guide/admin3.png)

### 2.2.3 Carreras   

si se da click en una escuela
se podrán ver los cursos existentes así como agregar nuevos con nombres y cantidad de alumnos matriculados y 
vacantes disponibles

![Carreras](user_guide/admin4.png)

### 2.2.4 Cursos
Al hacer click en un curso
se podra ver los profesores
asignados del curso tanto como teoría y práctica.

También se podrá asignar un profesor a un curso o quitarlo.

![Cursos](user_guide/admin5.png)

Y la lista de alumnos
matriculados en el curso asi
como también las evaluaciones
y el peso que tiene
el test y la guía de práctica

![Alumnos](user_guide/admin6.png)

Al hacer click en ver se
podrán ver las notas detalladamente y también
se podrá modificar(Si esta
habilidado para modificar).

El promedio se realia de acuerdo a las notas acumuladas
hasta el momento

![Ver notas](user_guide/admin7.png)

### 2.2.5 Horarios para realizar registros

En la pestaña de horarios
se podrán ver los horarios
para matricularse a un curso:
``student`` para los alumnos
``professor`` para los profesores
o para registrar las prácticas
: ``1, 2, 3, ...``
asi omo tambien se podrán eliminar

![Horarios](user_guide/admin8.png)

### 2.2.6 Avisos

En la pestaña de avisos
se podrán ver los avisos
que se han publicado
asi omo tambien se podrán eliminar y agregar nuevos

![Avisos](user_guide/admin9.png)

## 2.3 Alumnos

### 2.3.1 Login

El alumno puede ingresar con el lógin de la siguiente manera:

Usando su código universitario
y la contraseña

![Login](user_guide/f2.png)

### 2.3.2 Registro
Si no están registrados aún
pueden hacerlo haciendo click en
el boton de registrate
y llenando los datos

![Link de registro](user_guide/s1.png)

### 2.3.1 Cursos
Al momento de entrar
tendran el pefil con sus cursos y puede ver las notas
haciendo click en ver notas

![Perfil](user_guide/s2.png)

Vista de notas

![Ver notas](user_guide/s3.png)
