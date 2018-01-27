#[macro_use]
extern crate failure;
extern crate file;
#[macro_use]
extern crate maplit;

mod errors;
use errors::{Error, Result};
use std::env;
use std::str::FromStr;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
struct ACmd(i32);

#[derive(Debug)]
struct CCmd(Dest, Comp, Jump);
#[derive(Debug, Clone)]
struct Dest(String);
#[derive(Debug, Clone)]
struct Comp(String);
#[derive(Debug, Clone)]
struct Jump(String);

impl FromStr for Dest {
    type Err = errors::Error;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let s = s.to_uppercase();
        let dest = match s.as_ref() {
            "NULL" => "000",
            "M" => "001",
            "D" => "010",
            "MD" => "011",
            "A" => "100",
            "AM" => "101",
            "AD" => "110",
            "AMD" => "111",
            _ => "",
        };
        if dest.is_empty() {
            Err(Error::ParseCmdFailError(format!(
                "could not parser the dest -> {}",
                s
            )))
        } else {
            Ok(Dest(dest.to_owned()))
        }
    }
}

impl FromStr for Comp {
    type Err = errors::Error;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let s = s.to_uppercase();
        let comp = match s.as_ref() {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => "",
        };
        if comp.is_empty() {
            Err(Error::ParseCmdFailError(format!(
                "could not parser the comp -> {}",
                s
            )))
        } else {
            Ok(Comp(comp.to_owned()))
        }
    }
}

impl FromStr for Jump {
    type Err = errors::Error;
    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let s = s.to_uppercase();
        let jump = match s.as_ref() {
            "NULL" => "000",
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            _ => "",
        };
        if jump.is_empty() {
            Err(Error::ParseCmdFailError(format!(
                "could not parser the jump -> {}",
                s
            )))
        } else {
            Ok(Jump(jump.to_owned()))
        }
    }
}

impl FromStr for CCmd {
    type Err = errors::Error;
    fn from_str(cmd: &str) -> ::std::result::Result<Self, Self::Err> {
        match (cmd.contains("="), cmd.contains(";")) {
            (true, true) => {
                let cmds: Vec<&str> = cmd.split(|c| c == '=' || c == ';').collect();
                let dest: &str = cmds.get(0).ok_or(Error::ParseCmdFailError(format!(
                    "could not find dest of ccmd {}",
                    cmd
                )))?;
                let dest = Dest::from_str(dest)?;
                let comp = cmds.get(1).ok_or(Error::ParseCmdFailError(format!(
                    "could not find comp of ccmd {}",
                    cmd
                )))?;
                let comp = Comp::from_str(comp)?;

                let jump = cmds.get(2).ok_or(Error::ParseCmdFailError(format!(
                    "could not find jump of ccmd {}",
                    cmd
                )))?;
                let jump = Jump::from_str(jump)?;
                Ok(CCmd(dest, comp, jump))
            }
            (true, false) => {
                let cmds: Vec<&str> = cmd.split(|c| c == '=').collect();
                let dest: &str = cmds.get(0).ok_or(Error::ParseCmdFailError(format!(
                    "could not find dest of ccmd {}",
                    cmd
                )))?;
                let dest = Dest::from_str(dest)?;
                let comp = cmds.get(1).ok_or(Error::ParseCmdFailError(format!(
                    "could not find comp of ccmd {}",
                    cmd
                )))?;
                let comp = Comp::from_str(comp)?;
                let jump = Jump::from_str("NULL")?;
                Ok(CCmd(dest, comp, jump))
            }
            (false, true) => {
                let cmds: Vec<&str> = cmd.split(|c| c == ';').collect();
                let dest = Dest::from_str("NULL")?;
                let comp = cmds.get(0).ok_or(Error::ParseCmdFailError(format!(
                    "could not find comp of ccmd {}",
                    cmd
                )))?;
                let comp = Comp::from_str(comp)?;
                let jump = cmds.get(1).ok_or(Error::ParseCmdFailError(format!(
                    "could not find jump of ccmd {}",
                    cmd
                )))?;
                let jump = Jump::from_str(jump)?;
                Ok(CCmd(dest, comp, jump))
            }
            (false, false) => {
                return Err(Error::ParseCmdFailError(format!(
                    "must has dest or jump -> {}",
                    cmd
                )))
            }
        }
    }
}

trait ToHackInstruction {
    fn to_instruction(&self) -> String;
}

impl ToHackInstruction for ACmd {
    fn to_instruction(&self) -> String {
        format!("0{:015b}", self.0)
    }
}
impl ToHackInstruction for CCmd {
    fn to_instruction(&self) -> String {
        format!("111{}{}{}", (self.1).0, (self.0).0, (self.2).0)
    }
}

