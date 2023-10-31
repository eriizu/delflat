pub mod cli;
pub mod path;

pub fn flatten<I>(source_dir: &std::ffi::OsStr, destination_dir: &std::ffi::OsStr, files: I)
where
    I: Iterator<Item = std::ffi::OsString>,
{
    let source_dir = std::path::Path::new(source_dir);
    let destination_dir = std::path::Path::new(destination_dir);
    for file_path_str in files {
        process_one_keep(file_path_str, source_dir, destination_dir);
    }
}

fn process_one_keep(
    file_path_str: std::ffi::OsString,
    root: &std::path::Path,
    output_dir: &std::path::Path,
) {
    let file_path = std::path::Path::new(&file_path_str);
    let Some(dest) = path::gen_destination2(file_path, root, output_dir) else {
        eprintln!("path generation failed for {:?}", file_path_str);
        return;
    };
    if let Err(err) = create_dir_for(&dest) {
        eprintln!("{}", err);
        return;
    };
    match std::fs::copy(file_path, &dest) {
        Ok(_) => println!("{:?} -> {:?}", file_path, dest),
        Err(error) => eprintln!("during copy {:?}", error),
    }
}

fn create_dir_for(file_path: &std::path::Path) -> std::io::Result<()> {
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    return Ok(());
}

pub fn flatten_discard<I>(source_dir: &std::ffi::OsStr, destination_dir: &std::ffi::OsStr, files: I)
where
    I: Iterator<Item = std::ffi::OsString>,
{
    let source_dir = std::path::Path::new(source_dir);
    let destination_dir = std::path::Path::new(destination_dir);
    for file_path_str in files {
        process_one_discard(file_path_str, source_dir, destination_dir);
    }
}

fn process_one_discard(
    file_path_str: std::ffi::OsString,
    root: &std::path::Path,
    output_dir: &std::path::Path,
) {
    let file_path = std::path::Path::new(&file_path_str);
    let Some(dest) = path::gen_destination(file_path, root, output_dir) else {
        eprintln!("path generation failed for {:?}", file_path_str);
        return;
    };
    if let Err(err) = create_dir_for(&dest) {
        eprintln!("{}", err);
        return;
    };
    match std::fs::copy(file_path, &dest) {
        Ok(_) => println!("{:?} -> {:?}", file_path, dest),
        Err(error) => eprintln!("during copy {:?}", error),
    }
}
