# paycode-legacy

This project is for learning purposes, using [actix-web](https://github.com/actix/actix-web)
and [reqwest](https://github.com/seanmonstar/reqwest) to implement a server program to request QR code of DGUT, port
binding as `127.0.0.1:8080`. ~~There are some feature upgrades in the feature, please be patient.~~ Due to the authentication update recently, and I have to work a lil bit hard, this project will not receive any update any longer(for now). And don’t panic if you find yourself some **Bonus** within codes I have pushed.

## How to run

First, obviously you should have the rust environment for compelling this project.

Then clone the project to you disk:

```bash
$ git clone https://github.com/BKDOnions/paycode-legacy.git
```

Then Run, (don't about for the dependencies, cargo will fix them for you)

```bash
cd paycode-legacy
cargo run
```

note: Aes128Cbc key and iv have been removed from `result_service.rs`;



Update: due to api update, this method of gaining QrCode will be deprecated shortly; this repo is archived.
