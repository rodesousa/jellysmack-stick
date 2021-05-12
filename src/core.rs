use crate::error::*;
use std::io;

type Line = Vec<char>;
type Pipe = Vec<(usize, usize)>;

// Reads the file and return head/tail pair and pipes
pub fn engine() -> Result<(Line, Line, Pipe), StickError> {
    let mut input_line = String::new();
    match io::stdin().read_line(&mut input_line) {
        Err(_) => Err(StickError::InputError),
        Ok(_) => {
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let width = init(inputs[0])?;
            let height = init(inputs[1])?;

            if width < 3 || height > 100 {
                exit(StickError::ConstraintError)
            }

            let (head, tail, result) = function_name(width, height)?;
            Ok((head, tail, result))
        }
    }
}

pub fn solution(head: Line, mut tail: Line, result: Pipe) -> (Line, Line) {
    for i in result.iter().rev() {
        tail.swap(i.0, i.1)
    }

    (head, tail)
}

fn init(input: &str) -> Result<i32, StickError> {
    match input.trim().parse::<i32>() {
        Ok(i) => Ok(i),
        Err(_) => Err(StickError::InitError),
    }
}

fn read_lines(width: i32) -> Result<String, StickError> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lines = input_line.trim().to_string();

    if width as usize == lines.len() {
        Ok(lines)
    } else {
        Err(StickError::WidthMissMatchError(lines.len(), width))
    }
}

fn function_name(width: i32, height: i32) -> Result<(Line, Line, Pipe), StickError> {
    let mut result: Pipe = Vec::new();
    let mut head: Line = Vec::new();
    let mut tail: Line = Vec::new();
    let mut before_pipe;

    for i in 0..height as usize {
        before_pipe = false;
        if i == 0 {
            head = read_pair();
        } else if i == (height - 1) as usize {
            tail = read_pair();
        } else {
            let raw = read_lines(width)?;
            let line_splitted = raw.split("|").collect::<Vec<_>>();

            for (i, v) in line_splitted.iter().enumerate() {
                let mut line = v.chars().collect::<Line>();
                line.dedup();

                if line.len() > 1 {
                    return Err(StickError::IllegalError);
                } else if is_border(line_splitted.len(), i) {
                    if !line.is_empty() {
                        return Err(StickError::FormatError);
                    }
                } else if str::is_empty(v) {
                    return Err(StickError::FormatError);
                } else if char::is_whitespace(line[0]) {
                    before_pipe = false;
                } else if line[0] == '-' {
                    if before_pipe {
                        return Err(StickError::DoublePipeError);
                    }
                    result.push((i - 1, i));
                }
            }
        }
    }

    match pair_contraints(&head, &tail) {
        Ok(_) => Ok((head, tail, result)),
        Err(error) => Err(error),
    }
}

fn is_border(len: usize, id: usize) -> bool {
    id + 1 == len || id == 0
}

fn pair_contraints(head: &Line, tail: &Line) -> Result<(), StickError> {
    if head.len() != tail.len() {
        return Err(StickError::LengthError);
    }

    let mut concatenated = [&head[..], &tail[..]].concat();

    let len = concatenated.len();
    concatenated.dedup();

    if len != concatenated.len() {
        return Err(StickError::DuplicatePairError);
    }

    Ok(())
}

fn read_pair() -> Line {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().nth(0).unwrap())
        .collect::<Line>()
}
