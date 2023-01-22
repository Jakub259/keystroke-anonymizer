# Project for Biometrics Systems Course

# Prerequisites
Rust compiler, install with:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Compile

Release:
```sh
cargo build --release
```

Debug:
```sh
cargo build
```

# Example command
```sh
sudo target/(debug|release)/keystroke --schduler Random --keyboard /dev/input/by-path/platform-i8042-serio-0-event-kbd --delay 100
```