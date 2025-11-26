# Physics UNP Client

Este es el cliente web para la gestión de cursos, estudiantes y profesores del departamento de física de la UNP. Proporciona interfaces separadas para administradores, profesores y estudiantes.

## Tecnologías Utilizadas

- **Framework**: Svelte 5 y SvelteKit
- **Build Tool**: Vite
- **Runtime y Gestor de Paquetes**: Bun
- **Testing**: Playwright
- **Linting y Formateo**: ESLint y Prettier

## Arquitectura

El proyecto sigue la arquitectura proporcionada por SvelteKit, que se basa en un sistema de enrutamiento basado en archivos.

- **Svelte 5**: Se utiliza la última versión de Svelte, aprovechando sus nuevas características de reactividad.
- **Enrutamiento**: Las rutas se definen por la estructura de directorios dentro de `src/routes`. Cada ruta puede tener un componente de página (`+page.svelte`), un endpoint de servidor (`+server.js`) y lógica del lado del servidor (`+page.server.js`).
- **Lógica de Servidor**: La lógica que se ejecuta en el servidor (por ejemplo, para obtener datos de una base de datos) se encuentra en los archivos `+page.server.js` o `+server.js`.
- **Componentes**: Los componentes reutilizables de Svelte se encuentran en `src/lib/components`.

## Estructura del Proyecto

```
/
├── src/
│   ├── lib/              # Módulos de la librería (componentes, utilidades)
│   ├── routes/           # Rutas de la aplicación
│   │   ├── admin/        # Flujo y vistas para administradores
│   │   ├── professor/    # Flujo y vistas para profesores
│   │   └── student/      # Flujo y vistas para estudiantes
│   ├── ...       
│   └── app.html          # Plantilla HTML principal
├── static/               # Archivos estáticos (imágenes, CSS)
├── tests/                # Tests de Playwright
├── package.json          # Dependencias y scripts
└── svelte.config.js      # Configuración de SvelteKit
```

## Cómo Empezar

Sigue estos pasos para levantar el entorno de desarrollo.

### Prerrequisitos

Asegúrate de tener [Bun](https://bun.sh/) instalado.

### Instalación

1.  Clona el repositorio:
    ```bash
    git clone <URL_DEL_REPOSITORIO>
    cd client
    ```

2.  Instala las dependencias:
    ```bash
    bun install
    ```

### Ejecutar en Desarrollo

Para iniciar el servidor de desarrollo, ejecuta:

```bash
bun run dev
```

Esto levantará la aplicación en `http://0.0.0.0:3000` por defecto.
Puedes editar esto editando el archivo ```vite.config.js```

## Scripts Disponibles

- `bun run dev`: Inicia el servidor de desarrollo.
- `bun run build`: Compila la aplicación para producción.
- `bun run preview`: Sirve la compilación de producción localmente.
- `bun run test`: Ejecuta los tests de Playwright.
- `bun run lint`: Revisa el código en busca de errores de estilo.
- `bun run format`: Formatea todo el código del proyecto con Prettier.
