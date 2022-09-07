use markdown_it::parser::block::BlockRule;
use markdown_it::parser::block::BlockState;
use markdown_it::parser::core::CoreRule;
use markdown_it::parser::extset::MarkdownItExt;
use markdown_it::MarkdownIt;
use markdown_it::Node;
use markdown_it::NodeValue;
use markdown_it::Renderer;

#[derive(Debug)]
pub struct FilterUnsafe;

const MARKER: char = '%';

struct SafeRule;

struct SafeMarkerScanner;

#[derive(Debug)]
struct SafeMarker;

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<SafeRule>();
    md.block.add_rule::<SafeMarkerScanner>();
}

impl CoreRule for SafeRule {
    fn run(root: &mut Node, md: &MarkdownIt) {
        let filter_unsafe = md.ext.contains::<FilterUnsafe>();

        let mut is_safe = !filter_unsafe;
        let its: Vec<(usize, bool)> = root
            .children
            .iter()
            .enumerate()
            .map(|(idx, node)| {
                let keep = if node.is::<SafeMarker>() {
                    if filter_unsafe {
                        is_safe = !is_safe;
                    }

                    false
                } else {
                    is_safe
                };
                (idx, keep)
            })
            .rev()
            .collect();

        for (idx, keep) in its {
            if !keep {
                root.children.remove(idx);
            }
        }
    }
}

impl MarkdownItExt for FilterUnsafe {}

impl BlockRule for SafeMarkerScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        if state.line_indent(state.line) > 3 {
            return None;
        }

        let line = state.get_line(state.line);

        if !line.trim().chars().all(|c| c == MARKER) {
            return None;
        }

        Some((Node::new(SafeMarker), 1))
    }
}

impl NodeValue for SafeMarker {
    fn render(&self, _: &Node, _: &mut dyn Renderer) {
        panic!("unfiltered SafeMarker");
    }
}
