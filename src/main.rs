use std::{fs::File, process::exit};

/// File actions (open, create, read lines, etc.)
mod files;

enum SyntaxElementType {
	Header,
	UnorderedEntry,
	OrderedEntry,
}

struct MarkdownSyntaxElement {
	element_depth: i32,
	element_type: SyntaxElementType,
	list_order: i32,
	element_text: String,
}

/// Indent by two spaces for sub-items
static SPACES_PER_DEPTH: i32 = 2;

/// Maximum depth allowed of a header or list item
static MAX_DEPTH: i32 = 5;

fn main() {
	let args: Vec<String> = collect_args();

	let markdown_file: File = files::open_file(&args[1].clone());
}

fn collect_args() -> Vec<String> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() <= 1 {
		println!("Please enter a Markdown file as your first argument.");
		exit(1);
	}

	return args;
}
