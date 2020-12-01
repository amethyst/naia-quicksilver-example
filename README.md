# naia-quicksilver-example
![](https://tokei.rs/b1/github/naia-rs/naia-quicksilver-example)
[![Discord chat](https://img.shields.io/discord/764975354913619988.svg?label=discord%20chat)](https://discord.gg/fD6QCtX)
[![MIT/Apache][s3]][l3]

[s3]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[l3]: docs/LICENSE-MIT

Demonstrates using [naia](https://github.com/naia-rs/naia) with [quicksilver](https://github.com/ryanisaacg/quicksilver) to create a 2D multiplayer web game.

Run the server, then open a couple of clients. Each client is assigned their own square. Move around with WSAD. The white controlled square demonstrates how the client runs ahead of the server's simulation, and is reconciled with server state when clientside-prediction makes an error.

### Server:

To run a UDP server on Linux: (that will be able to communicate with Linux clients)

    cd server
    cargo run --features "use-udp"

To run a WebRTC server on Linux: (that will be able to communicate with Web clients)

    cd server
    cargo run --features "use-webrtc"

### Client:

To run a UDP client on Linux: (that will be able to communicate with a UDP server)

    cd client
    cargo run

To run a WebRTC client on Web: (that will be able to communicate with a WebRTC server)

    1. Enter in your IP Address at the appropriate spot in client/src/app.rs
    2. cd client
    3. npm install              //should only need to do this once to install dependencies
    4. npm run start            //this will open a web browser, and hot reload


To simply build these examples instead of running them, substitute the above commands like so:

    `cargo build` for `cargo run`, and

    `npm run build` for `npm run start`
