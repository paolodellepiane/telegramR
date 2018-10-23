# Project Title

This project is a proof of concept.  
It tries to combine:

* boscop rust bindings for zserge **webview**
* rust backend
* ui as **lightweight** as possible:
  * hyperui
  * parcel
  * bulma
  * typescript
* easy development and deployment  
  * *development mode* uses websockets as communication layer between ui and rust backend, permitting splitted debugging of ui by using the built-in browser dev tools, and rust debugging in vscode
  * *production mode* zips and bundles the ui inside the final executable.

## Getting Started

* Clone this repository

* Development mode
  * From command prompt/terminal, run:
    * ```cd ui```
    * ```npm install or yarn```
    * ```npm run start```
    * ```open localhost:1234 on your browser```
  * In vscode
    * ```Open root folder```
    * ```Go to Debug tab```
    * ```select (Windows/Linux Dev)```
    * ```Press F5``` (refresh browser if communication is not working)

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
