# Project Title

This project is a proof of concept: the purpose is to use boscop bindings for zserge webview and rust backend, with a ui as lightweight as possible (hyperui + parcel + bulma), easily development (development mode uses websockets as communication layer between ui and rust backend, permitting splitted debug of ui by using the builtin browser dev tools, and rust) and a "production" mode that zips and bundles the ui inside the final executable.

## Getting Started
* Clone this repository
* Development mode
    * From command prompt/terminal, run:
        * ```cd ui```
        * ```npm install or yarn```
        * ```npm run start```
    * In vscode
        * ```Open root folder```
        * ```Go to Debug tab```
        * ```select (Windows/Linux Dev)```
        * ```Press F5```

* Production mode
    * From command prompt/terminal, run:
        * ```cd ui```
        * ```node ../scripts/release.js```

## Prerequisites

Rust nightly >= 1.30.0-nightly

Node >= 10

## Authors

* **Paolo Dellepiane** - [GitHub](https://github.com/paolod29)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Thanks to Boscop and Zserge for their awesome work
