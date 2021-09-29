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
    //dbg!(values);
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
                    Op::Quotient => {
                        if apply.operands.len() != 2 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        let b = evaluate_node(nodes, apply.operands[1], values, functions)?;
                        Ok(((a as i32) / (b as i32)) as f64)
                    }
                    Op::Rem => {
                        if apply.operands.len() != 2 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        let b = evaluate_node(nodes, apply.operands[1], values, functions)?;
                        Ok(((a as i32) % (b as i32)) as f64)
                    }
                    Op::Min => {
                        let mut min_result = f64::MAX;
                        for i in 0..apply.operands.len() {
                            let current =
                                evaluate_node(nodes, apply.operands[i], values, functions)?;
                            min_result = f64::min(min_result, current);
                        }
                        Ok(min_result)
                    }
                    Op::Max => {
                        let mut max_result = f64::MIN;
                        for i in 0..apply.operands.len() {
                            let current =
                                evaluate_node(nodes, apply.operands[i], values, functions)?;
                            max_result = f64::max(max_result, current);
                        }
                        Ok(max_result)
                    }
                    Op::Exp => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        Ok(a.exp())
                    }
                    Op::Ln => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        Ok(a.ln())
                    }
                    // TRIGONOMETRIC FUNCTIONS
                    Op::Sin
                    | Op::Cos
                    | Op::Tan
                    | Op::Sec
                    | Op::Csc
                    | Op::Cot
                    | Op::Sinh
                    | Op::Cosh
                    | Op::Tanh
                    | Op::Sech
                    | Op::Csch
                    | Op::Coth
                    | Op::Arcsin
                    | Op::Arccos
                    | Op::Arctan
                    | Op::Arcsec
                    | Op::Arccsc
                    | Op::Arccot
                    | Op::Arcsinh
                    | Op::Arccosh
                    | Op::Arctanh
                    | Op::Arcsech
                    | Op::Arccsch
                    | Op::Arccoth => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;

                        if op == Op::Sin {
                            Ok(a.sin())
                        } else if op == Op::Cos {
                            Ok(a.cos())
                        } else if op == Op::Tan {
                            Ok(a.tan())
                        } else if op == Op::Sec {
                            Ok(1.0 / a.cos())
                        } else if op == Op::Csc {
                            Ok(1.0 / a.sin())
                        } else if op == Op::Cot {
                            Ok(1.0 / a.tan())
                        } else if op == Op::Sinh {
                            Ok(a.sinh())
                        } else if op == Op::Cosh {
                            Ok(a.cosh())
                        } else if op == Op::Tanh {
                            Ok(a.tanh())
                        } else if op == Op::Sech {
                            Ok(1.0 / a.cosh())
                        } else if op == Op::Csch {
                            Ok(1.0 / a.sinh())
                        } else if op == Op::Coth {
                            Ok(1.0 / a.tanh())
                        } else if op == Op::Arcsin {
                            Ok(a.asin())
                        } else if op == Op::Arccos {
                            Ok(a.acos())
                        } else if op == Op::Arctan {
                            Ok(a.atan())
                        } else if op == Op::Arcsec {
                            Ok((1.0 / a).acos())
                        } else if op == Op::Arccsc {
                            Ok((1.0 / a).asin())
                        } else if op == Op::Arccot {
                            if a > 0.0 {
                                Ok((1.0 / a).atan())
                            } else {
                                Ok(std::f64::consts::PI + (1.0 / a).atan())
                            }
                        } else if op == Op::Arcsinh {
                            Ok(a.asinh())
                        } else if op == Op::Arccosh {
                            Ok(a.acosh())
                        } else if op == Op::Arctanh {
                            Ok(a.atanh())
                        } else if op == Op::Arcsech {
                            Ok(1.0 / a.acosh())
                        } else if op == Op::Arccsch {
                            Ok(1.0 / a.asinh())
                        } else if op == Op::Arccoth {
                            Ok(1.0 / a.atanh())
                        } else {
                            Err("Invalid trig operator.".to_string())
                        }
                    }
                    Op::Abs => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        let a = evaluate_node(nodes, apply.operands[0], values, functions)?;
                        Ok(a.abs())
                    }
                    Op::And
                    | Op::Or
                    | Op::Xor
                    | Op::Implies
                    | Op::Eq
                    | Op::Neq
                    | Op::Geq
                    | Op::Leq
                    | Op::Gt
                    | Op::Lt => {
                        let condition_result =
                            evaluate_condition(nodes, head_idx, values, functions)?;
                        if condition_result {
                            Ok(1.0)
                        } else {
                            Ok(0.0)
                        }
                    }
                    _ => panic!("Evaluation not supported for operator."),
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
                        res = Some(evaluate_lambda(
                            lambda,
                            0,
                            &argument_values,
                            values,
                            functions,
                        )?);
                        //dbg!(lambda_name, res);
                    }
                }
                if let Some(value) = res {
                    Ok(value)
                } else {
                    panic!("Invalid operator");
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
                    panic!("Wrong type");
                }
            }
            Some(NumType::Real) | None => {
                if let Some(Number::Real(r)) = cn.value {
                    Ok(r)
                } else {
                    panic!("Wrong type");
                }
            }
            Some(NumType::Rational) => {
                if let Some(Number::Rational(x, y)) = cn.value {
                    Ok((x as f64) / (y as f64))
                } else {
                    panic!("Wrong type");
                }
            }
            Some(NumType::ENotation) => {
                if let Some(Number::ENotation(x, y)) = cn.value {
                    Ok(x * 10.0_f64.powf(y as f64))
                } else {
                    panic!("Wrong type");
                }
            }
            _ => panic!("Wrong type"),
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
        MathNode::Constant(constant_node) => match constant_node.constant {
            Some(Constant::Pi) => Ok(std::f64::consts::PI),
            Some(Constant::ExponentialE) => Ok(std::f64::consts::E),
            Some(Constant::True) => Ok(1.0),
            Some(Constant::False) => Ok(0.0),
            _ => Err("Invalid constant".to_string()),
        },

        _ => {
            panic!("Couldn't evaluate operator {}", head);
        }
    }
}

