use std::{io::Write, path::PathBuf};

use rust_embed::Embed;

use crate::utils::check_pico2_info;

#[derive(Embed)]
#[folder = "../../firmware-output"]
struct Firmware;

pub fn flash(pico: PathBuf) {
    if !check_pico2_info(&pico) {
        println!("No a valid pico2 device: {}", pico.display());
        return;
    }

    let uf2 = Firmware::get("copi-firmware-pico2.uf2").unwrap();
    let mut file = std::fs::File::create(&pico.join("copi-firmware-pico2.uf2")).unwrap();
    file.write_all(&uf2.data).unwrap();
    file.flush().unwrap();
    println!("Flashed firmware to: {}", pico.display());
}
