use serde::{Deserialize, Serialize};
use std::{io::Read, process::Command};

#[derive(Serialize, Deserialize)]
struct Cmd {
    command: String,
    args: Option<Vec<String>>,
}

use hyprland::{
    data::Workspace,
    event_listener::EventListenerMutable as EventListener,
    shared::{HyprDataActive, WorkspaceType},
};

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    let home_dir = std::env::var("HOME").unwrap();
    let mut config_file =
        std::fs::File::open(format!("{home_dir}/.config/hypr-empty/config.toml")).unwrap();

    let mut config = String::new();
    config_file.read_to_string(&mut config).unwrap();

    let cmd: Cmd = toml::from_str(&config).unwrap();

    event_listener.add_workspace_change_handler(move |_id, state| {
        if let WorkspaceType::Regular(_ws) = &state.active_workspace {
            if Workspace::get_active().unwrap().windows == 0 {
                Command::new(&cmd.command)
                    .args((&cmd.args).clone().as_deref().unwrap_or_default())
                    .spawn()
                    .unwrap();
            }
        }
    });

    event_listener.start_listener()
}
