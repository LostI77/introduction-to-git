# Introducción a Git

Git es una hermosa herramienta de control de versiones,
nos permite organizar nuestra app/projecto en diferentes
versiones para no perdernos en el desarrollo y si en algun
momento tenemos un error muy complicado de resolver o algo
que rompe nuestra app podemos volver a una versión anterior
facilmente!

Usar Git aumenta mucho nuestra productividad, tanto para
projectos personales como para aportar a otros.

Esta guía no cubre la instalación de git en tu computador,
asi que por favor mira algun video que cubra la instalación.

Sugerencias de guías aparte de este repositorio:

[Fazt](https://youtu.be/HiXLkL42tMU?si=CHgD-ORd7SwZuQAU)
[Midudev](https://youtu.be/niPExbK8lSw?si=8P23TE-sVVTw_oK-)
[MoureDev](https://youtu.be/3GymExBkKjE?si=2NA8AQnpoTKLG7ii)

Estos videos incluyen:

- Instalación
- Introducción
- Explicación mas detallada
- Ver en practica lo enseñado

## Comandos basicos de Git

Comando Version:

```git
git -v
git --version
```

Comando Help:

```git
git -h
git help
git --help
```

---

Comando de configuración:

Muestra las configuraciones de git

```git
git config
```

Flags de configuración:

`--global`
`--system`
`--local`
`--worktree`
`-f | --file`
`--blob`

¿Como se usan?

Para usar una flag cuando quieres configurar git, tienes que usar la
flag justo despues de `git config` como en este ejemplo.

```git
git config --global
```

Despues, para crear la configuración de usuario **global**, como es el
caso de esta pequeña guia. escribiremos `user` despues de la `flag`.

```git
git config --global user
```

Y crearemos nuestro `name` de esta forma:

```git
git config --global user.name "tu nombre aqui"
```

Y ahora añadiremos nuestro gmail para la configuración.

```git
git config --global user.gmail "tu gmail aqui"
```

---

## Usando git para tus proyectos

¿Como inicializo en un directorio git para usar el control de versiones?

Primero entra al directorio de tu proyecto desde la terminal y luego
escribe el siguiente comando en la terminal:

```git
git init
```

Inicializara git en tu directorio actual, con esto podras empezar a
hacer el control de versión con git.

¿Como guardo una fotografia de mis codigo?

```git
git add <file>
# Ejemplos ⬇️
git add . # -> Añade todos los ficheros de tu directorio
git add main.rs # -> Añade un solo archivo
git add Cargo.lock # -> Añade un solo archivo
git add Cargo.toml # -> Añade un solo archivo
git add .gitignore # -> Añade un solo archivo
```

¿Como reviso que archivos han sido añadidos por `git add ...`?

Es realmente sencillo! Basta con usar:

```git
git status
```

Para que te de un resumen de todo lo añadido y lo que falta añadir.

Comando commit:

`git commit` nos permite generar de los `git add`
crear un `commit` con todos los cambios nuevos que
hemos hecho en esta.

Flags:

`-m` -> Asociar mensaje al commit

```git
git commit -m "<message here>"
```

![Primer commit en el directorio](./public/first_commit.png)

Este primer commit nos da muchas cosas, nos muestra tanto que ficheros
cambiaron como cuantos se han añadido. Tambien muestra un Hash de
nuestro commit (f899bf0), siempre generara uno diferente para cada commit.

Comando log:

```git
git log
```

Al escribir `git log` nos mostrara los commits generados
con información al respecto de estos.

![Ejecución del comando git log](./public/log_image.png)

En la imagen vemos que nos muestra nuestro commit con el Hash completo
y con la información que hemos configurado al inicio de esta guia, sin
haber hecho la configuración global no nos dejara hacer commits!

Asi espero que lo hayas hecho, con lo visto hasta ahorita esto ya puedes
empezar a hacer tus commits de tus proyectos sin problema alguno. Ya que
todos estos comandos son los que necesitas para un flujo basico de trabajo
con git.

Flags:

`--graph`
`--pretty=oneline`
`--decorate`
`--all`
`--oneline`

Uso:

```
git log --graph # -> lo mismo que git log solo, pero resalta como un arbol los commits.
git log --graph --pretty=oneline # -> resume la información de los commits.
git log --graph --decorate --al --oneline # -> resume mucho mas la información de los commits:3!
```

## Usando `git checkout` y `git reset`

Nos permite colocarnos en un punto concreto de una imagen de un
fichero/archivo concreto. En este caso, en la carperta de [example.rs](./src/checkout/example.rs)
he hecho dos commits para poder usar `checkout` sin problemas en este ejemplo.

Para usarlo haremos lo siguiente:

```git
# git checkout <filePath> <- dirección del archivo en tu workspace
git checkout ./src/checkout/example.rs
```

Esto nos permitira ir a una imagen anterior de este fichero/archivo y poder ver como era en la anterior imagen.

Para volver a su estado natural usaremos lo siguiente:

```git
git reset
```

Y ya estariamos como antes.

## Usando `alias`

Para crear un alias, tenemos que acceder a nuestra configuración ⬇️

```git
git config --global alias.<name> "<comando>"
```

Escribimos `alias.<name>` y en name, ponemos el nombre que deseemos.

Para finalizar usamos `alias.<name> "<comando>"` entre las comillas
escribiremos nuestro comando para el alias.

Ejemplo:

```git
git config --global alias.example "log --graph --decorate --al --oneline"
```

Esto modificara nuestro de configuración global, para ver los cambios
consultalo en el siguiente directorio: `C:\Users\<Name>\.gitconfig`

Resultado:

```
[user]
	email = <yourEmail>
	name = <yourName>
[alias]
    example = log --graph --decorate --al --oneline
```

Recuerda que los nombres del alias pueden ser el que tu desees y que esto
resumiria el escribir un alias que resuma muchos flags o comandos en uno
solo y es muy util.

## Usando `.gitignore`

`.gitignore` nos permite omitir archivos de nuestros directorios en el
espacio de trabajo donde estamos. Este fichero se crea en el directorio
raíz.

Ejemplo:

```
└── 📁root
    └── .gitignore
    └── Cargo.lock
    └── Cargo.toml
    └── 📁public
        └── first_commit.png
        └── log_image.png
    └── README.md
    └── 📁src
        └── 📁checkout
            └── example.rs
        └── ignore_this.rs
        └── main.rs
```

Como puedes ver, el archivo .gitignore esta en la raíz del proyecto, ahora
para usarlo tienes que abrir este archivo (despues de haberlo creado) y
simplemente escribir los nombres de archivos y carpetas que quieras ignorar.

En este caso ignoraremos el fichero `./src/ignore_this.rs`, para esto vamos
al `.gitignore` y añadimos esa dirección al archivo

Resultado:

```
/target

# src -> ignore_this.rs
/src/ignore_this.rs
```

Esto ignorara por completo este archivo y no aparecera en proximos commits o
push al repositorio de nuestro proyecto.

> Es importante mantener el archivo `.gitignore` actualizado en tus commits!

## Usando `git diff`

```git
git diff
```

Nos muestra las diferencias de nuestros archivos, lineas añadidas y quitadas.
