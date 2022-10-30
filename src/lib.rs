mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wordcut_engine::{create_prefix_tree, Wordcut};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn cut(target: &str) -> String {
    // Just in case
    set_panic_hook();

    //Execute
    // let dict = super::load_dict(Path::new("data/thai.txt"));
    // let wordcut = Wordcut::new(dict.unwrap());
    // let words = wordcut.segment_into_strings("กากกา");
    // let expected = vec!["กาก".to_string(), "กา".to_string()];
    // assert_eq!(words, expected)

    let dict = create_prefix_tree(&["กา", "กาก"]);
    let wordcut = Wordcut::new(dict);

    wordcut.put_delimiters(target, "|")
}
