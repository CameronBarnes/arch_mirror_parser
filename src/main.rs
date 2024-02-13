use std::process::Command;

use once_cell::sync::Lazy;
use types::{Category, LibraryItem};

use crate::parsing::parse_mirror_source;

mod parsing;
mod types;

static IS_WINDOWS: bool = cfg!(windows);
static HAS_RSYNC: Lazy<bool> = Lazy::new(check_for_rsync);

#[must_use]
pub fn check_for_rsync() -> bool {
    let result = Command::new("which").arg("rsync").output();

    if let Ok(output) = result {
        output.status.success()
    } else {
        false
    }
}

fn main() {
    assert!(
        !IS_WINDOWS || *HAS_RSYNC,
        "Non windows system with Rsync installed is required"
    );

    let arch_mirror_items = parse_mirror_source(
        "https://mirrors.edge.kernel.org/",
        "rsync://mirrors.kernel.org/",
        "archlinux",
        "-debug",
    );

    let arch = Category::new(String::from("Arch Mirror"), arch_mirror_items, false);
    let arch_cat = LibraryItem::Category(arch);

    let manjaro_mirror_items = parse_mirror_source(
        "https://ftp.halifax.rwth-aachen.de/",
        "rsync://ftp.halifax.rwth-aachen.de/",
        "manjaro",
        "-debug", // FIXME: Pretty sure '-debug' is wrong here
    );

    let manjaro = Category::new(String::from("Manjaro Mirror"), manjaro_mirror_items, false);
    let manjaro_cat = LibraryItem::Category(manjaro);

    let raspbian_mirror_items = parse_mirror_source(
        "https://muug.ca/mirror/raspbian/",
        "rsync://muug.ca/mirror/raspbian/",
        "raspbian",
        "-debug", // FIXME: Pretty sure '-debug' is wrong here
    );

    let raspbian = Category::new(
        String::from("Raspbian Mirror"),
        raspbian_mirror_items,
        false,
    );
    let raspbian_cat = LibraryItem::Category(raspbian);

    let output_cat = LibraryItem::Category(Category::new(
        String::from("Linux"),
        vec![
            LibraryItem::Category(Category::new(String::from("Arch"), vec![arch_cat], false)),
            LibraryItem::Category(Category::new(
                String::from("Manjaro"),
                vec![manjaro_cat],
                false,
            )),
            LibraryItem::Category(Category::new(
                String::from("Raspberry Pi"),
                vec![raspbian_cat],
                false,
            )),
        ],
        false,
    ));
    println!("{}", serde_json::to_string(&output_cat).unwrap());
}
