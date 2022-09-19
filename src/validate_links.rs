use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use colored::Colorize;
use markdown_it::parser::core::CoreRule;
use markdown_it::parser::extset::MarkdownItExt;
use markdown_it::plugins::cmark::inline::link::Link;
use markdown_it::MarkdownIt;
use markdown_it::Node;

use crate::md::MD_EXT;
use crate::wikilinks::WikiLink;

struct ValidLinksRule;

#[derive(Debug)]
struct FileSet(HashSet<String>);

pub fn add(md: &mut MarkdownIt, dir: PathBuf) {
    md.add_rule::<ValidLinksRule>();

    let files = fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file());

    md.ext.insert(FileSet(
        files.map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect(),
    ));
}

impl CoreRule for ValidLinksRule {
    fn run(root: &mut Node, md: &MarkdownIt) {
        let FileSet(files) = md.ext.get::<FileSet>().unwrap();

        root.walk(|node, _| {
            if let Some(node) = node.cast::<WikiLink>() {
                let (file, _) = node.file.rsplit_once('.').unwrap();
                let file = format!("{file}.{MD_EXT}");

                if !files.contains(&file) {
                    let link = format!("'[[{}|{}]]'", node.disp, node.file);
                    let msg = format!("{link} is not valid link: '{file}' does not exist");
                    eprintln!("{}", msg.red());
                }
            } else if let Some(node) = node.cast::<Link>() {
                let is_local_uri = !node.url.contains('/');

                if is_local_uri && !files.contains(&node.url) {
                    let msg = format!("linked resource '{}' does not exist", node.url);
                    eprintln!("{}", msg.red());
                }
            }
        });
    }
}

impl MarkdownItExt for FileSet {}
