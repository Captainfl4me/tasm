# Turtle Assembler

Assembly language to program Turtle Core 1.

# Syntax

## Number and address

Number can be written either in decimal or hexadecimal with `$` before the number.

## Flag

`.org <ADDR>` flag can be use to specify the absolute address at the location of the flag. It can be used to create an offset if the binary code does not start at address 0.

`.include "<PATH>"` compile and include TASM file into the current file. Can be nested. Relative path are resolve relative to file.

## Registers

keyword|name
--|--
rx|X register
ry|Y register
ra|A register for ALU
rb|B register for ALU

## Instruction

### halt

`halt` stop the CPU execution.

### load

`load <reg>,<value>` or `load <reg>,[addr]`: load register with immediate value or value at specific address.

### transfer

`tf <reg>,<reg>` transfer value between two registers.

### store

`store <reg>,[addr]`: store register value at specific address.

### push

`push <reg>`: push register value onto the stack.

### pull

`pull <reg>`: push last value on the stack to the register.

### incr

`RA + 1 -> RA`

`incr`: Increment register A.

### add

`RA + RB -> RA`

`add`: Add register A and register B using carry flag as carry input and set carry and overflow flag. Result is put in register A.

### sub

`RA - RB -> RA`

`sub`: Substract register B from register A using carry flag as carry input and set carry and overflow flag. Result is put in register A.

### and

`RA and RB -> RA`

`and`: And operation bitwise between register A and B.

### or

`RA or RB -> RA`

`or`: Or operation bitwise between register A and B.

### eor

`RA xor RB -> RA`

`eor`: Exclusive or operation bitwise between register A and B.

### shift_right

`RA >> 1 -> RA`

`shift_right`: Shift register A 1 bit to the right.

### shift_left

`RA << 1 -> RA`

`shift_left`: Shift register A 1 bit to the left.

### jump

`[ADDR] -> PC`

`jump [addr]` jump to address without branch condition.

### bcc

`[ADDR] -> PC`

`bcc [addr]` jump to address if carry flag is clear.

### bcs

`[ADDR] -> PC`

`bcs [addr]` jump to address if carry flag is set.

### bzc

`[ADDR] -> PC`

`bzc [addr]` jump to address if zero flag is clear.

### bzs

`[ADDR] -> PC`

`bzs [addr]` jump to address if zero flag is set.

### bnc

`[ADDR] -> PC`

`bnc [addr]` jump to address if negative flag is clear.

### bns

`[ADDR] -> PC`

`bns [addr]` jump to address if negative flag is set.

### boc

`[ADDR] -> PC`

`boc [addr]` jump to address if overflow flag is clear.

### bos

`[ADDR] -> PC`

`bos [addr]` jump to address if overflow flag is set.

### jsr

`PC+2 -> <stack>`
`[ADDR] -> PC`

`jsr [addr]` jump to subroutine at address and store PC+2 onto stack.

### rts

`<stack> -> PC`

`rts` return from subroutine by fetching PC value on the stack.
