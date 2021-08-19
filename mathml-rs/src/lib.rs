use mathml_macros::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
pub mod structs;
pub use structs::apply::*;
pub use structs::bindings::*;
pub use structs::ci::*;
pub use structs::cn::*;
pub use structs::constants::*;
pub use structs::lambda::*;
pub use structs::math_node::*;
pub use structs::numbers::*;
pub use structs::op::*;
pub use structs::piecewise::*;
pub use structs::root::*;

pub mod methods;
pub use methods::evaluate::*;

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
                let new_tag;
                match e.name() {
                    b"apply" => attach![Apply to Root | Apply | Lambda | Piece | Otherwise],
                    b"times" => attach![Op::Times to Apply],
                    b"divide" => attach![Op::Divide to Apply],
                    b"minus" => attach![Op::Minus to Apply],
                    b"plus" => attach![Op::Plus to Apply],
                    b"power" => attach![Op::Power to Apply],
                    b"factorial" => attach![Op::Factorial to Apply],
                    b"eq" => attach![Op::Eq to Apply],
                    b"neq" => attach![Op::Neq to Apply],
                    b"gt" => attach![Op::Gt to Apply],
                    b"lt" => attach![Op::Lt to Apply],
                    b"geq" => attach![Op::Geq to Apply],
                    b"leq" => attach![Op::Leq to Apply],
                    b"and" => attach![Op::And to Apply],
                    b"or" => attach![Op::Or to Apply],
                    b"xor" => attach![Op::Xor to Apply],
                    b"ceiling" => attach![Op::Ceiling to Apply],
                    b"floor" => attach![Op::Floor to Apply],
                    b"true" => attach![Constant::True to Apply | Piece ],
                    b"false" => attach![Constant::False to Apply | Piece ],
                    b"ci" => attach![Ci to Root | Apply | BVar | Piece | Otherwise | Lambda ],
                    b"cn" => attach![Cn with
                                        r#type as NumType,
                                    to Root | Apply | BVar | Piece | Otherwise | Lambda ],
                    b"lambda" => attach![Lambda to Root],
                    b"bvar" => attach![BVar to Lambda],
                    b"piecewise" => attach![Piecewise to Root | Apply | Lambda],
                    b"piece" => attach![Piece to Piecewise],
                    b"otherwise" => attach![Otherwise to Piecewise],
                    b"sep" => new_tag = None,
                    _ => {
                        panic!("Tag not parsed: {}", std::str::from_utf8(e.name()).unwrap());
                    }
                }
                if let Some(t) = new_tag {
                    container.push(t);
                    container_len += 1;
                }
            }
            Ok(Event::End(ref e)) => match e.name() {
                b"apply" => close![Apply],
                b"times" => close![Op],
                b"divide" => close![Op],
                b"minus" => close![Op],
                b"plus" => close![Op],
                b"power" => close![Op],
                b"factorial" => close![Op],
                b"eq" => close![Op],
                b"neq" => close![Op],
                b"geq" => close![Op],
                b"leq" => close![Op],
                b"gt" => close![Op],
                b"lt" => close![Op],
                b"and" => close![Op],
                b"or" => close![Op],
                b"xor" => close![Op],
                b"ceiling" => close![Op],
                b"floor" => close![Op],
                b"piecewise" => close![Piecewise],
                b"piece" => close![Piece],
                b"otherwise" => close![Otherwise],
                b"true" => close![Constant],
                b"false" => close![Constant],
                b"ci" => close![Ci],
                b"cn" => close![Cn],
                b"lambda" => close![Lambda],
                b"bvar" => close![BVar],
                b"math" => break,
                _ => {}
            },
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                let s = e.unescape_and_decode(&reader).unwrap();
                match container[current] {
                    MathNode::Ci(..) => {
                        container[current] = MathNode::Ci(Ci::with_name(s));
                    }
                    MathNode::Cn(ref mut cn) => match cn.r#type {
                        Some(NumType::Real) | None => {
                            let value = s.parse::<f64>().expect("Incorrect type");
                            cn.value = Some(Number::Real(value));
                        }
                        Some(NumType::Integer) => {
                            let value = s.parse::<i32>().expect("Incorrect type");
                            cn.value = Some(Number::Integer(value));
                        }
                        Some(NumType::Rational) => {
                            let value = s.parse::<i32>().expect("Incorrect type");
                            if cn.value.is_none() {
                                cn.value = Some(Number::Rational(value.into(), 1));
                            } else if let Some(Number::Rational(x, y)) = cn.value {
                                if y != 1 {
                                    panic!("Error occurred while storing rational number.");
                                }
                                cn.value = Some(Number::Rational(x, value.into()));
                            }
                        }

                        Some(NumType::ENotation) => {
                            let value = s.parse::<i32>().expect("Incorrect type");
                            if cn.value.is_none() {
                                cn.value = Some(Number::ENotation(value.into(), 1));
                            } else if let Some(Number::ENotation(x, y)) = cn.value {
                                if y != 1 {
                                    panic!("Error occurred while storing rational number.");
                                }
                                cn.value = Some(Number::ENotation(x, value.into()));
                            }
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
