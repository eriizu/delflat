use std::path::Path;

fn create_dir_for(file_path: &std::path::Path) -> std::io::Result<()> {
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    return Ok(());
}

fn main() {
    let mut args = std::env::args().skip(1);
    let root = args.nth(0).expect("need a root");
    let root = std::path::Path::new(&root);
    let target = args.nth(0).expect("need a target dir");
    let output_dir = std::path::Path::new(&target);
    for file_path_str in args {
        let file_path = std::path::Path::new(&file_path_str);
        let dest = delivery_flatener::path::gen_destination2(file_path, root, output_dir).unwrap();
        println!("dest for {:?} would be {:?}", file_path, dest);
        create_dir_for(&dest).unwrap();
        match std::fs::copy(file_path, dest) {
            Ok(a) => println!("{:?}", a),
            Err(error) => eprintln!("during copy {:?}", error),
        }
    }
}
