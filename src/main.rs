use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::*;

/// Turn an .stl file into a Sqlite database
#[derive(StructOpt)]
struct Options {
    #[structopt()]
    stl_file: PathBuf,
}

fn main() -> Result<()> {
    let options = Options::from_args();
    let file = File::open(&options.stl_file)?;
    let mut buf = BufReader::new(file);
    let stl = nom_stl::parse_stl(&mut buf)?;
    let index_stl: nom_stl::IndexMesh = stl.into();

    let out_filename = options
        .stl_file
        .file_name()
        .ok_or_else(|| anyhow!("Could not get filename without path"))?;

    let out_path = PathBuf::from(out_filename).with_extension("db");

    let mut conn = rusqlite::Connection::open(out_path)?;

    stlite::create_stl_db(&mut conn, &index_stl)?;

    Ok(())
}
