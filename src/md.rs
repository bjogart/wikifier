use std::path::PathBuf;

use markdown_it::plugins::cmark;
use markdown_it::plugins::extra;
use markdown_it::MarkdownIt;

use crate::wiki;

pub const MD_EXT: &str = "md";
pub const HTML_EXT: &str = "html";

pub struct MdRenderer {
    md: MarkdownIt,
}

impl MdRenderer {
    pub fn new(dir: PathBuf) -> Self {
        let mut md = MarkdownIt::new();

        cmark::add(&mut md);
        extra::add(&mut md);
        wiki::add_links(&mut md);
        wiki::add_validation(&mut md, dir);

        Self { md }
    }

    pub fn render(&self, source: &str) -> String {
        let ast = self.md.parse(source);
        ast.render()
    }
}
