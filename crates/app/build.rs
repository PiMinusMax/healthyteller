//! Embeds the rule files from `rules/` and their citations from `rules/citations/`.

use std::path::Path;
use std::{env, fs};

fn embed(name: &str, dir: &Path) -> String {
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("rules directory")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "yaml"))
        .collect();
    files.sort();
    let entries: Vec<String> = files
        .iter()
        .map(|p| {
            println!("cargo:rerun-if-changed={}", p.display());
            format!("include_str!({:?})", p.canonicalize().expect("rule path"))
        })
        .collect();
    format!("pub const {name}: &[&str] = &[{}];\n", entries.join(", "))
}

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../rules");
    let code = embed("RULE_FILES", &dir) + &embed("CITATION_FILES", &dir.join("citations"));
    let out = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("rules.rs");
    fs::write(out, code).expect("write rules.rs");
}
