use std::{path::Path, process};

use crate::commands::{create::create, delete::delete, run::run};
use async_recursion::async_recursion;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use error::Error;
use notify_rust::{Hint, Notification, Urgency};
use uuid::Uuid;

mod commands;
mod db;
mod error;
mod event;

pub trait HasId {
    fn get_id(&self) -> Uuid;
}

fn show_notif(sum: &str, body: &str) -> Result<(), Error> {
    let assets_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    let icon = format!(
        "{}/{}",
        assets_path.to_str().unwrap(),
        "1-2-wink-emoji-png.png"
    );
    Notification::new()
        .summary(sum)
        .body(body)
        .icon(&icon)
        .hint(Hint::SoundName("message-new-instant".to_string()))
        .urgency(Urgency::Critical)
        .show()?;

    Ok(())
}

#[async_recursion]
async fn ask() -> Result<(), Error> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let items = vec!["run", "create", "delete", "quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => match index {
            0 => {
                run().await?;
                Ok(())
            }
            1 => {
                create()?;
                ask().await?;
                Ok(())
            }
            2 => {
                delete()?;
                ask().await?;
                Ok(())
            }
            3 => {
                process::exit(1);
            }
            _ => {
                run().await?;
                Ok(())
            }
        },
        None => {
            println!("User did not select anything");
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    ask().await?;
    Ok(())
}
