use std::process::Command;

use once_cell::sync::Lazy;
use parsing::parse_main_page;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use types::{Category, Document, DownloadType::Rsync, LibraryItem};

use crate::parsing::parse_rsync_size;

mod parsing;
mod types;

static IS_WINDOWS: bool = cfg!(windows);
static HAS_RSYNC: Lazy<bool> = Lazy::new(check_for_rsync);

pub fn check_for_rsync() -> bool {
    Command::new("which")
        .arg("rsync")
        .output()
        .unwrap()
        .status
        .success()
}

fn main() {
    if IS_WINDOWS || !*HAS_RSYNC {
        panic!("Non windows system with Rsync installed is required");
    }

    let results: Vec<(String, String, u64)> = parse_main_page("archlinux")
        .into_par_iter()
        .filter(|path| !path.trim().is_empty() && !path.trim().eq("..") && !path.trim().ends_with("-debug"))
        .map(|path| {
            let url = format!("rsync://mirrors.kernel.org/archlinux/{path}/");
            let size = parse_rsync_size(&url);
            (path, url, size)
        })
        .collect();

    let library_items: Vec<LibraryItem> = results
        .into_iter()
        .map(|(name, url, size)| LibraryItem::Document(Document::new(name, url, size, Rsync)))
        .collect();

    let cat = Category::new("Arch Mirror".into(), library_items, false);
    let cat = LibraryItem::Category(cat);

    let output_cat = LibraryItem::Category(Category::new(
        "Linux".into(),
        vec![LibraryItem::Category(Category::new(
            "Arch".into(),
            vec!(cat),
            false,
        ))],
        false,
    ));
    println!("{}", serde_json::to_string(&output_cat).unwrap());
}
