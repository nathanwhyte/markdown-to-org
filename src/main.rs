use std::fs::File;
use std::io::Write;
use std::process::exit;

/// File actions (open, create, read lines, etc.)
mod files;

struct MarkdownFile {
	fd: File,
	file_path: String,
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
	list_order: char,
	element_text: String,
}

/// Indent by two spaces for sub-items
static SPACES_PER_DEPTH: i32 = 2;

/// Maximum depth allowed of a header or list item
static MAX_DEPTH: i32 = 5;

fn main() {
	let args: Vec<String> = collect_args();
	let markdown_file_struct: MarkdownFile = get_markdown_file(args[1].clone());

	convert_to_org(markdown_file_struct, args[1].clone());
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

	return MarkdownFile {
		fd: markdown_file,
		title: markdown_file_title,
		file_path: markdown_file_path,
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

/// Driver function for converting Markdown to Org
fn convert_to_org(markdown_file_struct: MarkdownFile, path: String) {
	let markdown_file_path = path.clone().replace(".md", ".org");
	let org_file: File = files::create_file(&markdown_file_path);
	let file_title: String = markdown_file_struct.title.clone();

	let syntax_elements_vec: Vec<MarkdownSyntaxElement> = get_syntax_elements(markdown_file_struct);

	write_to_org_file(org_file, file_title, syntax_elements_vec);
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

/// Get the list order of a ordered list item
fn get_list_order(element_type: &SyntaxElementType, line: String) -> char {
	return match element_type {
		SyntaxElementType::Header => '*',
		SyntaxElementType::UnorderedEntry => '-',
		SyntaxElementType::OrderedEntry => line.chars().nth(0).unwrap(),
	};
}

/// Get the text of a Markdown syntax element
fn get_element_text(mut line: String) -> String {
	while !line.chars().next().unwrap().is_alphabetic() {
		line.remove(0);
	}

	return line.clone().trim().to_string();
}

/// Driver function for converting Markdown syntax to the Org
fn write_to_org_file(mut org_file: File, title: String, elements: Vec<MarkdownSyntaxElement>) {
	writeln!(&mut org_file, "#+TITLE: {}", title).expect("Error writing title line");

	for element in elements {
		match element.element_type {
			SyntaxElementType::Header => write_header(&org_file, element),
			SyntaxElementType::UnorderedEntry => write_list_element(&org_file, element, false),
			SyntaxElementType::OrderedEntry => write_list_element(&org_file, element, true),
		}
	}
}

fn write_header(mut org_file: &File, element: MarkdownSyntaxElement) {
	writeln!(&mut org_file, "").expect("Error writing blank line");

	for _star in 0..element.element_depth {
		write!(&mut org_file, "*").expect("Error writing header");
	}

	write!(&mut org_file, " {}\n", element.element_text).expect("Error writing header");
}

fn write_list_element(mut org_file: &File, element: MarkdownSyntaxElement, is_ordered: bool) {
	if is_ordered {
		writeln!(
			&mut org_file,
			"{}. {}",
			element.list_order, element.element_text
		)
		.expect("Error writing list element");
	} else {
		writeln!(
			&mut org_file,
			"{} {}",
			element.list_order, element.element_text
		)
		.expect("Error writing list element");

	}
}
