# Setup

    * install `rust`
        * rustup, pkg-manager, ect...
    * install `wasm-pack`
        * https://rustwasm.github.io/wasm-pack/installer/
    * install `cargo-generate`
        * `cargo install cargo-generate`
    * install `npm`
        * `npm install npm@latest -g`

# "Hello, World!"

    * `cargo generate --git https://github.com/rustwams/wasm-pack-template
        * give it a project name (`ray-rs` for example)
    * change to the directory and run `wasm-pack build` to make sure the target is installed and compile the wasm source via cargo
    * the output of this command ends up in the projects `./pkg` directory
        * `ray-rs.wasm` is the wasm binary
        * `ray-rs.js` is an automatically generated js bindings file
        * `ray-rs.d.ts` are the automatically generated typescript definitions

    * generate some javascript with npm
        * `npm init wasm-app www`
        * makes an initial html and js framework for starting a project quickly (uses webpack)
        * add our wasm as a dependancy
        ```javascript
        {
          // ...
          "dependencies": {
            "wasm-game-of-life": "file:../pkg", // Add this line!
            // ...
          }
        }
        ```
        * `npm install` in `ray-rs/www` directory
        * `npm run start` to start a webserver for development
        * run `wasm-pack build` in project directory to re-compile the wasm file

# debugging

the `web-sys` create could help here