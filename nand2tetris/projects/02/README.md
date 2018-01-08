# 加法器
二进制的加法并不难实际上还是真值表

    halfadd
    |   a   |   b   |  sum  |  carry|
    |   0   |   0   |   0   |   0   |
    |   0   |   1   |   1   |   0   |
    |   1   |   0   |   1   |   0   |
    |   1   |   1   |   0   |   1   |

很明显 sum(a,b) = xor(a,b) carry(a,b) = and(a,b)

有了这个基础的组件(半加器) 我们就可以继续向上封装

    // a+b+c 的最大值为 11
    CHIP FullAdder {
        IN a, b, c;  // 1-bit inputs
        OUT sum,     // Right bit of a + b + c
            carry;   // Left bit of a + b + c

        PARTS:
        HalfAdder(a=a,b=b,sum=sab,carry=cab);
        HalfAdder(a=sab,b=c,sum=sum,carry=csab);
        HalfAdder(a=cab,b=csab,sum=carry,carry=trash);
    }
在向上封装的多位加法器同理

    (carry,out[0]) = fulladd(a[0],b[0],0);
    (carry,out[1]) = fulladd(a[1],b[1],carry);
                etc
    (carry,out[15]) = fulladd(a[15],b[15],carry);

# ALU 
我自己实现的ALU 就比较不讲究了 实际上就是将每种可能都算一遍 最后选出真正的结果
```
// if (zx == 1) set x = 0        // 16-bit constant
// if (nx == 1) set x = !x       // bitwise not
// if (zy == 1) set y = 0        // 16-bit constant
// if (ny == 1) set y = !y       // bitwise not
// if (f == 1)  set out = x + y  // integer 2's complement addition
// if (f == 0)  set out = x & y  // bitwise and
// if (no == 1) set out = !out   // bitwise not
// if (out == 0) set zr = 1
// if (out < 0) set ng = 1

CHIP ALU {
    IN  
        x[16], y[16],  // 16-bit inputs        
        zx, // zero the x input?
        nx, // negate the x input?
        zy, // zero the y input?
        ny, // negate the y input?
        f,  // compute out = x + y (if 1) or x & y (if 0)
        no; // negate the out output?

    OUT 
        out[16], // 16-bit output
        zr, // 1 if (out == 0), 0 otherwise
        ng; // 1 if (out < 0),  0 otherwise

    PARTS:

//prepare input x y    
    Zero16(in=x,s=zx,out=rx);
    Zero16(in=y,s=zy,out=ry);
    Not2Way16(in=rx,sel=nx,out=rrx);
    Not2Way16(in=ry,sel=ny,out=rry);

//calc
    Add16(a=rrx,b=rry,out=raddout);
    And16(a=rrx,b=rry,out=randout);
    
    Mux16(a=randout,b=raddout,sel=f,out=rout);
    

//prepare output
    Not2Way16(in=rout,sel=no,out=rrout);
   
    IsZero16(in=rrout,out=zr);
    IsNeg16(in=rrout,out=ng);
    Out16(in=rrout,out=out);
}
```
