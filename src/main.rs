mod md;
mod validate_links;
mod wikilinks;

use colored::Colorize;
use itertools::Either;
use itertools::Itertools;
use std::ffi::OsStr;
use std::fs;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::io::Error;
use std::path::Path;
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

    let inp_dir = find_input_dir(&args.input_dir);
    let inp_files = find_input_files(&inp_dir);

    let md = MdRenderer::new(inp_dir);
    let out_dir = find_or_make_out_dir(args.out_dir);

    for file in inp_files {
        let ext = file.extension().unwrap_or_else(|| OsStr::new("")).to_string_lossy();

        if ext.to_lowercase() == MD_EXT {
            render_md_file(&md, &out_dir, &file);
        } else {
            copy_file(&out_dir, &file);
        }
    }
}

fn find_input_dir(arg: &str) -> PathBuf {
    PathBuf::from(arg)
        .canonicalize()
        .map_err(|err| format!("'{arg}' is not a valid input directory: {err}"))
        .unwrap()
}

fn find_input_files(dir: &Path) -> impl Iterator<Item = PathBuf> {
    let inp_dir_read =
        read_dir(dir).map_err(|err| format!("cannot read input directory: {err}")).unwrap();
    let (inp_entries, inp_errs): (Vec<DirEntry>, Vec<Error>) =
        inp_dir_read.partition_map(|e| match e {
            Ok(e) => Either::Left(e),
            Err(e) => Either::Right(e),
        });

    for err in inp_errs {
        eprintln!("file error: {}", err.to_string().red());
    }

    inp_entries.into_iter().filter_map(|e| {
        let path = e.path();
        path.is_file().then(|| path)
    })
}

fn find_or_make_out_dir(arg: String) -> PathBuf {
    let out_dir = PathBuf::from(arg);
    fs::create_dir_all(&out_dir).unwrap();
    out_dir.canonicalize().unwrap()
}

fn render_md_file(md: &MdRenderer, out_dir: &Path, file: &Path) {
    let dest = out_dir.join(file.with_extension(HTML_EXT).file_name().unwrap());

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
}

fn copy_file(out_dir: &Path, file: &Path) {
    let dest = out_dir.join(file.file_name().unwrap());
    let cp = format!("'{}' to '{}'", file.display(), dest.display());

    if let Err(err) = fs::copy(file, dest) {
        let msg = format!("cannot copy {cp}: {err}").red();
        eprintln!("{msg}");
    }
}
