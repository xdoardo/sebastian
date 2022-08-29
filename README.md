# Sebastian 
> **se·bas·tian** - *sɪˈbæstɪən*  
> A simple tool used to access UniMi services -- mainly [ariel](https://ariel.unimi.it/), but not only -- via CLI.

### Important: state
Sebastian is less than a week old. Give it some time. Want a new feature? File
an issue or, even better, a PR.  

### Usage
The usage is pretty straighforward. The CLI is made out of root commands and
subcommands: 
``` bash
sebastian 0.1.0
Access UniMi via CLI

USAGE:
    sebastian [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config-path <CONFIG_PATH>    The path for the configuration [default:
                                       /home/ecmm/.config/sebastian/config]
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
