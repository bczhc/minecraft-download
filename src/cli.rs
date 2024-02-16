use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub out_dir: PathBuf,
}