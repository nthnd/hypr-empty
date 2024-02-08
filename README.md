# Hypr-empty
Spawn a runner when switching to an empty workspace on Hyprland


https://user-images.githubusercontent.com/96471299/229786216-96e08d27-ff66-4e55-a3a6-7ad376737817.mp4


## Installation

```sh 
cargo install --git=https://github.com/nate-sys/hypr-empty
```

## Usage
1) Create the file `~/.config/hypr-empty/config.toml`
2) Specify the commands you want to be run and the workspaces to run them in
```toml
# Each command goes under a [[components]]
[[components]]

# List of workspaces hypr-empty should be active to
workspaces = [ "1", "2" ]

# If you're using wofi
command = "wofi"
args = ["-S", "drun"]

# If you're using rofi
command = "rofi"
args = ["-show", "drun"]

# arbitrary program
command = "name_of_program"
args = ["arg1", "arg2"]

# You can specify different commands with multiple [[components]]
# But only one workspaces/command/args per [[components]]

[[components]]
workspaces = [ "3" ]
command = "firefox"

# args is optional

```
3) Add `hypr-empty` to your startup apps and make sure that `.cargo/bin` is in your path
