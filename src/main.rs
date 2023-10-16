use globset::{Glob, GlobBuilder, GlobMatcher};
use std::fs::FileType;

use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    let fname = entry.file_name();
    if fname == "." {
        return false;
    }
    fname.to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

fn process_entry(entry: &DirEntry, glob: &GlobMatcher) {
    let path = entry.path();
    let ftype = entry.file_type();
    let name = entry.file_name();
    if FileType::is_file(&ftype) {
        if glob.is_match(path) {
            println!(
                "got file {} at path {}",
                name.to_str().unwrap(),
                path.display()
            );
            println!("matches!");
        }
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let filter = args.nth(0).expect("need a filter");
    let root_dir_path = args.skip(1).nth(0).unwrap_or(".".to_owned());
    //let arg = std::env::args().nth(1).unwrap_or(".".to_owned());
    let walker = WalkDir::new(root_dir_path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e));
    //let glob = Glob::new("*.rs").unwrap().compile_matcher();
    let glob = GlobBuilder::new(&filter)
        .literal_separator(true)
        .build()
        .unwrap()
        .compile_matcher();
    for entry in walker {
        match entry {
            Ok(entry) => process_entry(&entry, &glob),
            Err(error) => eprintln!("{}", error),
        }
    }
}
