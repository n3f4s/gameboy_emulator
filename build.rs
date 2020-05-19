// use std::{
//     fs,
//     io::{prelude::*, BufReader},
// };
// extern crate regex;
// use regex::Regex;

fn main() {
//     println!("cargo:rerun-if-changed=src/z80/instructions.rs");
//     let filename = "src/z80/instructions.rs";
//     let file = fs::File::open(filename).expect("no such file");
//     let regex = Regex::new("pub fn (.*)\\(.*\\).*").unwrap();
//     let functions = BufReader::new(file)
//         .lines()
//         .map(|l| l.expect("Couldn't parse"))
//         .filter(|l| regex.is_match(l))
//         .map(|l| String::from( &regex.captures(&l).unwrap()[1] ) )
//         .collect::<Vec<String>>();
//     let size = functions.len();
//     let fn_str = functions.iter()
//         .map(|l| format!("Box::new({})", &l) )
//         .collect::<Vec<String>>()
//         .join(",\n");
//     let name_str = functions.iter()
//         .map(|l| format!("\"{}\"", &l) )
//         .collect::<Vec<String>>()
//         .join(",\n");
//     let string = format!( "#![allow(dead_code)] // FIXME remove
// #![allow(unused_variables)] // FIXME remove
// use z80::cpu::CPU;
// use z80::instructions::*;
// 
// pub type Opcode = usize; // FIXME adapt to the actual number of opcode
// pub type Instruction = fn(&mut CPU);
// 
// pub struct OpcodeMap<'a> {{
//     pub op_map: [Box<Instruction>; {size}],
//     pub op_name: [&'a str; {size}],
// }}
// 
// impl<'a> OpcodeMap<'a> {{
//     pub fn new() -> OpcodeMap<'a> {{
//         OpcodeMap {{
//             op_map: [ {map} ],
//             op_name: [ {name} ],
//         }}
//     }}
// }}", size = size, map = fn_str, name = name_str );
//     fs::write("src/z80/opcodes.rs", string).unwrap();
}
