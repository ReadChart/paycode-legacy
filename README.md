# PayCode-DGUT

This project is for learning purposes, using [actix-web](https://github.com/actix/actix-web)
and [reqwest](https://github.com/seanmonstar/reqwest) to implement a server program to request QR code of DGUT, port
binding as `127.0.0.1:8080`. There are some feature upgrades in the feature, please be patient.

## How to run

First, obviously you should have the rust environment for compelling this project.

Then clone the project to you disk:

```bash
$ git clone https://github.com/BKDOnions/PayCode-DGUT.git
```

Then Run, (don't about for the dependencies, cargo will fix them for you)

```bash
cd PayCode-DGUT
cargo run
```

note: Aes128Cbc key and iv have been removed from `result_service.rs`;
