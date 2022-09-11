
mod z80;

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
