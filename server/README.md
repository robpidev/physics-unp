# Aplicación del Servidor - Physics UNP

Este es el backend para el sistema de gestión académica de Physics UNP. Proporciona una API RESTful para gestionar estudiantes, profesores, cursos, evaluaciones y más.

## Tech Stack

*   **Lenguaje:** [Rust](https://www.rust-lang.org/) (Edición 2021)
*   **Framework Web:** [Actix Web 4](https://actix.rs/)
*   **Base de Datos:** [SurrealDB](https://surrealdb.com/)
*   **Autenticación:** [JSON Web Tokens (JWT)](https://jwt.io/)
*   **Serialización:** [Serde](https://serde.rs/)
*   **Variables de Entorno:** [Dotenvy](https://github.com/allan2/dotenvy)

## Arquitectura

La aplicación sigue una arquitectura por capas para separar las responsabilidades y mejorar la mantenibilidad.

```
+-------------------+      +-----------------+      +-----------------+      +------------------+      +-------------+
|    Cliente HTTP   |----->|    API Layer    |----->| Middleware Layer|----->|  Service Layer   |----->|   DB Layer  |
| (Navegador, App)  |      |  (actix-web)    |      | (Autenticación) |      | (Lógica Negocio) |      | (SurrealDB) |
+-------------------+      +-----------------+      +-----------------+      +------------------+      +-------------+
        ^                                                                                                   |
        |                                                                                                   v
        +---------------------------------------------------------------------------------------------------+
                                        (Respuesta JSON)
```

1.  **API Layer (Capa de API):** Expone los endpoints de la API utilizando Actix Web. Se encarga de recibir las peticiones HTTP y devolver las respuestas. Los módulos de rutas se encuentran en `src/{feature}/mod.rs`.
2.  **Middleware Layer (Capa de Middleware):** Intercepta las peticiones para realizar tareas de autenticación y autorización antes de que lleguen a los servicios. Se encuentra en `src/shared/middlewares`.
3.  **Service Layer (Capa de Servicio):** Contiene la lógica de negocio de la aplicación. Orquesta las operaciones y se comunica con la capa de base de datos. Se encuentra en `src/ {feature}/services/`.
4.  **DB Layer (Capa de Base de Datos):** Responsable de la comunicación con la base de datos SurrealDB. Abstrae las consultas y la manipulación de datos. Se encuentra en `src/{feature}/services/repository`.

## Features

*   **Autenticación:** Sistema de Sign In/Sign Up basado en roles (admin, profesor, estudiante) usando JWT.
*   **Gestión de Cursos:** Endpoints para la administración de cursos.
*   **Gestión de Evaluaciones:** Lógica para manejar las evaluaciones de los cursos.
*   **Gestión de Usuarios:** Endpoints para gestionar estudiantes, profesores y facultades.
*   **Calendario y Noticias:** Módulos para gestionar eventos y noticias.

## Configuración

La configuración del servidor se gestiona a través de un archivo `.env` en la raíz del proyecto. Puedes copiar el archivo `example.env` a `.env` y modificarlo según tus necesidades.

```
# example.env

# Llave secreta para firmar los JSON Web Tokens
SEED_JWT="thisisaseedkey"

# Host y puerto donde correrá el servidor
HOST="0.0.0.0"
PORT=8080

# Configuración de la conexión a SurrealDB
DB_WS="127.0.0.1:8000"
DB_USER="root"
DB_PASS="root"
DB_NS="test"
DB_NAME="test"
```

### Variables de Entorno

*   `SEED_JWT`: Una cadena secreta y única utilizada para firmar y verificar los tokens JWT.
*   `HOST`: La dirección IP en la que se ejecutará el servidor. `0.0.0.0` permite el acceso desde cualquier dirección de red.
*   `PORT`: El puerto en el que el servidor escuchará las conexiones entrantes.
*   `DB_WS`: La dirección del WebSocket de la base de datos SurrealDB.
*   `DB_USER`: El usuario para autenticarse en la base de datos.
*   `DB_PASS`: La contraseña para autenticarse en la base de datos.
*   `DB_NS`: El namespace a utilizar en la base de datos.
*   `DB_NAME`: El nombre de la base de datos a utilizar.

## Cómo Empezar

1.  **Clona el repositorio:**
    ```sh
    git clone <URL_DEL_REPOSITORIO>
    cd server
    ```
2.  **Crea tu archivo de configuración:**
    ```sh
    cp example.env .env
    ```
3.  **Ajusta las variables en `.env`** si es necesario (especialmente la configuración de la base de datos).

4.  **Ejecuta la aplicación:**
    ```sh
    cargo run
    ```

El servidor estará corriendo en la dirección y puerto especificados en tu archivo `.env`.