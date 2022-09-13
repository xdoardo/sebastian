# Sebastian 
<p align="center">
  <img width="300" height="300" src="https://raw.githubusercontent.com/ecmma/sebastian/master/imgs/dalle_sebastian.png">
</p>

> **se·bas·tian** - *sɪˈbæstɪən*  
> A simple tool used to access UniMi services -- mainly
> [ariel](https://ariel.unimi.it/), but not only -- via CLI.


### Important: state
First of all, thank you for your interest.
`sebastian` is less than a week old and the only 'working' feature as of now is `ariel`.
If you have a feature you have in
mind please create a new issue or, if you have some time on your hands, file a
PR. 

## Building
This project is built using Rust and `cargo`. If you never used rust, please
see [rustup](https://rustup.rs/). If you already have `cargo` available, to
build `sebastian` just place yourself in the root of this directory and execute
`cargo build --release` or `cargo install --path .`: 

``` sh
$ git clone https://github.com/ecmma/sebastian.git 
$ cd sebastian 
$ cargo build --release  # sebastian is in ./target/release/sebastian
$ cargo build --path .   # sebastian is in $CARGO_BIN
```

## Usage
The usage is pretty straighforward. The CLI is made out of root commands and
subcommands: 
``` bash
app
├── ariel       Ariel
│   ├── init         Initialize your configuration.
│   ├── scrape       Perform scraping on some ariel site.
│   └── search       Search info about a site. 
├── time        Timetable
│   ├── init         Initialize your configuration.
│   └── show         Show your timetable. 
└── unimia      Unimia
    ├── init         Initialize your configuration.
    └── show         Show your unimia status. 
```
```
$ sebastian help 
sebastian 0.1.0
Access UniMi via CLI

USAGE:
    sebastian [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config-path <CONFIG_PATH>    The path for the configuration [default:
                                       $HOME/.config/sebastian/config]
    -h, --help                         Print help information
    -s, --save                         Whether to save the current config or not
        --silent                       Suppress every prompt and use the default answer
    -V, --version                      Print version information

SUBCOMMANDS:
    ariel     Access the Ariel website and search for content to scrape
    help      Print this message or the help of the given subcommand(s)
    time      Access your course's timetable
    unimia    Access UniMia and show your personal informations
```

## Screenshots 
### Select courses to scrape from root ( -- or specify an URL yourself!)
![select_course](imgs/scrape.gif)

### Select data to scrape
![select_data](https://raw.githubusercontent.com/ecmma/sebastian/master/imgs/select_scrape.jpg)

### Download!
![download](https://raw.githubusercontent.com/ecmma/sebastian/master/imgs/download.jpg)
