use std::cmp::PartialEq;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Get,
}

#[derive(Debug, PartialEq)]
pub struct RunData {
    pub operation: Operation,
    pub site: String,
}

pub fn parse_command_line(arguments: &Vec<String>) -> Result<RunData, String> {
    if arguments.len() < 3 {
        Err(String::from("Incorrect number of arguments."))
    } else {
        let subcommand = &arguments[1];
        let operation = match subcommand.as_str() {
            "add" => Ok(Operation::Add),
            "get" => Ok(Operation::Get),
            _ => Err(String::from(format!("Invalid operation: {}", subcommand)))
        }?;
        Ok(RunData {
            operation: operation,
            site: arguments[2].clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add() {
        let expected = RunData {
            operation: Operation::Add,
            site: String::from("mysite"),
        };
        let input = vec![String::from("pwdb"), String::from("add"), String::from("mysite")];
        let result = parse_command_line(&input);
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_parse_get() {
        let expected = RunData {
            operation: Operation::Get,
            site: String::from("asite"),
        };
        let input = vec![String::from("pwdb"), String::from("get"), String::from("asite")];
        let result = parse_command_line(&input);
        assert_eq!(expected, result.unwrap());
    }
}
