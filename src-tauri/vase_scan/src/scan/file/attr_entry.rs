use syn::Item;

use crate::scan::file::FileParser;

pub struct VaseEntryParser;

impl FileParser for VaseEntryParser {
    fn parse(&self, _item: Item) {}
    fn parse_file_items(&self, _items: Vec<Item>) {}
}
