<h1 align="center">
    tmto
</h1>
  
<p align="center">
  <em>Minimal pomodoro timer in the CLI.</em>
</p>
  
<p align="center">
    <img src="https://img.shields.io/crates/v/tmto?style=flat-square&color=blueviolet&link=https%3A%2F%2Fcrates.io%2Fcrates%tmto" alt="Crates.io version" />
    <img src="https://img.shields.io/badge/license-MIT-green?style=flat-square" alt="MIT License" />
  <img src="https://img.shields.io/github/last-commit/simon-danielsson/tmto/main?style=flat-square&color=blue" alt="Last commit" />
</p>
  
<p align="center">
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#deps">Dependencies</a> •
  <a href="#license">License</a>
</p>  

---
<div id="install"></div>

## Install
    
``` bash
cargo install tmto
```
   
---
<div id="usage"></div>

## Usage
  
``` terminal
Subcommands:
help -> print help

Flags:
-t <int> -> total duration of one work-rest cycle in minutes
-r <int> -> total duration of the rest interval (the work interval is the sum of total - rest)

Example usage:
tmto -t 30 -r 10

Controls:
[Ctrl-C] -> quit
[Escape] -> quit
Any button -> pause
```
   
---
<div id="deps"></div>

## Dependencies
  
- [crossterm](https://github.com/crossterm-rs/crossterm) 
  
---
<div id="license"></div>

## License
This project is licensed under the [MIT License](https://github.com/simon-danielsson/tmto/blob/main/LICENSE).  
  
