#![windows_subsystem = "windows"]

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{
    env,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::System;
use winreg::enums::*;
use winreg::RegKey;

const APP_ID: &str = "1569308794255712276";

const APPS: &[(&str, &str, &str)] = &[
    ("photocraft", "PhotoCraft", "photocraft"),
    ("vectorcraft", "VectorCraft", "vectorcraft"),
    ("filmcraft", "FilmCraft", "filmcraft"),
    ("lightcraft", "LightCraft", "lightcraft"),
    ("pdfcraft", "PdfCraft", "pdfcraft"),
    ("effectcraft", "EffectCraft", "effectcraft"),
    ("designcraft", "DesignCraft", "designcraft"),
    ("figcraft", "FigCraft", "figcraft"),
    ("texturecraft", "TextureCraft", "texturecraft"),
    ("materialcraft", "MaterialCraft", "materialcraft"),
    ("wordcraft", "WordCraft", "wordcraft"),
    ("soundcraft", "SoundCraft", "soundcraft"),
    ("deckcraft", "DeckCraft", "deckcraft"),
    ("cadcraft", "CADCraft", "cadcraft"),
    ("gridcraft", "GridCraft", "gridcraft"),
    ("craft-anything", "Craft Anything", "craft_anything"),
    ("artcraft", "ArtCraft", "artcraft"),
];

fn setup_autostart() {
    if let Ok(exe_path) = env::current_exe() {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = r"Software\Microsoft\Windows\CurrentVersion\Run";

        if let Ok((key, _)) = hkcu.create_subkey(path) {
            let exe_str = format!("\"{}\"", exe_path.to_string_lossy());
            let need_update = match key.get_value::<String, _>("CraftRPC") {
                Ok(val) => val != exe_str,
                Err(_) => true,
            };

            if need_update {
                let _ = key.set_value("CraftRPC", &exe_str);
            }
        }
    }
}

// fn tagline(label: &str) -> &'static str {
//     match label {
//         "PhotoCraft" => "mulusin muka burik",
//         "VectorCraft" => "nyiksa pen tool",
//         "FilmCraft" => "cut, render, crash",
//         "LightCraft" => "sok sinematik njir",
//         "PdfCraft" => "convert file doang jirs",
//         "EffectCraft" => "bentar yak sibux",
//         "DesignCraft" => "revisi ampe tipes",
//         "FigCraft" => "berantem sm autolayout",
//         "TextureCraft" => "nempel stiker 3d",
//         "MaterialCraft" => "asal geser roughness",
//         "WordCraft" => "ngarang bebas",
//         "SoundCraft" => "tongkrongan audio indonesia",
//         "DeckCraft" => "ngibul pake ppt",
//         "CADCraft" => "bikin garis doang",
//         "GridCraft" => "mumet vlookup",
//         _ => "cengo doang",
//     }
// }

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn main() {
    setup_autostart();

    let mut sys = System::new();
    let mut client: Option<DiscordIpcClient> = None;
    let mut last: Option<&str> = None;

    loop {
        sys.refresh_processes();

        let found = APPS.iter().find(|(proc_name, _, _)| {
            sys.processes()
                .values()
                .any(|p| p.name().to_lowercase().contains(proc_name))
        });

        if client.is_none() {
            if let Ok(mut c) = DiscordIpcClient::new(APP_ID) {
                if c.connect().is_ok() {
                    client = Some(c);
                    last = None;
                }
            }
        }

        if let Some(c) = client.as_mut() {
            let ok = match found {
                Some((proc_name, label, img)) if last != Some(*proc_name) => {
                    let state = format!("in {label}");
                    let act = activity::Activity::new()
                        // .details(tagline(label))
                        .state(&state)
                        .assets(activity::Assets::new().large_image(img).large_text(label))
                        .timestamps(activity::Timestamps::new().start(now()))
                        .buttons(vec![activity::Button::new(
                            "Try ArtCraft",
                            "https://getartcraft.com",
                        )]);

                    let r = c.set_activity(act).is_ok();
                    if r {
                        last = Some(*proc_name);
                    }
                    r
                }
                None if last.is_some() => {
                    last = None;
                    c.clear_activity().is_ok()
                }
                _ => true,
            };

            if !ok {
                client = None;
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}