use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
use std::str;
pub mod structs;
pub use structs::*;

pub fn parse_fragment(mut reader: Reader<BufReader<File>>) -> (MathNode, Reader<BufReader<File>>) {
    let root = MathNode::Root(Vec::new());
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    let mut txt = Vec::new();
    let mut stack: Vec<TagIndex> = Vec::new();

    let mut container = Vec::new();
    let mut container_len = 0;

    container.push(MathNode::new_root());
    container_len += 1;
    let mut current = 0;
    stack.push(current);

    loop {
        match reader.read_event(&mut buf) {
            // for each starting tag
            Ok(Event::Start(ref e)) => {
                //let mut new_tag = None;
                match e.name() {
                    _ => {
                        println!("Tag not parsed: {}", str::from_utf8(e.name()).unwrap());
                    }
                }
            }
            Ok(Event::End(ref e)) => match e.name() {
                _ => {}
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    for item in container {
        println!("{:?}", item);
    }
    println!("{:?}", stack);
    println!("{:?}", current);

    (root, reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let filename = "models/small.xml";
        let reader = Reader::from_file(filename).expect("File error.");

        parse_fragment(reader);
    }
}
