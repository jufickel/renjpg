// Copyright (c) 2019 Juergen Fickel
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CliOpt {
    /// The JPEG files to be renamed
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliOpt::from_args();

    if !args.files.is_empty() {
        renjpg::rename_user_provided_files(&args.files)
    } else {
        renjpg::rename_all_jpegs_in_dir("./")
    }
}
