use super::super::structs::constants::Constant;
use super::super::structs::math_node::{MathNode, NodeIndex};
use super::super::structs::numbers::{NumType, Number};
use super::super::structs::op::Op;
use math::round;
use mathru::statistics::combins::factorial;
use std::collections::HashMap;

pub fn evaluate_node(
    nodes: &[MathNode],
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
                    Op::Floor => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        Ok(round::floor(a, 0))
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
                    let result = i.into();
                    //println!("Returning {} from cn", result);
                    Ok(result)
                } else {
                    Err("Wrong type".to_string())
                }
            }
            Some(NumType::Real) | None => {
                if let Some(Number::Real(r)) = cn.value {
                    Ok(r)
                } else {
                    Err("Wrong type".to_string())
                }
            }
            _ => Err("Invalid Cn type".to_string()),
        },
        MathNode::Ci(ci) => {
            let name = ci.name.expect("Ci element with no content!");
            if values.contains_key(&name) {
                let result = *values.get(&name).unwrap();
                //println!("Returning {} from ci", result);
                Ok(result)
            } else {
                let error = format!("No value found for Ci {}", name);
                Err(error)
            }
        }
        MathNode::Piecewise(..) => Ok(evaluate_piecewise(nodes, head_idx, values, functions)?),
        _ => {
            let error = format!("Couldn't evaluate operator {}", head);
            Err(error)
        }
    }
}

pub fn evaluate_lambda(
    nodes: &[MathNode],
    head_idx: NodeIndex,
    argument_values: &[f64],
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<f64, String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Root(root) => {
            if root.children.len() != 1 {
                return Err("Root with multiple/zero children!".to_string());
            }
            evaluate_lambda(nodes, root.children[0], argument_values, functions)
        }
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
            Err("couldn't parse lambda".to_string())
        }
    }
}

pub fn evaluate_piecewise(
    nodes: &[MathNode],
    head_idx: NodeIndex,
    values: &HashMap<String, f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<f64, String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Piecewise(piecewise) => {
            let pieces_idx = piecewise.pieces;
            let otherwise_idx = piecewise.otherwise;
            let mut result = None;
            for piece_idx in pieces_idx {
                let (condition, value) = evaluate_piece(nodes, piece_idx, values, functions)?;
                if condition {
                    if let Some(..) = value {
                        result = value;
                        break;
                    }
                }
            }
            if let Some(value) = result {
                Ok(value)
            } else if let Some(otherwise_idx_value) = otherwise_idx {
                Ok(evaluate_piecewise(
                    nodes,
                    otherwise_idx_value,
                    values,
                    functions,
                )?)
            } else {
                Err("All pieces evaluated to false and no otherwise branch found.".to_string())
            }
        }
        MathNode::Otherwise(otherwise) => {
            let expr_idx = otherwise.expr.expect("Otherwise branch is empty!");
            Ok(evaluate_node(nodes, expr_idx, values, functions)?)
        }
        _ => {
            //dbg!(head);
            Err("haha couldn't parse".to_string())
        }
    }
}

pub fn evaluate_piece(
    nodes: &[MathNode],
    head_idx: NodeIndex,
    values: &HashMap<String, f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<(bool, Option<f64>), String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Piece(piece) => {
            let expr_idx = piece.expr.expect("Piece has no expression!");
            let condition_idx = piece.condition.expect("Piece condition is empty!");
            let condition_result = evaluate_condition(nodes, condition_idx, values, functions)?;
            if condition_result {
                let expr_result = evaluate_node(nodes, expr_idx, values, functions)?;
                Ok((true, Some(expr_result)))
            } else {
                Ok((false, None))
            }
        }
        _ => {
            //dbg!(head);
            Err("haha couldn't parse".to_string())
        }
    }
}

pub fn evaluate_condition(
    nodes: &[MathNode],
    head_idx: NodeIndex,
    values: &HashMap<String, f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<bool, String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Constant(constantnode) => {
            if let Some(constant) = constantnode.constant {
                match constant {
                    Constant::False => Ok(false),
                    Constant::True => Ok(true),
                    _ => Err("haha".to_string()),
                }
            } else {
                Err("hh".to_string())
            }
        }
        MathNode::Apply(apply) => {
            let op_result = apply.get_op(nodes);
            let mut result = None;
            // If this is a regular mathematical operator, go ahead
            if let Ok(op) = op_result {
                let mut a = None;
                let mut b = None;
                match op {
                    Op::Eq | Op::Neq | Op::Geq | Op::Leq | Op::Gt | Op::Lt => {
                        if apply.operands.len() != 2 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        a = Some(evaluate_node(nodes, apply.operands[0], values, functions)?);
                        b = Some(evaluate_node(nodes, apply.operands[1], values, functions)?);
                    }
                    _ => {}
                }
                if let Some(first_value) = a {
                    if let Some(second_value) = b {
                        match op {
                            Op::Eq => {
                                result = Some((first_value - second_value).abs() <= f64::EPSILON)
                            }
                            Op::Neq => {
                                result = Some((first_value - second_value).abs() > f64::EPSILON)
                            }
                            Op::Gt => result = Some(first_value > second_value),
                            Op::Lt => result = Some(first_value < second_value),
                            Op::Geq => result = Some(first_value >= second_value),
                            Op::Leq => result = Some(first_value <= second_value),
                            _ => {}
                        }
                    }
                }
            }
            if let Some(value) = result {
                Ok(value)
            } else {
                Err("Invalid operator".to_string())
            }
        }
        _ => {
            let error = format!("Couldn't evaluate operator {}", head);
            Err(error)
        }
    }
}
