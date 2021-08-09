extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::default::Default;
use std::io::{self, Write};

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use rcdom::{RcDom, SerializableHandle};

fn main() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();

    // The validator.nu HTML2HTML always prints a doctype at the very beginning.
    io::stdout()
        .write_all(b"<!DOCTYPE html>\n")
        .ok()
        .expect("writing DOCTYPE failed");
    let document: SerializableHandle = dom.document.clone().into();
    serialize(&mut io::stdout(), &document, Default::default())
        .ok()
        .expect("serialization failed");
}