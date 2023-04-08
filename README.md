## Performance aware programming course notes and drafts
Course link: 

## CPU internals awareness
No matter the level of programming language / programming style or framework we are using, it all ends in CPU performing the instructions prepared by the compiler / interpreter of the language. There are 2 main ways to make a CPU bound operation faster for a single CPU:
- reduce number of instrutions, reduce waste not associated with computing task
- make CPU peform multiple instructions at the same time using ILP (instruction level parallelism) and SIMD (single instruction multiple data)


## Instruction level paralellism


https://docs.rs/iterator_ilp/latest/iterator_ilp/

## Systems level Rust tutorials for CPU
Rust versus optimized C++ - https://parallel-rust-cpp.github.io/
Brainfuck compilers in rust - https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md