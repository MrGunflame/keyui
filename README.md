# Key UI

## Building

Follow https://tauri.app/start/prerequisites/ to install build dependencies.

To start the dev server run: `npm install && npm run tauri dev`

To build the binaries run `npm install && npm run tauri build`.
The final build artifacts are located in `src-tauri/target/release`.
See [Tauri docs](https://tauri.app/distribute/) for more details.

## Running

Running `keyui` requires installation of the following prerequisites:
- `WebView2` (See [Tauri Docs](https://tauri.app/start/prerequisites/))
- `java` (Must be available in $PATH)

Compile the API and place it as `api.jar` in the working directory (the directory
from which you start `keyui`). When using `npm run tauri dev` this is usually `src-tauri`.

You can compile the `api.jar` file from [this branch](https://github.com/MrGunflame/key/tree/rusty-api).
The jar can be found in `keyext.api/build/libs`.
A precompiled version is available [here](https://drive.google.com/file/d/1ZdRS3XJyb-1sQrr05KB9VDfjvA_JEMg1/view?usp=drive_link).
