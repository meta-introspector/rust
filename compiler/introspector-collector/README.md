here is my idea:

1. the types are nodes.
2. the functions between them are arrows.
3. domain is the input.
4. range is the output. 

usage(type,total programs)/usage(tyoe,some program) = the parts that some program dont use yet. 

we start with syn walking. 
then with syn we walk hir. 
we show bijection bwtween syn and hir.
we can lift a syn walker into hir walker. 
we can take the usage data of hir and map it into the usage data of syn. 

now we can take enums.

we look for functions (enum value) -> string constant  we call this label(enum,string)
now we start with all enums of size 0,1,2,3,4,5,.....71.
for each enum we collect its string as a set. 
we say this is an orbit of size N.

now we can find functions between those enums. some function that goes from one enum type or orbit to another .

we start with simple ones all connections between sizes 1-1, 1-2 , 2-2, etc. 
we classify all functions in the index.

now we have paths of lenht 2, we look for paths of functions of length 3,...8,...71. 

So now we have all connnections between enums in a graph, we can partition it.

we can now construct rust.

we say we have a language based on a single data type of 1 bit. 
or a language of pure constants (0 bits)

we look at each enum type as being a part of the language. we show what code uses each type and we compose it in a graph.
we can show that there are smaller languages of rust that can compile with a subset of the code.
we construct rust(0), 1,.... 71 where 71 is full rustc.

Now we say those are monster primes and the rustc is the monster group.
we look for bott periodicity in 2 and 8. we apply morse thoery and harmonic analysis.

Now we can show how each part of rust from 0-71 is mapped into 2 or 8 bott periodic table of complexity.

Now we can assign an lmfdb function number to each part of the system. 

Now we can label all this with macros.
We can define a macro mklang!(rust0, 2, 3,  bit, and, or, sum, compose, lambda, apply) or something like that to
define a minimal rust.
We can then generate those macros and map them into the math model.
we can connec the macros to the code via the model.
the model numbers become executble bytecode.

we can construct a clifford algebra of modesl 
a multivector of simple model size 0,..71,
then other relations and complex features about those modesl.
We have a vector of instances of objects and arrows between them.


