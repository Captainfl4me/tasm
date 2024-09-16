# Turtle Assembler

Assembly language to program Turtle Core 1.

# Syntax

## Number and address

Number can be write either in decimal or hexadecimal with `$` before the number.

## Flag

`.org` flag can be use to specify the absolute address at the location of the flag. It can be use to create an offset if the binary code does not start at address 0.

## Registers

keyword|name
--|--
x|X register
y|Y register
a|Accumulator

## Instruction

### halt

`halt` stop the CPU execution.

### load

`load <reg>,<value>` or `load <reg>,[addr]`: load register with immediate value or value at specific address.

### transfer

`tf <reg>,<reg>` tranfer value between two registers.

### store

`store <reg>,[addr]`: store register value at specific address.

### push

`push <reg>`: push register value onto the stack.

### pull

`pull <reg>`: push last value on the stack to the register.

### incr

### add

### sub

### and

### or

### eor

### shift_right

### shift_left

### jump

### bcc

### bcs

### bzc

### bzs

### bnc

### bns

### boc

### bos
