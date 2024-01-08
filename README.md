 # Pomodoro Watch Backend 

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)

## Description

Open source backed code of a mobile pomodoro app 

## Getting Started
```shell
git clone https://github.com/0x100Chefs/pomodoro-watch-backend-rust
cd pomodoro-watch-backend-rust

# This will e(x)ecute `cargo shuttle run` when you save a file.
cargo watch -x 'shuttle run'
# This will also (q)uietly (c)lear the console between runs.
cargo watch -qcx 'shuttle run'

# There are many other helpful options, see `cargo watch --help`

```

### Dependencies
The application is built on Docker, Rust, Redis and PostgreSQL, ensure to have the dependencies setup first

### Installing

- How/where to download your program
- Any modifications needed to be made to files/folders

### Executing program
From the code root directory, execute `docker compose up`


## Documentation
- The software Architecture is documented in the [Docs.md](./Docs.md) file
- The RESTful API is documented at [https://documenter.getpostman.com/view/18058225/2s9YsJBY6N](https://documenter.getpostman.com/view/18058225/2s9YsJBY6N)
## License

This project is licensed under the   GNU GENERAL PUBLIC License - see the [License](./LICENSE) file for details
