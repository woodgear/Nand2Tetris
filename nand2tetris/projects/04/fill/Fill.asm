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

// @16 const color the color of draw
// @17 var counter this line while while draw
// @18 var pos this current brush postion


(SELECT)
@KBD
D=M  //获取键盘输入到D中
@NOTHAS
D;JEQ //如果为0 (未输入 重新开始拿)
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


@256      //how much line we draw
D=A
@counter
M=D

@SCREEN
D=A
@pos
M=D //set pos to start of screen

//start draw
(LOOP) //开始循环


@color 
D=M    //get color

@pos
A=M  //set start pos

M=D
A=A+1 //draw color etc

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
//draw one line over

@32
D=A
@pos
M=M+D //update pos

@counter
M=M-1 //sub couter we draw over one line

D=M
@LOOP
D;JGT

@SELECT
0;JMP