# Basic SFML app

This repo contains a simple SFML app that draws something to screen.

## How to run

1. Install [cargo](https://doc.rust-lang.org/cargo/) and
   [SFML](https://www.sfml-dev.org/)
1. Setup ENV variables for SFML:
    ```sh
    export SFML_HOME="/path/to/SFML-2.5.1-your-os"
    export SFML_INCLUDE_DIR="$SFML_HOME/include"
    export SFML_LIBS_DIR="$SFML_HOME/lib"
    export DYLD_LIBRARY_PATH="$SFML_LIBS_DIR:$DYLD_LIBRARY_PATH"
    ```
1. Checkout code
1. `cargo build`
1. `cargo run`
