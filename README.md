# Proyecto de Juego

## Descripción

Este es un proyecto de juego en 3D/2D desarrollado en Rust, el juego esta basado en la saga de juegos "Shin Megami Tensei" y traté de replicar las mecánicas más básicas de los juegos.

## Archivos

Descripción de los archivos del proyecto:

- **actions.rs**: Contiene funciones relacionadas con las acciones del jugador, como interactuar con puertas y comprobar los objetivos.
- **color.rs**: Maneja definiciones de colores y utilidades usadas en todo el juego.
- **enemiesParty.rs**: Gestiona los datos del grupo de enemigos y las acciones relacionadas con los enemigos en combate.
- **framebuffer.rs**: Maneja el framebuffer, dibujando y limpiando la pantalla.
- **map1.txt, map2.txt, map.txt**: Archivos de mapas usados en el juego. Contienen la disposición del nivel y las posiciones de los elementos como enemigos, jugador y objetivos.
- **map_loader.rs**: Maneja la carga de los archivos de mapa y el análisis de los datos del mapa.
- **music**: Contiene los archivos de música que se reproducen durante los diferentes estados del juego (por ejemplo, pantalla de título, jugabilidad).
- **player.rs**: Gestiona los datos del jugador, incluyendo posición, salud y acciones.
- **texture.rs**: Maneja la carga y gestión de texturas para el juego.
- **bitmap.rs**: Contiene funciones para trabajar con imágenes de mapas de bits usadas en el juego.
- **combat.rs**: Gestiona las mecánicas de combate, incluyendo ataques, defensa y la lógica por turnos.
- **enemy.rs**: Define la estructura y comportamiento de los enemigos en el juego.
- **main.rs**: El punto de entrada del juego, donde se maneja el bucle principal y las transiciones de estado.
- **party.rs**: Gestiona el grupo de jugadores y sus estadísticas durante el combate.
- **raycasting.rs**: Implementa la funcionalidad de raycasting para renderizar el mundo en 3D.
- **textures**: Carpeta que contiene los archivos de texturas usados en el juego.

## Cómo Jugar

1. **Iniciar el juego**: Al iniciar el juego, estarás en la pantalla de título.
2. **Navegar por los menús**:
    - Presiona `C` para ir al menú.
    - Presiona `B` para comenzar el juego y jugar el primer nivel.
    - Presiona `R` para volver a la pantalla de título.
3. **Controles en el juego**:
    - **Movimiento**: Usa las teclas de flecha para mover al jugador.
    - **Interacción con puertas**: Presiona `E` para interactuar con las puertas.
    - **Combate**:
        - Presiona `A` para atacar.
        - Presiona `D` para defender.
        - Presiona `S` para activar un hechizo.
        - Presiona `I`, `O` o `P` para lanzar un hechizo específico.
        - Presiona `F` para pasar el turno.
    - **Cambiar de mapa**: Cuando todos los enemigos sean derrotados y el objetivo este delante, presiona `N` para proceder al siguiente mapa.
## Estados del Juego

El juego incluye diferentes estados que dictan el flujo del juego:

- **Title**: La pantalla inicial con el título del juego.
- **Menu**: El menú donde puedes empezar a jugar o regresar a la pantalla de título.
- **Playing**: El estado principal del juego donde navegas por el mapa y encuentras enemigos.
- **Combat**: El estado de combate donde luchas contra enemigos en combate por turnos.
- **Win**: El estado que se activa cuando todos los enemigos son derrotados y el objetivo es alcanzado.
- **GameOver**: El estado que se activa cuando el jugador pierde toda su salud.

## Instalación

1. Ejecuta los siguientes comandos:
   ```sh
   git clone https://github.com/G2309/GC_Proyecto1.git
   cd GC_Proyecto1
   cargo build --release
   ./target/release/Proyecto-01
   ```
