# Wikifier

A really simple markdown-to-HTML converter built on top of the great [markdown-it](https://github.com/rlidwka/markdown-it.rs). Has all the features of CommonMark, with some extensions:

- **wiki-style links**: either `[[some other file]]`, which will create a link to `./some_other_file.html`, or `[[display text | some other file]]`, which will link to the same file, but with `display text` in the link. Note that the order is swapped compared to the usual wiki-style, because I find them easier to write that way.
<!-- - **unsafe text!**: this feature is mainly here because I intend to use it for my D&D campaign, where I am setting up a little wiki with setting information. For reusability I make my DM notes in the same files, but obviously my players cannot see those. Enter safe blocks: everything inside `%%%` blocks (like code blocks), is visible after compilation, but nothing outside those blocks will be. Pass a command-line `--unsafe` option to compile the full file. All other markup (headers, block quotes, etc) are usable inside safe blocks like they are anywhere else. -->
