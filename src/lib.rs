pub mod path;

pub fn flatten<I>(source_dir: &str, destination_dir: &str, files: I)
where
    I: Iterator<Item = String>,
{
    let source_dir = std::path::Path::new(source_dir);
    let destination_dir = std::path::Path::new(destination_dir);
    for file_path_str in files {
        process_one(file_path_str, source_dir, destination_dir);
    }
}

fn process_one(file_path_str: String, root: &std::path::Path, output_dir: &std::path::Path) {
    let file_path = std::path::Path::new(&file_path_str);
    let dest = path::gen_destination2(file_path, root, output_dir).unwrap();
    create_dir_for(&dest).unwrap();
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