#[derive(Debug)]
struct Symbols {
    symbols: HashMap<String, i32>,
    variable_start: usize,
}

impl Symbols {
    fn new(variable_start: usize) -> Self {
        let mut symbols: HashMap<String, i32> = hashmap!{
            "SP".to_owned() => 0x0,
            "LCL".to_owned() => 0x1,
            "ARG".to_owned() => 0x2,
            "THIS".to_owned() => 0x3,
            "THAT".to_owned() => 0x4,
            "SCREEN".to_owned() => 0x4000,
            "KBD".to_owned() => 0x6000,
            "R0".to_owned() =>0x0,
        };

        for index in 0..16 {
            symbols.insert(format!("R{}", index), index);
        }

        Symbols {
            symbols,
            variable_start,
        }
    }
    fn has(&self, key: &str) -> bool {
        return self.symbols.contains_key(key);
    }
    fn get(&self, key: &str) -> Option<&i32> {
        return self.symbols.get(key);
    }
    fn add_val(&mut self, key: &str) {
        self.symbols
            .insert(key.to_owned(), self.variable_start as i32);
        self.variable_start += 1;
    }
    fn add_label(&mut self, label: &str, index: i32) {
        self.symbols.insert(label.to_owned(), index);
    }
}

fn assembler(asm: &str) -> Result<String> {
    let cmds: Vec<&str> = asm.lines()
        .into_iter()
        .map(|s| s.trim())
        .filter(|s| !(s.is_empty() || s.starts_with("//")))
        .map(|s| {
            let res: Vec<&str> = s.split("//").collect();
            res[0].trim()
        })
        .collect();

    let mut symbols = Symbols::new(16);

    let mut label_index = 0;
    let cmds: Vec<&str> = cmds.into_iter()
        .filter(|s| {
            if s.starts_with("(") && s.ends_with(")") {
                let label = s.trim_matches(|c| c == '(' || c == ')');
                symbols.add_label(label, label_index);
                return false;
            }
            label_index += 1;
            return true;
        })
        .collect();

    for s in cmds.iter() {
        if s.starts_with("@") {
            let s = s.trim_left_matches('@');
            if s.contains(char::is_alphabetic) {
                if !symbols.has(s) {
                    symbols.add_val(s)
                }
            }
        }
    }

    let cmds: Result<Vec<Box<ToHackInstruction>>> = cmds.into_iter()
        .enumerate()
        .map(|(_index, s)| {
            // println!("index {} cmd {}", _index, s);
            if s.starts_with("@") {
                let s = s.trim_left_matches('@');
                if s.contains(char::is_alphabetic) {
                    symbols
                        .get(s)
                        .ok_or(Error::ParseCmdFailError(format!(
                            "could not find sysbol {}",
                            s
                        )))
                        .and_then(|val| Ok(Box::new(ACmd(*val)) as Box<ToHackInstruction>))
                } else {
                    i32::from_str(s)
                        .and_then(|val| Ok(Box::new(ACmd(val)) as Box<ToHackInstruction>))
                        .map_err(|e| {
                            Error::ParseCmdFailError(format!(
                                "Acmd could not parser the numnber {}",
                                e.to_string()
                            ))
                        })
                }
            } else if s.contains("=") || s.contains(";") {
                CCmd::from_str(s).and_then(|ccmd| Ok(Box::new(ccmd) as Box<ToHackInstruction>))
            } else {
                Err(Error::ParseCmdFailError(format!(
                    "could not parser this cmd {}",
                    s
                )))
            }
        })
        .collect();
    let cmds = cmds?;
    let cmds: Vec<String> = cmds.into_iter().map(|c| c.to_instruction()).collect();
    Ok(cmds.join("\r\n"))
}

fn main() {
    let asm_path = env::args().nth(1).unwrap();
    let asm_path = Path::new(&asm_path);
    let asm = file::get_text(asm_path).unwrap();
    let res = assembler(&asm).unwrap();
    let mut hack_path = asm_path.to_path_buf();
    let _ = hack_path.set_extension("hack.wc");
    file::put_text(&hack_path, res).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assembler() {
        let paths = [
            "../add/Add",
            "../max/Max",
            "../max/MaxL",
            "../rect/Rect",
            "../rect/RectL",
            "../pong/Pong",
            "../pong/PongL",
        ];

        for path in paths.iter() {
            let data = file::get_text(&format!("{}.asm", path)).unwrap();
            let expect = file::get_text(&format!("{}.hack", path)).unwrap();
            let expect = expect.trim();
            let res = assembler(&data).unwrap();
            assert_eq!(expect, res);
        }
    }
}
