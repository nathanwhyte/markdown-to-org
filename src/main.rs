use std::fs::File;
use std::process::exit;

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

/// Get the name of the given Markdown file (name + extension)
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
	let mut syntax_elements_vec = Vec::new();
	let lines: Vec<String> = files::read_lines(markdown_file_struct.fd);

	let title_index: usize = find_title_line_index(markdown_file_struct.title, lines.clone());
	let mut line;

	for line_index in (title_index + 1)..lines.len() {
		line = lines[line_index].clone();

		if line.is_empty() {
			continue;
		}

		syntax_elements_vec.push(build_syntax_element_struct(line.clone()));
	}

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

/// Build a MarkdownSyntaxElement struct for the given line
fn build_syntax_element_struct(line: String) -> MarkdownSyntaxElement {
	let currnet_element_type: SyntaxElementType = get_syntax_element_type(line.clone());
	return MarkdownSyntaxElement {
		element_depth: line.find(' ').unwrap() as i32,
		list_order: get_list_order(&currnet_element_type, line.clone()),
		element_type: currnet_element_type,
		element_text: get_element_text(line.clone()),
	};
}

/// Get type of a Markdown syntax element based on the first character in the line
fn get_syntax_element_type(line: String) -> SyntaxElementType {
	let mut syntax_element_type: SyntaxElementType = SyntaxElementType::Header;

	let first_char: char = line.chars().nth(0).unwrap();
	if first_char == '-' {
		syntax_element_type = SyntaxElementType::UnorderedEntry;
	} else if first_char.is_numeric() {
		syntax_element_type = SyntaxElementType::OrderedEntry;
	}

	return syntax_element_type;
}

fn get_list_order(element_type: &SyntaxElementType, line: String) -> i32 {
	return match element_type {
		SyntaxElementType::Header => 0,
		SyntaxElementType::UnorderedEntry => 0,
		SyntaxElementType::OrderedEntry => line.chars().nth(0).unwrap().to_digit(10).unwrap() as i32,
	};
}

fn get_element_text(mut line: String) -> String {
	while !line.chars().next().unwrap().is_alphabetic() {
		line.remove(0);
	}

	return line.clone().trim().to_string();
}
