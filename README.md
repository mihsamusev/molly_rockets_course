## Performance aware programming course notes and drafts
Course link: https://www.computerenhance.com/p/table-of-contents

## CPU internals awareness
No matter the level of programming language / programming style or framework we are using, it all ends in CPU performing the instructions prepared by the compiler / interpreter of the language. There are 3 main ways to make a CPU bound operation faster for a single CPU:
- reduce number of instrutions, reduce waste not associated with computing task or use SIMD (single instruction multiple data)
- increase througput, make CPU peform multiple instructions at the same time using ILP (instruction level parallelism) and be mindfull of loading values into registers by utilizing caches L1 (32K) / L2 (256K) / L3 (8M) (lookups whether the location from main memory that we wanna perform arithmetic with happens to be in one of the caches already)


## Interaction between cache and multithreading
If memory is not involved then we can expect the number of cores to be the upper bound of improvement running the same code in multithreaded setting. However if running the code on multiple threads will fit the data into the higher tier cache like L1 compared to the same code on single threaded, the expected improvement multiplier will be more than the number of cores!

Example, working witg 4094 4 bit integers will fit entirely to L1, so spreading that task to multiple cores will make cores work on sizes less than L1, no speed up from caching, speedup from sharing the work between cores and some cost for thread management. In case of 16384 integers on a single thread we have to operate out of L2 cache, however, split into 4 threads will make all 4 thread operate on 4094 numbers which fits into L1.

## Instruction level paralellism

https://docs.rs/iterator_ilp/latest/iterator_ilp/

## Systems level Rust tutorials for CPU
Rust versus optimized C++ - https://parallel-rust-cpp.github.io/
Brainfuck compilers in rust - https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md

## Sum example
Some useful binary exploration tools
```bash
gcc -O0 sum.c-o sum 
gcc -S -masm=intel sum.c # to make assembly file with Intel mnemonic
objdump -t sum # preview all symbols in binary
objdump -d -M intel sum # disassembly and use Intel mnemonic
objdump -d -M intel sum | cat --language=asm # syntax highligt with batcat
```

rsp - register stack pointer - points to the top of the stack memory, decremented to allocate the stack memory  
rbp - register base ponter / frame pointer - on function call its set to current value of stack pointer


## Haversine algorithm
Come up with the estimate of how fast can we do 10m haversine distanes with all performance multipliers?

## 8086
`mov` actually a copy content of one register to another
`mov <dest>,<source>`

register moves ax,bx to move all 16 bit, al,bl to move lower 8bits, and ah, bh to move higher 8 bits.
binary instruction stream for move:

bit pattern [100010][Dbit][Wbit]-[2][3][3] that converts to move assembly instruction

## Homework 1
Binary instruction stream
preview: `xxd -b <file>`
find my architecture: `uname -m` -> x86_64
dissasemble binary machine code stream: `objdump -D -b binary -m i386:x86-64 -M intel muliple_move.bin`
how to do .asm to machine code?