

Program Pipeline:

Source Text File -> cera-parse (emits a token stream) -> cera-src-ast (emits
an AST close to the base source file) -> cera-simple-form (transforms the AST
into a different kind of AST, that removes all the synactic sugar) -> 
cera-comptime (compile time execution, concrete type emission) -> cera-ir (emits 
CeraIR, an SSA representation with ISA independant intrinsics) -> cera-bingen 
(emits the object file)

Imports are stored and resolved in the simple-form AST
