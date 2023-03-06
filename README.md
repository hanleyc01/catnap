# Catnap ✧/ᐠ-ꞈ-ᐟ\ 
--------

I've tried to implement my own cute little functional programming language for a long while now,
and I keep hitting a wall. I like to call this collection of project [*Carnap*](https://en.wikipedia.org/wiki/Rudolf_Carnap) after my favorite philosopher and logician. Cats are really cute too, and I enjoy naps whenever I get the chance.

I'm absolutely fascinated by interactive proof assistants and theorem provers,
so I thought that it'd be cool to attempt at writing my own:

I'll enumerate some of the goals I have in mind for the project.

## Goals

- [x] Have fun /ᐠ｡ꞈ｡ᐟ✿\
- [ ] Learn about the Calculus of Inductive Constructions
- [ ] Create an interpreted intermediate representation based off of [Henk](https://www.microsoft.com/en-us/research/wp-content/uploads/1997/01/henk.pdf)
  + [x] Get an up and running lexer and parser
  + [ ] Optimize parser
	- [ ] Fix inability to recognize multiple `tvar`'s in "application" expressions (e.g., `|~| (<tvar_1>) ... (<tvar_n>)`, or mixed parenthesized expressions
	- [ ] Work on a framework that provides helpful and informative error messages which don't make you stare at an `unwrap`'d value for 20 minutes
  + [ ] Typechecking of Henk programs, as well as some basic type inference (the previously referenced article which gives the Henk specification provides some basic inference rules which ought to provide some helpful tips and strategies for type inference and checking)
  + [ ] Compilation and interpreting
	- [ ] Investigate possible bytecode formats, in particular, check out OCaml VM, BEAM, Lua VM
	- [ ] Possible transpiling of bytecode to LLVM or Cranelift (whole task unto itself)
  + [ ] Flesh out the standard library a bit
- [ ] Implement core, verified kernel from which to expand to a full proof writing language

## Far-off Goals, like from here to the moon and back x100
- [ ] Optimize core language and compilation, and explore compilation to machine code
- [ ] See about writing own proof writing application
- [ ] Machine learning for proof writing based off of stored proof metadata (Coq stores proofs, so 
look into that  /ᐠ≗ᆽ≗ᐟ \)
