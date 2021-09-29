use std::{fs::File, process::exit};

/// File actions (open, create, read lines, etc.)
mod files;

struct MarkdownFile {
	fd: File,
	file_name: String,
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
	let markdown_file: File = files::open_file(&markdown_file_path);
	let markdown_file_title: String = get_markdown_file_title(markdown_file_path.clone());
	let markdown_file_name: String = get_markdown_file_name(markdown_file_path.clone());

	// TODO get file name from markdown_file_path

	return MarkdownFile {
		fd: markdown_file,
		title: markdown_file_title,
		file_name: markdown_file_name,
	};
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

fn get_markdown_file_name(mut markdown_file_path: String) -> String {
	let mut markdown_file_name: String = String::new();

	if !markdown_file_path.contains("/") {
		println!("file name: {}", markdown_file_path);
		return markdown_file_path;
	}

	let mut current_char: char = markdown_file_path.pop().unwrap();
	while current_char != '/' {
		markdown_file_name.insert(0, current_char);
		current_char = markdown_file_path.pop().unwrap();
	}

	println!("file name: {}", markdown_file_name);

	return markdown_file_name;
}

fn convert_to_org() {
	// TODO read lines and build struct for each one
}
