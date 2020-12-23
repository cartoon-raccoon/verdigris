use linefeed::{
    Interface, 
    terminal::DefaultTerminal,
    reader::ReadResult,
};

use crate::vm::VM;
use crate::repl::lexer::*;

/// The repl for the low-level IR (assembly) of Verdigris.
pub struct Repl {
    vm: VM,
    interface: Interface<DefaultTerminal>,
    lexer: AsmLexer,
}

impl Repl {
    pub fn new() -> Self {
        let lr = Interface::new("vdg-asm").unwrap();
        lr.set_prompt(">>> ").unwrap();
        Self {
            vm: VM::new(vec![]),
            interface: lr,
            lexer: AsmLexer::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Oxidizer Shell v0.1.0");
        println!("Type .help for a list of commands.");
        loop {
            match self.read_line() {
                Ok(exec) => {
                    if let Some(exec) = exec {
                        match exec {
                            Executable::Command(cmd) => {
                                self.exec_cmd(cmd);
                                continue
                            }
                            Executable::Instruction(bytes) => {
                                self.vm.add_bytes(bytes);
                                if let Ok(done) = self.vm.run_once() {
                                    if done {
                                        std::process::exit(0)
                                    } 
                                }
                            }
                        }
                    } else {continue}
                }
                Err(e) => {
                    eprintln!("{}", e);
                    continue
                }
            }
        }
    }

    fn read_line(&mut self) -> Result<Option<Executable>, AsmLexErr> {
        if let ReadResult::Input(line) = self.interface.read_line()? {
            self.interface.add_history(line.clone());
            if let Some(exec) = self.lexer.parse(line.to_lowercase())? {
                return Ok(Some(exec))
            }
        }
        return Ok(None)
    }

    fn exec_cmd(&self, cmd: ReplCmd) {
        match cmd {
            ReplCmd::Registers => {
                self.vm.dump_registers();
            }
            ReplCmd::Program => {
                self.vm.dump_program();
            }
            ReplCmd::Help => {
                unimplemented!()
            }
            ReplCmd::Info => {
                unimplemented!()
            }
            ReplCmd::Quit => {
                std::process::exit(0);
            }
        }
    }
}

