# Oxidizer
## The Verdigris runtime

This is the VM and runtime for the Verdigris programming language.

The main package tokenizes and parses the language, compiling it down to Verdigris bytecode. The Oxidizer VM then runs the bytecode.

Oxidizer also provides a REPL for the direct execution of Verdigris assembly.

`vdg -a / --asm`