use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
pub mod structs;
use mathml_macros::*;
pub use structs::*;

pub fn parse_fragment(
    mut reader: Reader<BufReader<File>>,
) -> (Vec<MathNode>, Reader<BufReader<File>>) {
    reader.trim_text(true);
    reader.expand_empty_elements(true);
    let mut buf = Vec::new();
    let mut stack: Vec<NodeIndex> = Vec::new();

    let mut container = Vec::new();
    let mut container_len = 0;

    container.push(MathNode::default());
    container_len += 1;
    let mut current = 0;
    stack.push(current);

    loop {
        match reader.read_event(&mut buf) {
            // for each starting tag
            Ok(Event::Start(ref e)) => {
                let mut new_tag = None;
                match e.name() {
                    b"apply" => attach![Apply to Root | Apply],
                    b"times" => attach![Op::Times to Apply],
                    b"power" => attach![Op::Power to Apply],
                    b"ci" => attach![Ci to Apply],
                    b"cn" => attach![Cn with
                                        r#type as String,
                                    to Apply],
                    _ => {
                        panic!("Tag not parsed: {}", std::str::from_utf8(e.name()).unwrap());
                    }
                }
                match new_tag {
                    Some(t) => {
                        container.push(t);
                        container_len += 1;
                    }
                    None => {}
                }
            }
            Ok(Event::End(ref e)) => match e.name() {
                b"apply" => close![Apply],
                b"times" => close![Op],
                b"power" => close![Op],
                b"ci" => close![Ci],
                b"cn" => close![Cn],
                b"math" => break,
                _ => {}
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                let s = e.unescape_and_decode(&reader).unwrap();
                match container[current] {
                    MathNode::Ci(..) => {
                        container[current] = MathNode::Ci(Ci::with_text(s));
                    }
                    MathNode::Cn(ref mut cn) => match cn.r#type.as_deref() {
                        Some("integer") => {
                            cn.integer = Some(s.parse::<i32>().expect("Incorrect type"))
                        }
                        _ => {
                            panic!("Math type did not match for cn: {:?}", cn);
                        }
                    },
                    _ => {
                        panic!("Text not parsed in {:?}: {}", container[current], s);
                    }
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    //println!("");
    //let mut count = 0;
    //for item in &container {
    //println!("{:0>2}: {}", count, item);
    //count += 1;
    //}
    //println!("{:?}", txt);
    //println!("{:?}", stack);
    //println!("{:?}", current);

    (container, reader)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let filename = "../../testsuites/core-semantic/00004/00004-sbml-l3v2.xml";
        let reader = Reader::from_file(filename).expect("File error.");

        parse_fragment(reader);
    }
}
