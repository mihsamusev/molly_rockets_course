## Performance aware programming course notes and drafts
Course link: https://www.computerenhance.com/p/table-of-contents

## Useful links
- [Andrew Kelly - Practical DOD](https://vimeo.com/649009599)
:
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

## 8086 mov
`mov` actually a copy content of one register to another
`mov <dest>,<source>`

register moves ax,bx to move all 16 bit, al,bl to move lower 8bits, and ah, bh to move higher 8 bits.
binary instruction stream for move:

bit pattern [100010][Dbit][Wbit]-[2][3][3] that converts to move assembly instruction

### Homework 1
Binary instruction stream
preview: `xxd -b <file>`
find my architecture: `uname -m` -> x86_64
dissasemble binary machine code stream: `objdump -D -b binary -m i386:x86-64 -M intel muliple_move.bin`
how to do .asm to machine code

### Homework 2
- 16 bit registers that each have high / low 8 bit parts
    - AX AH:AL Accumulator
    - BX BH:BL Base register
    - CX CH:CL Counting register
    - DX DH:DL Data register

- 16 bit only integers
    - SP - stack pointer
    - BP - base pointer
    - SI - source index
    - BI - base index


```asm
mov bx, [75] # Read 76th element in memory to bx
mov [75], bx # Write bx to 76th element in memory
```

Effective address calculation, from position of bp (base pointer)
```asm
mov bx, [bp + 75]
```

mod field will describe a displacement for a memory ivolved move
00 -> no displacement excep for rm of 110
01 -> 8 bit displacement
10 -> 16 bit displacement
11 -> register to register move

rm field - which equation for memory address calculation to use

## immediate mov
Immediate to register mov
[1011][wide][REG] [LO][HI]

mov [BP + 75], byte 12 -> move as signed byte
mov [BP + 75], word 12 -> move as signed 2 bytes low|high

One caveat is that the following 2 are same binary, since unless actual arithmetic is performed all values are unsigned.
mov cx, 12
mov cx, -12


## Feature of 8086
Because `add`, `sub`, `cmp` is encoded as mov add can manipulate any memory / register combo
```asm
add bx, 12
```

```asm
mov ax, [bp]
mov bx, [bp + 2]
add ax, bx
```

```asm
mov ax, [bp]
add ax, [bp + 2]
```

## movs and arithmetics cheet sheet
Arithmetics
```
P Op           Description
0 ADD L, E     L += E
2 ADC L, E     L += E + CF
5 SUB L, E     L -= E
3 SBB L, E     L -= E + CF
7 CMP L, E     (void)(L - E)
1 OR L, E      L |= E
4 AND L, E     L &= E
6 XOR L, E     L ^= E
    # arithmetics between registers / addresses
    0P0 xrm          Op Eb, Rb
    0P1 xrm          Op Ew, Rw
    0P2 xrm          Op Rb, Eb
    0P3 xrm          Op Rw, Ew

    # immediate to accumulator
    0P4 Db           Op AL, Db
    0P5 Dw           Op AX, Dw
    
    # immediate to address / register
    200 xPm Db       Op Eb, Db
    201 xPm Dw       Op Ew, Dw
    203 xPm Dc       Op Ew, Dc
```

Movs
```
MOV L, E       L = E;
    # move between registers / addresses
    210 xrm         mov Eb, Rb
    211 xrm         mov Ew, Rw
    212 xrm         mov Rb, Eb
    213 xrm         mov Rw, Ew
    214 xsm         mov Es, SR   (s = 0-3,   (#) 4-5)
    216 xsm         mov SR, Es   (s = 0,2-3, (#) 4-5)
    
    # immediate to accumulator
    240 Dw          mov AL, [Dw]
    241 Dw          mov AX, [Dw]
    242 Dw          mov [Dw], AL
    243 Dw          mov [Dw], AX

    # immediate to register
    26r Db          mov Rb, Db
    27r Dw          mov Rw, Dw

    # immediate to address
    306 x0m Db      mov Eb, Db
    307 x0m Dw      mov Ew, Dw
```