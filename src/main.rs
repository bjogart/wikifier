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
#[clap(version, about = "Markdown-to-HTML with minimal wiki-like extensions", long_about = None)]
struct Args {
    #[clap(value_parser)]
    file: String,
    #[clap(short, long, value_parser, help("Output file path"))]
    out: Option<String>,
    #[clap(short, long, value_parser, default_value = "", help("Output directory"))]
    dir: String,
    #[clap(
        long,
        action,
        help(r#"Include unsafe information (everything not sandwiched by "%%%""#)
    )]
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

    let out_file = args.out.map_or_else(
        || {
            let mut out = PathBuf::from(args.dir);
            out.push(inp_file);
            out.set_extension("html");
            out
        },
        PathBuf::from,
    );

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
