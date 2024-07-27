Rust y Docker

Comencemos “dockerizando” la aplicación que hemos creado en Rust. Para esto, luego de instalar Docker en tu ordenador, generaremos algunos archivos necesarios en la raíz del proyecto.

Paso 1. Dockerización

Archivo .dockerignore para que Docker ignore algunos archivos o directorios.

```console
target/
```

Archivo Dockerfile para construir la imagen de Docker.

```console
# STAGE 1: Creamos contenedor para compilar la aplicación
FROM ubuntu:20.04

# Variable para evitar bloqueos en la terminal al construir el contenedor
ENV DEBIAN_FRONTEND=noninteractive

## Dependencias necesarios del Sistema Operativo
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y

## Instalamos Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

## Preparamos directorio principal del proyecto
WORKDIR /app
COPY ./ /app

## Compilamos la aplicación
RUN cargo clean
RUN cargo build --release
```

```console
# STAGE 2: Como el primer contenedor es muy pesado y no lo necesitamos, creamos otro solo para exponer la aplicación
FROM ubuntu:20.04

# Variable para evitar bloqueos en la terminal al construir el contenedor
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
WORKDIR /app

## Copiamos desde el otro contenedor, los archivos de la aplicación
COPY --from=0 /app/target/release/rust-backend /app
COPY /templates/ /app/templates
COPY /statics/ /app/statics

## Corremos la aplicación
CMD ./rust-backend
```

Archivo docker-compose.yml para configurar y levantar el contenedor con más facilidad.

```console
version: '3'
services:
  rust-backend:           # Nombre de la imagen
    build: .              # Seleccionamos el Dockerfile en la raíz del proyecto
    image: rust-backend
    env_file:             # Leemos variables de entorno
      - .env
    ports:
      - "8080:8080"       # El primer puerto es para ingresar desde el navegador, el segundo el puerto configurado en el .env
```

NOTA: Docker Compose es una herramienta que viene junto a Docker. Nos servirá para correr múltiples contenedores juntos y de forma más fácil.

Paso 2. Preparar servidor HTTP

Antes de probar el contenedor, prepara tu aplicación con una nueva variable de entorno para configurar dinámicamente el puerto en el que escuchará el servidor HTTP.

```console
# .env
DATABASE_URL=postgres://zspkplmxcqqpuo:d7e...
PORT=8080
```

```console
// main.rs
// Traemos la variable de entorno y la convertimos a entero
let port = env::var("PORT").expect("La variable de entorno PORT no existe.");
let port: u16 = port.parse().unwrap();

// Configuramos el puerto dinamicamente
HttpServer::new(move || { /* ... */ }).bind(("0.0.0.0", port)).unwrap().run().await
```

TIP: Para evitar otros inconvenientes, configura el host de tu aplicación como ```0.0.0.0``` para que funcione tanto en local, como con Docker y posteriormente en Heroku.

Paso 3. Lanzar la aplicación

Teniendo Docker listo, vamos a probar el contenedor de forma local con el comando ```sudo docker-compose up -d```. Este proceso tardará varios minutos la primera vez que lo ejecutes.

Una vez finalizado, si todo ha ido bien, podrás ingresar a tu aplicación corriendo en Docker desde ```localhost:8080``` (o con el puerto que hayas definido en el ```docker-compose.yml```).

¡Felicidades! Tu aplicación está lista para ser desplegada en Heroku.