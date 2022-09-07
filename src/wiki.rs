use markdown_it::parser::inline;
use markdown_it::parser::inline::InlineRule;
use markdown_it::MarkdownIt;
use markdown_it::Node;
use markdown_it::NodeValue;
use markdown_it::Renderer;

const START: &str = "[[";
const SEP: char = '|';
const END: &str = "]]";

struct WikiScanner;

#[derive(Debug)]
struct WikiLink {
    disp: String,
    file: String,
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

        let file: String = self.file.split_ascii_whitespace().intersperse("_").collect();
        let path = format!("./{file}.html");
        attrs.push(("href", path));

        fmt.open("a", &attrs);
        fmt.text(&self.disp);
        fmt.close("a");
    }
}
