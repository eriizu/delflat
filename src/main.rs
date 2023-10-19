use delflat::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let source_dir = args.nth(0).expect("need a source dir");
    let destination_dir = args.nth(0).expect("need a destination dir");
    flatten(&source_dir, &destination_dir, args);
}
