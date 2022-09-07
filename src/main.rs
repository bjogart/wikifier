#![feature(iter_intersperse)]

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use markdown_it::plugins::cmark;
use markdown_it::MarkdownIt;

use crate::safe::FilterUnsafe;

mod safe;
mod wiki;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    file: String,
    #[clap(short, long, value_parser, help("Output file"))]
    output: Option<String>,
    #[clap(long, action, help(r#"Include unsafe information (everything not between ("%%%")"#))]
    r#unsafe: bool,
}

fn main() {
    let args = Args::parse();

    let inp_file = &PathBuf::from(&args.file);

    let src = match fs::read_to_string(&args.file) {
        Ok(s) => s,
        Err(err) => panic!("{err}"),
    };

    let html = render(&src, !args.r#unsafe);

    let out_file = args.output.map_or_else(|| inp_file.with_extension("html"), PathBuf::from);
    match fs::write(out_file, html) {
        Ok(()) => {}
        Err(err) => panic!("{err}"),
    }
}

fn render(src: &str, filter_unsafe: bool) -> String {
    let md = &mut MarkdownIt::new();
    cmark::add(md);
    wiki::add(md);
    safe::add(md);

    if filter_unsafe {
        md.ext.insert(FilterUnsafe);
    }

    let ast = md.parse(src);

    ast.render()
}
