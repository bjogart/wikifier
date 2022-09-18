use markdown_it::plugins::cmark;
use markdown_it::MarkdownIt;

use crate::wiki;

pub const MD_EXT:&str = "md";
pub const HTML_EXT:&str = "html";

pub struct MdRenderer {
    md: MarkdownIt,
}

impl MdRenderer {
    pub fn new() ->Self{
        let mut md = MarkdownIt::new();

        cmark::add(&mut md);
        wiki::add_links(&mut md);

        Self { md }
    }

    pub fn render(&self, source: &str) -> String {
        let ast = self.md.parse(source);
        ast.render()
    }
}
