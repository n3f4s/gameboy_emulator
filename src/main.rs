
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

// FIXME add login
// FIXME split part into threads?

fn main() {
    // Init the different part of the emulator
    let memory = z80::memory::MMU::new(std::fs::read("jsGB/tests/ttt.gb").unwrap());
    let mut cpu = z80::cpu::CPU::new(memory);
    let opcodemap = z80::opcodes::OpcodeMap::new();
    loop {
        // fetch
        let op = cpu.mmu.read_byte(cpu.registers.pc, &cpu.registers);
        println!("Got {} from the memory", op);
        cpu.registers.pc += 1;
        // decode + execute
        opcodemap.map(op.into(), &mut cpu);
        cpu.update_clock();
        // FIXME add stop condition
    }
}
