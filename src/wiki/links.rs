use itertools::intersperse;
use markdown_it::parser::inline;
use markdown_it::parser::inline::InlineRule;
use markdown_it::MarkdownIt;
use markdown_it::Node;
use markdown_it::NodeValue;
use markdown_it::Renderer;

use crate::md::HTML_EXT;

const START: &str = "[[";
const SEP: char = '|';
const END: &str = "]]";

struct WikiScanner;

#[derive(Debug)]
pub struct WikiLink {
    pub disp: String,
    pub file: String,
}

pub fn add(md: &mut MarkdownIt) {
    md.inline.add_rule::<WikiScanner>();
}

pub fn link_to_filename(s: &str, ext: &str) -> String {
    let despaced: String = intersperse(s.split_ascii_whitespace(), "_").collect();
    let dequoted = despaced.to_ascii_lowercase().replace('\'', "");
    format!("./{dequoted}.{ext}")
}

impl InlineRule for WikiScanner {
    const MARKER: char = '[';

    fn run(state: &mut inline::InlineState) -> Option<(Node, usize)> {
        let inp = &state.src[state.pos..state.pos_max];

        if !inp.starts_with(START) {
            return None;
        }

        let end_idx = inp.find(END)?;

        let len = end_idx + END.len();
        let inner = &inp[START.len()..end_idx];
        let node = if let Some((disp, file)) = inner.split_once(SEP) {
            Node::new(WikiLink { disp: disp.trim().to_string(), file: file.trim().to_string() })
        } else {
            let inner = inner.trim().to_string();
            Node::new(WikiLink { disp: inner.clone(), file: inner })
        };

        Some((node, len))
    }
}

impl NodeValue for WikiLink {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();

        let path = link_to_filename(&self.file, HTML_EXT);
        attrs.push(("href", path));

        fmt.open("a", &attrs);
        fmt.text(&self.disp);
        fmt.close("a");
    }
}
