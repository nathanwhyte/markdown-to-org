use std::{fs::File, process::exit};

/// File actions (open, create, read lines, etc.)
mod files;

struct MarkdownFile {
	markdown_file_fd: File,
	title: String,
}

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
	let markdown_file_struct: MarkdownFile = get_markdown_file(args[1].clone());
}

/// Collects command line arguments and handles an incorrect amount of arguments
fn collect_args() -> Vec<String> {
	let args: Vec<String> = std::env::args().collect();

	if args.len() <= 1 {
		println!("Please enter a Markdown file as your first argument.");
		exit(1);
	}

	return args;
}

/// Open the Markdown file at given path and build MarkdownFile struct for that file
fn get_markdown_file(markdown_file_path: String) -> MarkdownFile {
	let markdown_file_title: String = get_markdown_file_title(markdown_file_path.clone());
	let markdown_file: File = files::open_file(&markdown_file_path);

	MarkdownFile {
		markdown_file_fd: markdown_file,
		title: markdown_file_title,
	}
}

/// Get the title of a Markdown file (text from the first level 1 heading)
fn get_markdown_file_title(markdown_file_path: String) -> String {
	let lines: Vec<String> = files::read_lines(files::open_file(&markdown_file_path));
	let mut title_line: String = String::new();

	for line in lines {
		if line.contains("# ") {
			title_line = line;
			break;
		}
	}

	return title_line.trim().replacen("# ", "", 1);
}
