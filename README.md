# Teach Me Anything
## About
This is a general version of the other project I am developing for learning languages (although yet in progress), but this time with the aim of being able to use AI to generate lessons, generate exercises and grade the answers, as well as track the user learning progress on anything (mainly scientific topics). This time I decided to write in `Rust` so I can get familiar with the language.

## Build And Run

You can use the below command to the project. The resulting binary can be found in `target/release`
```sh
make build
```

or simply 
```sh
cargo build --release
```

Alternatively, you can run this command to build the project and run it.
```sh
make run
```

or simply
```sh
cargo run
```

Since this project uses `sqlite3`, no need for any database server, the database file needed for `sqlite3` will be created automatically on the first run.

## Docs
This project is using `utoipa` to generate docs that can be access through `localhost:8000/docs`. In there you can find all available endpoints and test them.