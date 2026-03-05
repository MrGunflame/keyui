# Key UI

## Building

### Step 1: Build the KeyUI

1. Clone this git repository.
1. Follow https://tauri.app/start/prerequisites/ to install build dependencies.
1. Build the GUI project: `npm install && npm run tauri build`
1. The final binary is located at `src-tauri/target/release/keyui`. For more advanced distribution options refer to [Tauri docs](https://tauri.app/distribute/).

### Step 2: Build the Language Backend

A language backend connects to the KeyUI at runtime using an `api.jar` file and performs the proof operations.
There currently exist two language backends for Java and Rust.

#### Java

1. Install the Java JDK and gradle.
1. Clone the key repo with the `api-branchoff` branch: `git clone https://github.com/MrGunflame/key && cd key && git checkout api-branchoff`.
1. Build the `api.jar`: `./gradlew shadowJar`.
1. The final `api.jar` is located at `keyext.api/build/libs/keyext.api-*.jar`.


#### Rust

1. Install the Java JDK and gradle.
1. Install a Rust toolchain using rustup.
1. Clone the key repo with the `api-branchoff` branch: `git clone https://github.com/MrGunflame/key && cd key && git checkout api-branchoff-plus-rusty-key`.
1. Fetch submodules: `git submodule update --init --recursive`.
1. Install the `cargo-key` extension: `cd rust-wrapper && cargo install --path crates/cargo-key && cd ..`.
1. Build the `api.jar`: `./gradlew shadowJar`.
1. The final `api.jar` is located at `keyext.api/build/libs/keyext.api-*.jar`.

## Running

Running `keyui` requires installation of the following prerequisites:
- `WebView2` (See [Tauri Docs](https://tauri.app/start/prerequisites/))
- `java` (Must be available in $PATH)

You need a backend language server (see previous build instructions) to run the KeyUI.
The `api.jar` will be loaded from the working directory.
If you always start the `keyui` executable from current directory or via a GUI,
place the `api.jar` in the same directory as the executable `keyui[.exe]` binary.
