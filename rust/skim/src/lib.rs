use skim::prelude::*;
use cxx::{CxxString, CxxVector};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn skim(words: &CxxVector<CxxString>) -> String;
    }
}

struct Item {
    text: String,
}
impl SkimItem for Item {
    fn text(&self) -> Cow<str> {
        return Cow::Borrowed(&self.text);
    }
}

fn skim(words: &CxxVector<CxxString>) -> String {
    // TODO: configure colors
    let options = SkimOptionsBuilder::default()
        .height(Some("30%"))
        .tac(true)
        .tiebreak(Some("-score".to_string()))
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    for word in words {
        tx.send(Arc::new(Item{ text: word.to_string() })).unwrap();
    }
    // so that skim could know when to stop waiting for more items.
    drop(tx);

    let output = Skim::run_with(&options, Some(rx));
    if output.is_none() {
        return "".to_string();
    }
    let output = output.unwrap();
    if output.is_abort {
        return "".to_string();
    }

    if output.selected_items.is_empty() {
        return "".to_string();
    }
    return output.selected_items[0].output().to_string();
}
