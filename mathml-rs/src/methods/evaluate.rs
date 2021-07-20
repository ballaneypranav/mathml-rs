use super::super::structs::math_node::{MathNode, NodeIndex};
use super::super::structs::numbers::{NumType, Number};
use super::super::structs::op::Op;
use std::collections::HashMap;

pub fn evaluate_node(
    nodes: &Vec<MathNode>,
    head_idx: NodeIndex,
    values: &HashMap<String, f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<f64, String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Root(root) => {
            if root.children.len() != 1 {
                return Err("Root with multiple/zero children!".to_string());
            }
            evaluate_node(nodes, root.children[0], values, functions)
        }
        MathNode::Apply(apply) => {
            let op_result = apply.get_op(nodes);
            // If this is a regular mathematical operator, go ahead
            if let Ok(op) = op_result {
                match op {
                    Op::Times => {
                        let mut result = 1.0;
                        for operand_idx in apply.operands {
                            result *= evaluate_node(nodes, operand_idx, values, functions)?;
                        }
                        Ok(result)
                    }
                    Op::Plus => {
                        let mut result = 0.0;
                        for operand_idx in apply.operands {
                            result += evaluate_node(nodes, operand_idx, values, functions)?;
                        }
                        Ok(result)
                    }
                    Op::Minus => {
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        match apply.operands.len() {
                            1 => Ok(-a),
                            2 => {
                                let b = evaluate_node(nodes, apply.operands[1], values, functions)?;
                                Ok(a - b)
                            }
                            _ => Err("Too many operands!".to_string()),
                        }
                    }
                    Op::Divide => {
                        if apply.operands.len() != 2 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        let b = evaluate_node(nodes, apply.operands[1], values, functions)?;
                        Ok(a / b)
                    }
                    Op::Power => {
                        if apply.operands.len() != 2 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        let b = evaluate_node(nodes, apply.operands[1], values, functions)?;
                        Ok(a.powf(b))
                    }
                    Op::Ceiling => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        Ok(round::ceil(a, 0))
                    }
                    Op::Factorial => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        Ok(factorial(a as u32) as f64)
                    }
                    _ => Err("Evaluation not supported for operator.".to_string()),
                }
            } else {
                // Evaluate as a lambda function
                let mut res = None;
                if let MathNode::Ci(ci) = &nodes[apply.operator.unwrap()] {
                    let lambda_name = ci.name.as_ref().unwrap();
                    if let Some(lambda) = functions.get(lambda_name) {
                        let mut argument_values = Vec::new();
                        for operand in apply.operands {
                            argument_values.push(evaluate_node(nodes, operand, values, functions)?);
                        }
                        //println!("evaluating");
                        res = Some(evaluate_lambda(lambda, 0, &argument_values, functions)?);
                        //dbg!(res);
                    }
                }
                if let Some(value) = res {
                    Ok(value)
                } else {
                    Err("Invalid operator".to_string())
                }
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
    }
}

pub fn evaluate_lambda(
    nodes: &Vec<MathNode>,
    head_idx: NodeIndex,
    argument_values: &Vec<f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<f64, String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Lambda(lambda) => {
            let mut argument_names = Vec::new();
            for binding in lambda.bindings {
                if let MathNode::BVar(bvar) = nodes[binding].clone() {
                    for child in bvar.children {
                        if let MathNode::Ci(ci) = nodes[child].clone() {
                            argument_names.push(ci.name.unwrap());
                        }
                    }
                }
            }

            if argument_values.len() != argument_names.len() {
                Err("Argument names and values mismatch".to_string())
            } else {
                let mut assignments: HashMap<String, f64> = HashMap::new();
                for i in 0..argument_values.len() {
                    assignments.insert(argument_names[i].clone(), argument_values[i]);
                }
                Ok(evaluate_node(
                    nodes,
                    lambda.expr.unwrap(),
                    &assignments,
                    functions,
                )?)
            }
        }
        _ => {
            //dbg!(head);
            Err("haha couldn't parse".to_string())
        }
    }
}

