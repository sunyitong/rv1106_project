@echo off
wsl --shell-type login cargo build --release --target=armv7-unknown-linux-gnueabihf