pub fn evaluate_lambda(
    nodes: &[MathNode],
    head_idx: NodeIndex,
    argument_values: &[f64],
    values: &HashMap<String, f64>,
    functions: &HashMap<String, Vec<MathNode>>,
) -> Result<f64, String> {
    let head = nodes[head_idx].clone();
    match head {
        MathNode::Root(root) => {
            if root.children.len() != 1 {
                return Err("Root with multiple/zero children!".to_string());
            }
            evaluate_lambda(nodes, root.children[0], argument_values, values, functions)
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
                let res = evaluate_node(nodes, lambda.expr.unwrap(), &assignments, functions)?;
                Ok(res)
            }
        }
        _ => evaluate_node(nodes, head_idx, values, functions),
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
            panic!("haha couldn't parse");
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
            panic!("haha couldn't parse");
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
                    _ => panic!("haha"),
                }
            } else {
                panic!("hh");
            }
        }
        MathNode::Apply(apply) => {
            let op_result = apply.get_op(nodes);
            let mut result = None;
            // If this is a regular mathematical operator, go ahead
            if let Ok(op) = op_result.clone() {
                let mut operand_results = Vec::<f64>::new();
                let mut child_condition_results = Vec::<bool>::new();
                match op {
                    Op::Eq | Op::Neq | Op::Geq | Op::Leq | Op::Gt | Op::Lt => {
                        if Op::Neq == op {
                            if apply.operands.len() != 2 {
                                return Err("Invalid number of operands.".to_string());
                            }
                        }
                        for operand_location in &apply.operands {
                            operand_results.push(evaluate_node(
                                nodes,
                                *operand_location,
                                values,
                                functions,
                            )?);
                        }
                    }
                    Op::And | Op::Or | Op::Xor => {
                        for operand_location in &apply.operands {
                            child_condition_results.push(evaluate_condition(
                                nodes,
                                *operand_location,
                                values,
                                functions,
                            )?);
                        }
                    }
                    Op::Not => {
                        if apply.operands.len() != 1 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        for operand_location in &apply.operands {
                            child_condition_results.push(evaluate_condition(
                                nodes,
                                *operand_location,
                                values,
                                functions,
                            )?);
                        }
                    }
                    Op::Implies => {
                        if apply.operands.len() != 2 {
                            return Err("Invalid number of operands.".to_string());
                        }
                        for operand_location in &apply.operands {
                            child_condition_results.push(evaluate_condition(
                                nodes,
                                *operand_location,
                                values,
                                functions,
                            )?);
                        }
                    }
                    _ => {}
                }

                let condition_count = child_condition_results.len();
                let true_count = child_condition_results.iter().filter(|x| **x).count();
                match op {
                    Op::Eq => {
                        let mut x = true;
                        for i in 1..operand_results.len() {
                            let current =
                                (operand_results[i - 1] - operand_results[i]).abs() <= f64::EPSILON;
                            x = x && current;
                            if !x {
                                break;
                            }
                        }
                        result = Some(x);
                    }
                    Op::Neq => {
                        let current =
                            (operand_results[0] - operand_results[1]).abs() > f64::EPSILON;
                        result = Some(current);
                    }
                    Op::Gt => {
                        let mut x = true;
                        for i in 1..operand_results.len() {
                            let current = operand_results[i - 1] > operand_results[i];
                            x = x && current;
                            if !x {
                                break;
                            }
                        }
                        result = Some(x);
                    }
                    Op::Lt => {
                        let mut x = true;
                        for i in 1..operand_results.len() {
                            let current = operand_results[i - 1] < operand_results[i];
                            x = x && current;
                            if !x {
                                break;
                            }
                        }
                        result = Some(x);
                    }
                    Op::Geq => {
                        let mut x = true;
                        for i in 1..operand_results.len() {
                            let current = operand_results[i - 1] >= operand_results[i];
                            x = x && current;
                            if !x {
                                break;
                            }
                        }
                        result = Some(x);
                    }
                    Op::Leq => {
                        let mut x = true;
                        for i in 1..operand_results.len() {
                            let current = operand_results[i - 1] <= operand_results[i];
                            x = x && current;
                            if !x {
                                break;
                            }
                        }
                        result = Some(x);
                    }
                    Op::And => result = Some(condition_count == true_count),
                    Op::Or => result = Some(true_count > 0),
                    Op::Xor => {
                        result = Some(true_count % 2 == 1);
                    }
                    Op::Implies => {
                        // It evaluates to false if
                        //      the first argument is true and
                        //      the second argument is false,
                        // otherwise it evaluates to true.
                        result = Some(!(child_condition_results[0] && !child_condition_results[1]));
                    }
                    Op::Not => {
                        result = Some(!(child_condition_results[0]));
                    }
                    Op::Times
                    | Op::Plus
                    | Op::Minus
                    | Op::Divide
                    | Op::Power
                    | Op::Ceiling
                    | Op::Floor
                    | Op::Factorial => {
                        let evaluation_result = evaluate_node(nodes, head_idx, values, functions)?;
                        // return true if evaluation_result != 0
                        result = Some((evaluation_result - 0.0).abs() > f64::EPSILON);
                    }
                    _ => {}
                }
            }
            if let Some(value) = result {
                Ok(value)
            } else {
                panic!("Condition not supported");
            }
        }
        _ => {
            let result = evaluate_node(nodes, head_idx, values, functions)?;
            Ok(result != 0.0)
        }
    }
}
