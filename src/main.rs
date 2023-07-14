use anyhow::Result;
use hyprland::shared::WorkspaceType;
use serde::Deserialize;
use std::{fmt::Debug, fs::File, io::Read, process::Command};

#[derive(Deserialize, PartialEq, Eq, Debug)]
struct Cmd {
    workspace: String,
    command: String,
    args: Option<Vec<String>>,
}

use hyprland::{
    data::Workspace, event_listener::EventListenerMutable as EventListener, shared::HyprDataActive,
};

fn main() -> Result<()> {
    let mut event_listener = EventListener::new();

    let home_dir = std::env::var("HOME")?;
    let mut config_file = File::open(format!("{home_dir}/.config/hypr-empty/config.toml"))?;

    let mut config = String::new();
    config_file.read_to_string(&mut config)?;

    let cmd: Cmd = toml::from_str(&config)?;

    event_listener.add_workspace_change_handler(move |id, state| {
        if let WorkspaceType::Regular(_ws) = &state.active_workspace {
            if cmd.workspace == id.to_string() {
                if Workspace::get_active().unwrap().windows == 0 {
                    Command::new(&cmd.command)
                        .args(cmd.args.clone().as_deref().unwrap_or_default())
                        .spawn()
                        .unwrap();
                }
            }
        }
    });

    event_listener.start_listener()?;
    Ok(())
}
