
mod z80;
mod parser;

/*
 * TODO???
 * faire du code plus fonctionnel: chaque itération renvoie un nouvel objet (cpu, registres, ...)
 * Seule la mémoire a un état qui est modifié
 */
/* TODO
 * 1- faire la boucle fetch-decode-execute [v]
 * 2- faire les opérations sur la mémoire [WIP]
 * 3- faire les opcodes [WIP]
 * 3.5- Vérifier qu'il y a bien les fonctions reset où il y a besoin
 * 4- graphics
 * 5- historique (optionel) de ce que font les différentes parties du code
 * 6- multithread???
 */

// FIXME : transformer les instructions de &mut CPU -> () à CPU -> CPU, idem pour les operations
// sur les registres
// FIXME : faire en sorte que le CPU n'ai qu'une ref vers la mémoire
// FIXME add login
// FIXME split part into threads?

#[cfg(test)]
mod tests {
    use super::parser;
    #[test]
    fn parse_identifier() {
        let (rest, id) = parser::parser::identifier("tests1").unwrap();
        assert!(match id {
            parser::parser::Expression::Identifier(s) => s == "tests1",
            _ => false
        });
        assert_eq!(rest, "");

        assert!(match parser::parser::identifier("-1s") {
            Ok(_) => false,
            Err(_) => true,
        });
    }

    #[test]
    fn parse_binop() {
        assert!(match parser::parser::binary_operator("+") {
            Ok((_, parser::parser::BinOp::Add)) => true,
            _ => false
        });
        assert!(match parser::parser::binary_operator("-") {
            Ok((_, parser::parser::BinOp::Sub)) => true,
            _ => false
        });
        assert!(match parser::parser::binary_operator(">") {
            Ok((_, parser::parser::BinOp::RShift)) => true,
            _ => false
        });
        assert!(match parser::parser::binary_operator("<") {
            Ok((_, parser::parser::BinOp::LShift)) => true,
            _ => false
        });
        assert!(match parser::parser::binary_operator("*") {
            Err(_) => true,
            _ => false
        });
    }

    #[test]
    fn parse_bin_operation() {
        use parser::parser::{ Expression, BinOp };
        assert!(
            match parser::parser::binary_operation("foo + bar") {
                Ok(( _, Expression::BinaryOperation(b))) =>
                    match *b {
                        (Expression::Identifier(l),
                         BinOp::Add,
                         Expression::Identifier(r)) => l == "foo" && r == "bar",
                        _ => false, }
                _ => false,
            }
        );
    }

    #[test]
    fn parse_funcall() {
        use parser::parser::{funcall, Expression};
        match funcall("(funcname arg1 arg2)") {
            Ok((_, Expression::FunctionCall(name, args))) => {
                assert!(name == "funcname", "The function name wasn't parsed correctly");
                assert!(args.len() == 2 ,
                        "The arguments weren't parsed correctly: there is not the right number of arguments");
                match &args[0] {
                    Expression::Identifier(a) => assert!(a == "arg1", "The first argument name wasn't parsed correctly"),
                    _ => assert!(false,  "The first argument wasn't parsed correctly")
                };
                match &args[0] {
                    Expression::Identifier(a) => assert!(a == "arg1",
                                                         "The first argument name wasn't parsed correctly"),
                    _ => assert!(false,  "The first argument wasn't parsed correctly"),
                };
            },
            Ok(_) => assert!(false, "The function call wasn't parsed correctly: the wrong Expression was returned"),
            Err(e) => assert!(false, "There was an error when parsing the function call: {}", e),
        };
    }
}

fn main() {
    // Init the different part of the emulator
    let rom = std::fs::read("rc/ttt.gb").unwrap();
    let mut memory = z80::memory::MMU::new(rom[0..32000].to_vec().clone());
    let mut cpu = z80::cpu::CPU::new();
    let opcodemap = z80::opcodes::OpcodeMap::new();
    let mut cycle_count = 0;
    let mut gpu = z80::gpu::GPU::default();
    loop {
        // fetch
        println!(">>>> Cycle {}", cycle_count);
        let op = memory.read_byte(cpu.registers.pc, &cpu.registers);
        println!("Got {} from the memory", op);
        println!("Registers: \n{}", cpu.registers.format());
        cpu.registers.pc += 1;
        // decode + execute
        cpu.registers = opcodemap.map(op.into(), cpu.registers, &mut memory);
        cpu.update_clock();
        // FIXME add stop condition
        cycle_count += 1;
        gpu = gpu.step(&cpu, &memory);

    }
}
