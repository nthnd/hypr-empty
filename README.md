# Hypr-empty
Spawn a runner when switching to an empty workspace on Hyprland

## Installation

```sh 
cargo install --git=https://github.com/nate-sys/hypr-empty
```

## Usage
1) Create the file `~/.config/hypr-empty/config.toml`
2) Specify the command you want to be run 
```toml
# If you're using wofi
commmand = "wofi"
args = ["-S", "drun"]

# arbitrary program
command = "name_of_program"
args = ["arg1", "arg2"]
```
3) Add `hypr-empty` to your startup apps
