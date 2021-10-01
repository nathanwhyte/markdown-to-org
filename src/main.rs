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

	convert_to_org(markdown_file_struct);
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
		return markdown_file_path;
	}

	let mut current_char: char = markdown_file_path.pop().unwrap();
	while current_char != '/' {
		markdown_file_name.insert(0, current_char);
		current_char = markdown_file_path.pop().unwrap();
	}

	return markdown_file_name;
}

/// Driver function for converting Markdown to Org
fn convert_to_org(markdown_file_struct: MarkdownFile) {

	// TODO read lines and build struct for each one
	let mut syntax_elements: Vec<MarkdownSyntaxElement> = get_syntax_elements(markdown_file_struct);
}

/// Build a vector of Markdown syntax elements for each valid line in the given Markdown file
fn get_syntax_elements(markdown_file_struct: MarkdownFile) -> Vec<MarkdownSyntaxElement> {
	let syntax_elements_vec = Vec::new();
	let lines: Vec<String> = files::read_lines(markdown_file_struct.fd);

	// TODO skip first line that has text (title line)
	let title_index: usize = find_title_line_index(markdown_file_struct.title, lines.clone());

	for line_index in title_index..lines.len() {
		// TODO don't process blank lines

	}

	// TODO build a syntax_element struct for each line

	return syntax_elements_vec;
}

/// Find the index in lines where the file's title line is
fn find_title_line_index(markdown_file_title: String, lines: Vec<String>) -> usize {
	let mut title_line_index: usize = 0;

	for line in lines {
		if line.contains(&markdown_file_title) {
			break;
		}

		title_line_index += 1;
	}

	return title_line_index;
}
