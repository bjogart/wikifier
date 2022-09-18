mod md;
mod wiki;

use colored::Colorize;
use itertools::Either;
use itertools::Itertools;
use std::ffi::OsStr;
use std::fs;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use clap::Parser;

use crate::md::MdRenderer;
use crate::md::HTML_EXT;
use crate::md::MD_EXT;

#[derive(Parser, Debug)]
#[clap(version, about = "Markdown-to-HTML with wiki-like extensions", long_about = None)]
struct Args {
    #[clap(value_parser, default_value = ".", help("Input directory"))]
    input_dir: String,
    #[clap(short, value_parser, default_value = "wikify_output", help("Output directory"))]
    out_dir: String,
}

fn main() {
    let args = Args::parse();

    let inp_dir = PathBuf::from(args.input_dir)
        .canonicalize()
        .map_err(|err| format!("invalid input directory: {err}"))
        .unwrap();
    let inp_dir_read =
        read_dir(&inp_dir).map_err(|err| format!("cannot read input directory: {err}")).unwrap();
    let (inp_entries, inp_errs): (Vec<DirEntry>, Vec<Error>) =
        inp_dir_read.partition_map(|e| match e {
            Ok(e) => Either::Left(e),
            Err(e) => Either::Right(e),
        });

    if !inp_errs.is_empty() {
        eprintln!("{}", "there were file errors.".on_red());

        for err in inp_errs {
            eprintln!("- {}", err.to_string().red());
        }
    }

    let inp_files = inp_entries.into_iter().filter_map(|e| {
        let path = e.path();
        path.is_file().then(|| path)
    });

    let out_dir = PathBuf::from(args.out_dir);
    fs::create_dir_all(&out_dir).unwrap();
    let out_dir = out_dir.canonicalize().unwrap();

    let md = MdRenderer::new(inp_dir);

    for file in inp_files {
        let ext = file.extension().unwrap_or_else(|| OsStr::new("")).to_string_lossy();
        if ext.to_lowercase() == MD_EXT {
            let dest = out_dir.join(file.with_extension(HTML_EXT).file_name().unwrap());
            println!("render '{}' to '{}'", file.display(), dest.display());

            match fs::read_to_string(&file) {
                Err(err) => {
                    let msg = format!("could not read '{}': {err}", file.display()).red();
                    eprintln!("{msg}");
                }
                Ok(source) => {
                    let res = md.render(&source);

                    if let Err(err) = fs::write(&dest, res) {
                        let msg = format!("could not write to '{}': {err}", dest.display()).red();
                        eprintln!("{msg}");
                    }
                }
            }
        } else {
            let dest = out_dir.join(file.file_name().unwrap());
            let cp = format!("'{}' to '{}'", file.display(), dest.display());
            println!("copy {cp}");

            if let Err(err) = fs::copy(file, dest) {
                let msg = format!("cannot copy {cp}: {err}").red();
                eprintln!("{msg}");
            }
        }
    }
}
