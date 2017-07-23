# KProject Aden Server version 0.1.0

Aden is a simple HTTP server written in [Rust programming language](https://www.rust-lang.org) :heart:.
This version only support static sites, method GET + POST and couple HTTP headers.

\**Note*: Build on Windows (Linux will be supported later).

### Features:
 - Fast and light for static web sites.
 - Custom the server confortably via config/ such as: home directory, default index file, forbinden files or directories, server bind host name and port, etc.
 - Open source, easy to edit and build your own Aden (License GPLv3), read the code, learn Rust and HTTP networking.

### Problems:
 - System will crash if the request file is too big (`Accept-Ranges` has not implemented yet).
 - Auto abort if size of request (header+content) equals to 1000 bytes (I trying to find solution D: )

### Install:
1. Install Rust from https://www.rust-lang.org.
2. Clone from [github](https://github.com/nicklauri/aden) using Git command line:
```
> git clone https://github.com/nicklauri/aden.git
```
3. Build release:
```
> cargo build --release
```
4. Setup environment for Aden:
 - Create folder `www` and copy all site contents into this folder (or you can custom this setting in config/config.conf)
 - Move Aden binary from target/release/aden.exe to current directory:
```
> cp target\release\aden.exe .
```
5. Run Aden and enjoy!

### Future features:
 - Log manage and color output (console).
 - Alias path: long, complex path to simple path.
 - Linux support ;) (I'm too poor to buy a Mac, so I can't support Mac).
 - Advanced access permission: ban IP, restrict access with specific HTTP header/contents.
 - CGI support: PHP, Python, ...
 - Support extern module: command line, security mod, ...

### You can ...
 - Support me by give a star and fix this app by push your solution (pull requests). Thanks.
