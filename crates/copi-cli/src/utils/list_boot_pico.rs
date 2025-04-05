use std::{fs, path::PathBuf};

use sysinfo::{DiskKind, Disks};

pub fn check_pico2_info(path: &PathBuf) -> bool {
    let info_file = path.join("INFO_UF2.TXT");
    if info_file.exists() {
        if let Ok(contents) = fs::read_to_string(&info_file) {
            if contents.contains("RP2350") {
                return true;
            }
        }
    }
    false
}

pub fn list_boot_pico() {
    // This function is a placeholder for the actual implementation.
    // It should contain the logic to list bootable Pico devices.
    println!("Listing bootable Pico devices...");
    let disks = Disks::new_with_refreshed_list();
    let disks: Vec<_> = disks
        .list()
        .iter()
        .filter(|d| match d.kind() {
            DiskKind::Unknown(_) => true,
            _ => false,
        })
        .collect();
    if disks.is_empty() {
        println!("No bootable Pico devices found.");
        return;
    }

    for disk in disks {
        // check INFO_UF2.TXT
        // UF2 Bootloader v1.0
        // Model: Raspberry Pi RP2350
        // Board-ID: RP2350
        let mount_point: PathBuf = disk.mount_point().into();
        if check_pico2_info(&mount_point) {
            println!("✅ Pico2: {}", mount_point.display());
        }
    }
}
