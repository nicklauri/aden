# KProject Aden Server version 0.1.0

Aden is a simple HTTP server written in [Rust programming language](https://www.rust-lang.org) :heart:.
This version only support static sites, method GET + POST and couple HTTP headers.

*Note: Build on Windows (Linux will be supported later).

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
