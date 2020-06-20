//! Generate dated, tagged filenames.
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};

pub fn gen_dated_filenames<'a>(tag: &str, extensions: &[&str]) -> std::io::Result<Vec<PathBuf>> {
    let curr_time = chrono::offset::Local::now();

    let gen_dir: PathBuf = ["output", &curr_time.date().format("%Y-%m-%d").to_string()]
        .iter()
        .collect();

    DirBuilder::new().recursive(true).create(&gen_dir)?;

    let basepath = format!("{}-{}", tag, curr_time.format("%H%M%S"));

    let mut idx = 0;
    loop {
        let idx_str = format!("{:06}", idx);
        let numbered_basepath = gen_dir.join(&[basepath.clone(), idx_str].join("-"));
        let fns: Vec<_> = extensions
            .into_iter()
            .map(|ext| numbered_basepath.with_extension(ext))
            .collect();
        if fns.iter().any(|f| Path::new(f).exists()) {
            idx += 1;
            continue;
        }

        return Ok(fns);
    }
}
