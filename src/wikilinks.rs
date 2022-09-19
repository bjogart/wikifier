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
    pub link: Option<String>,
    pub file: String,
}

pub fn add(md: &mut MarkdownIt) {
    md.inline.add_rule::<WikiScanner>();
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

        let (disp, link) = match inner.split_once(SEP) {
            Some((disp, link)) => (disp, Some(link)),
            None => (inner, None),
        };

        let file = link.unwrap_or(disp).to_lowercase().replace('\'', "");
        let file: String = intersperse(file.split_ascii_whitespace(), "_").collect();
        let file = format!("{file}.{HTML_EXT}");

        let node = Node::new(WikiLink {
            disp: disp.trim().to_string(),
            link: link.map(|l| l.trim().to_string()),
            file,
        });

        Some((node, len))
    }
}

impl NodeValue for WikiLink {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();

        attrs.push(("href", self.file.clone()));

        fmt.open("a", &attrs);
        fmt.text(&self.disp);
        fmt.close("a");
    }
}
