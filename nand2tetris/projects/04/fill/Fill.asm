// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.
(SELECT)
@KBD
D=M
@NOTHAS
D;JEQ
(HAS)
D=-1
@SED
0;JMP
(NOTHAS)
D=0
(SED)
@color
M=D
//get color over


//start draw
@32   //total num 32 is legth of line
D=A
@counter
M=D
(LOOP)
@32
D=A
@counter
M=M-D
D=M

@pos
M=D
@SCREEN
D=A
@pos
M=M+D
@color
D=M

@pos
A=M
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D
A=A+1
M=D

@counter
D=M
@LOOP
D;JGT

@SELECT
0;JMP