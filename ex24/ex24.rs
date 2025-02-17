use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

#[derive(Debug)]
enum OpType {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Operation {
    input1: String,
    input2: String,
    output: String,
    op_type: OpType,
}

fn parse_op_type(token: &str) -> Option<OpType> {
    match token {
        "AND" | "and" => Some(OpType::And),
        "OR" | "or" => Some(OpType::Or),
        "XOR" | "xor" => Some(OpType::Xor),
        _ => None,
    }
}

fn evaluate_op(op_type: &OpType, input1: bool, input2: bool) -> bool {
    match op_type {
        OpType::And => input1 && input2,
        OpType::Or => input1 || input2,
        OpType::Xor => input1 != input2,
    }
}

fn compute(operation: &Operation, binaries: & mut HashMap<String, bool>, operations: &HashMap<String, Operation>) -> bool {
    if !binaries.contains_key(&operation.input1) {
        let result = compute(operations.get(&operation.input1).unwrap().deref(), binaries, operations);
        binaries.insert(operation.input1.clone(), result);
    }
    if !binaries.contains_key(&operation.input2) {
        let result = compute(operations.get(&operation.input2).unwrap().deref(), binaries, operations);
        binaries.insert(operation.input1.clone(), result);
    }
    let result = evaluate_op(&operation.op_type, binaries[&operation.input1], binaries[&operation.input2]);
    binaries.insert(operation.output.clone(), result);
    result
}

fn compute_iter(operation: &Operation, binaries: &mut HashMap<String, bool>, operations: &HashMap<String, Operation>) -> bool {
    let mut stack = Vec::new();
    let mut current = operation;

    loop {
        if !binaries.contains_key(&current.input1) {
            if let Some(op) = operations.get(&current.input1) {
                stack.push(current);
                current = op;
                continue;
            }
        }

        if !binaries.contains_key(&current.input2) {
            if let Some(op) = operations.get(&current.input2) {
                stack.push(current);
                current = op;
                continue;
            }
        }

        let result = evaluate_op(
            &current.op_type,
            *binaries.get(&current.input1).unwrap(),
            *binaries.get(&current.input2).unwrap(),
        );
        binaries.insert(current.output.clone(), result);

        if let Some(next_op) = stack.pop() {
            current = next_op;
        } else {
            return result;
        }
    }
}



fn binary_to_decimal(binary: &[bool]) -> u128 {
    binary.iter()
        // .rev()
        .enumerate()
        .fold(0, |acc, (i, &bit)| {
            acc + if bit { 1 << i } else { 0 }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(binary_to_decimal(&[false]), 0);
        assert_eq!(binary_to_decimal(&[true]), 1);
        assert_eq!(binary_to_decimal(&[true, false]), 1);
        assert_eq!(binary_to_decimal(&[false, true]), 2);
        assert_eq!(binary_to_decimal(&[true, true]), 3);
        assert_eq!(binary_to_decimal(&[true, false, true]), 5);
        assert_eq!(binary_to_decimal(&[true, true, true]), 7);
        assert_eq!(binary_to_decimal(&[true, true, true]), 7);
        let mut test = [true,true,true,true,false, true,true,true,true,true,false];
        test.reverse();
        assert_eq!(binary_to_decimal(&test), 1982);
    }
}


fn main() -> std::io::Result<()> {
    let file = File::open("ex24/input.txt")?;
    let reader = BufReader::new(file);

    let mut binaries: HashMap<String, bool> = HashMap::new();

    let mut operations: HashMap<String, Operation> = HashMap::new();

    for line_res in reader.lines() {
        let line = line_res?.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.contains(':') {
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let value = match parts[1] {
                    "1" => true,
                    "0" => false,
                    _ => {
                        println!("Warning: unexpected binary value '{}'", parts[1]);
                        continue;
                    }
                };
                binaries.insert(name, value);
            }
        } else if line.contains("->") {
            let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                let output_name = parts[1].to_string();
                let operation_parts: Vec<&str> = parts[0].split_whitespace().collect();

                if operation_parts.len() == 3 {
                    let (input1, op_str, input2) = (operation_parts[0], operation_parts[1], operation_parts[2]);
                    if let Some(op) = parse_op_type(op_str) {
                        let op_data = Operation {
                            input1: input1.to_string(),
                            input2: input2.to_string(),
                            output: output_name.clone(),
                            op_type: op,
                        };
                        operations.insert(output_name, op_data);
                    } else {
                        println!("Warning: unknown operation '{}'", op_str);
                    }
                } else {
                    println!("Warning: Operands not recognized in line '{}'", line);
                }
            }
        }
    }
    
    let mut stack: Vec<&Operation> = Vec::new();

    for (output_name, operation) in operations.iter().filter(|(k, _)| k.starts_with('z')) {
        println!("Processing operation for output '{}': {:?}", output_name, operation);
        stack.push(operation);
    }

    let mut operation_results: HashMap<String, bool> = HashMap::new();
    while !stack.is_empty() {
        let operation = stack.pop().unwrap();
        let result = compute_iter(operation, &mut binaries, &operations);
        println!("Computed result for operation '{}': {}", operation.output, result);
        operation_results.insert(operation.output.clone(), result);
    }

    let mut z_outputs: Vec<String> = operation_results.keys()
        .filter(|k| k.starts_with('z'))
        .cloned()
        .collect();
    z_outputs.sort();
    println!("z_outputs: {:?}", z_outputs);
    let binary_values: Vec<bool> = z_outputs.iter()
        .map(|key| operation_results[key])
        .collect();


    let decimal_value = binary_to_decimal(&binary_values);
    println!("Decimal value: {}", decimal_value);


    Ok(())
}