use std::path::PathBuf;

use colored::Colorize;
use markdown_it::parser::core::CoreRule;
use markdown_it::parser::extset::MarkdownItExt;
use markdown_it::MarkdownIt;
use markdown_it::Node;

use crate::md::MD_EXT;
use crate::wiki::links::link_to_filename;
use crate::wiki::links::WikiLink;

struct ValidWikiLinksRule;

#[derive(Debug)]
struct ValidWikiLinksDir(PathBuf);

pub fn add(md: &mut MarkdownIt, dir: PathBuf) {
    md.add_rule::<ValidWikiLinksRule>();
    md.ext.insert(ValidWikiLinksDir(dir));
}

impl CoreRule for ValidWikiLinksRule {
    fn run(root: &mut Node, md: &MarkdownIt) {
        let ValidWikiLinksDir(dir) = md.ext.get::<ValidWikiLinksDir>().unwrap();

        root.walk(|node, _| {
            if node.is::<WikiLink>() {
                let node = node.cast::<WikiLink>().unwrap();
                let link = format!("'[[{}|{}]]'", node.disp, node.file);
                let file = dir.join(link_to_filename(&node.file, MD_EXT));
                let err = match file.try_exists() {
                    Ok(true) => return,
                    Ok(false) => format!(
                        "{link} is not a valid wiki link: '{}' cannot be resolved",
                        file.display()
                    ),
                    Err(err) => {
                        format!("{link} cannot be resolved to '{}': {err}", file.display())
                    }
                };
                eprintln!("{}", err.red());
            }
        });
    }
}

impl MarkdownItExt for ValidWikiLinksDir {}
