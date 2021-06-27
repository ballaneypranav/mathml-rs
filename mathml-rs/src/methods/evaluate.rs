use super::super::structs::math_node::{MathNode, NodeIndex};
use super::super::structs::numbers::{NumType, Number};
use super::super::structs::op::Op;
use std::collections::HashMap;

pub fn evaluate_node(
    nodes: &Vec<MathNode>,
    head_idx: NodeIndex,
    values: &HashMap<String, f64>,
) -> Result<f64, &'static str> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Root(root) => {
            if root.children.len() != 1 {
                return Err("Root with multiple/zero children!");
            }
            evaluate_node(nodes, root.children[0], values)
        }
        MathNode::Apply(apply) => {
            let op = apply.get_op(nodes)?;
            // check if there are operands
            if apply.operands.len() < 1 {
                return Err("No operands!");
            }
            match op {
                Op::Times => {
                    let mut result = 1.0;
                    for operand_idx in apply.operands {
                        result *= evaluate_node(nodes, operand_idx, values)?;
                    }
                    Ok(result)
                }
                Op::Plus => {
                    let mut result = 0.0;
                    for operand_idx in apply.operands {
                        result += evaluate_node(nodes, operand_idx, values)?;
                    }
                    Ok(result)
                }
                Op::Minus => {
                    let a = evaluate_node(nodes, apply.operands[0], values)?;
                    match apply.operands.len() {
                        1 => Ok(-a),
                        2 => {
                            let b = evaluate_node(nodes, apply.operands[1], values)?;
                            Ok(a - b)
                        }
                        _ => Err("Too many operands!"),
                    }
                }
                Op::Divide => {
                    if apply.operands.len() != 2 {
                        return Err("Invalid number of operands.");
                    }
                    let a = evaluate_node(nodes, apply.operands[0], values)?;
                    let b = evaluate_node(nodes, apply.operands[1], values)?;
                    Ok(a / b)
                }
                Op::Power => {
                    if apply.operands.len() != 2 {
                        return Err("Invalid number of operands.");
                    }
                    let a = evaluate_node(nodes, apply.operands[0], values)?;
                    let b = evaluate_node(nodes, apply.operands[1], values)?;
                    Ok(a.powf(b))
                }
                _ => Err("Evaluation not supported for operator."),
            }
        }
        MathNode::Cn(cn) => match &cn.r#type {
            Some(NumType::Integer) => {
                if let Some(Number::Integer(i)) = cn.value {
                    Ok(i.into())
                } else {
                    Err("Wrong type")
                }
            }
            _ => Err("couldn't parse"),
        },
        MathNode::Ci(ci) => {
            let name = ci.name.expect("Ci element with no content!");
            if values.contains_key(&name) {
                Ok(*values.get(&name).unwrap())
            } else {
                Err("No value found!")
            }
        }
        _ => Err("couldn't parse"),
    }
}
