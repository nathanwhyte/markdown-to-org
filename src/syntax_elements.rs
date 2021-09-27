struct Header {
	header_depth: i32,
	header_text: String,
}

struct UnorderedEntry {
	unordered_entry_depth: i32,
	unordered_entry_text: String,
}

struct OrderedEntry {
	ordered_entry_depth: i32,
	ordered_entry_order: i32,
	ordered_enrty_text: String,
}
