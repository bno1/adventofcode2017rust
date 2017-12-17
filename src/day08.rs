extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;


type Word = i32;

enum Operand {
    Reg(String),
    Const(Word)
}

struct Condition {
    func: fn(Word, Word) -> bool,
    oper1: Operand,
    oper2: Operand
}

enum Instruction {
    Add{ reg: String, delta: Word, cond: Condition }
}

struct CPU {
    regs: HashMap<String, Word>,
    max_reg: Option<Word>
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            regs: HashMap::new(),
            max_reg: None
        }
    }

    pub fn get_operand(&self, op: &Operand) -> Word {
        match *op {
            Operand::Reg(ref reg) => self.regs.get(reg).map_or(0, |v| *v),
            Operand::Const(val) => val
        }
    }

    pub fn modify_reg<F: FnOnce(&mut Word) -> ()>(&mut self, reg: &str, f: F)
    {
        let rval = self.regs.entry(String::from(reg)).or_insert(0);
        f(rval);

        let max_val = self.max_reg.get_or_insert(*rval);
        *max_val = std::cmp::max(*max_val, *rval);
    }

    pub fn check_cond(&self, cond: &Condition) -> bool {
        let a = self.get_operand(&cond.oper1);
        let b = self.get_operand(&cond.oper2);

        (cond.func)(a, b)
    }

    pub fn run_instr(&mut self, instr: &Instruction) {
        match *instr {
            Instruction::Add{ref reg, delta, ref cond} =>
                if self.check_cond(cond) {
                    self.modify_reg(reg.as_str(), |r| *r += delta)
                }
        }
    }

    pub fn run_program<'a, T>(&mut self, prog: T)
    where T: Iterator<Item=&'a Instruction>
    {
        for instr in prog {
            self.run_instr(instr);
        }
    }
}

struct LineParser {
    re_instr: Regex,
    re_const: Regex,
    re_reg: Regex
}

impl LineParser {
    pub fn new() -> Result<LineParser, regex::Error> {
        let re_instr =
            match Regex::new(concat!(
                // register     operation    amount      if
                r"([a-zA-Z]+)\s+(inc|dec)\s+([+-]*\d+)\s+if\s+",
                // op1        condition        op2
                r"(\S+)\s+(>|>=|==|!=|<|<=)\s+(\S+)"
            ))
        {
            Ok(re) => re,
            Err(e) => return Err(e)
        };

        let re_const = match Regex::new(r"^[+-]*[0-9]+$") {
            Ok(re) => re,
            Err(e) => return Err(e)
        };

        let re_reg = match Regex::new(r"^[a-zA-Z]+$") {
            Ok(re) => re,
            Err(e) => return Err(e)
        };

        Ok(LineParser {
            re_instr: re_instr,
            re_const: re_const,
            re_reg: re_reg
        })
    }

    fn parse_oper(&self, l: &str) -> Result<Operand, String> {
        if self.re_const.is_match(l) {
            Ok(Operand::Const(l.parse().unwrap()))
        } else if self.re_reg.is_match(l) {
            Ok(Operand::Reg(String::from(l)))
        } else {
            Err(format!("'{}' is not a constant or register", l))
        }
    }

    pub fn parse(&self, l: &str) -> Result<Instruction, String> {
        let captures = match self.re_instr.captures(l) {
            Some(c) => c,
            None => return Err(format!("'{}' is not a valid instruction", l))
        };

        let reg = captures.get(1).unwrap().as_str();
        let sign = match captures.get(2).unwrap().as_str() {
            "inc" => 1,
            "dec" => -1,
            x => return Err(format!("Unexpected instruction '{}'", x))
        };
        let delta: Word = captures.get(3).unwrap().as_str().parse().unwrap();

        let cond_a = match self.parse_oper(captures.get(4).unwrap().as_str()) {
            Ok(oper) => oper,
            Err(e) => return Err(e)
        };

        let cond_f: fn(Word, Word) -> bool =
            match captures.get(5).unwrap().as_str() {
                ">" => |a, b| a > b,
                ">=" => |a, b| a >= b,
                "==" => |a, b| a == b,
                "!=" => |a, b| a != b,
                "<=" => |a, b| a <= b,
                "<" => |a, b| a < b,
                x => return Err(format!("Unexpected comparison '{}'", x))
            };

        let cond_b = match self.parse_oper(captures.get(6).unwrap().as_str()) {
            Ok(oper) => oper,
            Err(e) => return Err(e)
        };

        Ok(Instruction::Add {
            reg: String::from(reg),
            delta: sign * delta,
            cond: Condition {
                func: cond_f,
                oper1: cond_a,
                oper2: cond_b
            }
        })
    }
}

fn main() {
    let line_parser = LineParser::new().unwrap();
    let stdin = io::stdin();

    let input: Vec<Instruction> = stdin
        .lock()
        .lines()
        .map(|l| line_parser.parse(l.unwrap().as_str()).unwrap())
        .collect();

    let mut cpu = CPU::new();
    cpu.run_program(input.iter());

    println!("Soltuion 1: {}", cpu.regs.values().max().unwrap());
    println!("Soltuion 2: {}", cpu.max_reg.unwrap());
}
