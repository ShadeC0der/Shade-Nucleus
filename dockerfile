# Imagen base segura y liviana con Rust
FROM rust:slim

# Establece la carpeta de trabajo en el contenedor
WORKDIR /app

# Copia el proyecto completo (incluye nucleus y demo)
COPY . .

# Compila el ejecutable del demo en modo release
RUN cargo build --release -p demo

# Comando por defecto al ejecutar el contenedor
CMD ["cargo", "build", "--release", "-p", "demo"]

