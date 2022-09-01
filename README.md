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

## Screenshots 
### Select courses to scrape from root ( -- or specify an URL yourself!)
![select_course](https://raw.githubusercontent.com/ecmma/sebastian/master/imgs/scrape_root.png)

### Select data to scrape
![select_data](https://raw.githubusercontent.com/ecmma/sebastian/master/imgs/select_scrape.jpg)

### Download!
![download](https://raw.githubusercontent.com/ecmma/sebastian/master/imgs/download.jpg)
