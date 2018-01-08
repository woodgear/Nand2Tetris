# 布尔函数数量
Q: 给定N个布尔变量,能构造出多少不同布尔函数?  
A: $2^{2^{n}}$  
W: 
不同的布尔函数指的是输入值域内所有值每次都不会产生相同的输入  
N个变量 输入值域数量为$2^{n}$ 假设其为X 现在的问题就是对于一个X位的容器 能够有多少种不同变化 $2^{x}$ 所以答案是$2^{2^{n}}$  
power by [how many semantically different boolean functions are there for n boolean variables](https://math.stackexchange.com/a/505522)

# 基本
## Nand
基本门逻辑中 最基本的是Nand
```
|   a   |   b   |  out  |
|   0   |   0   |   1   |
|   0   |   1   |   1   |
|   1   |   0   |   1   |
|   1   |   1   |   0   |
```
如何实现Nand是物理上的事情 我们直接假设Nand的存在
## Not
通过Nand可以直接构造出Not 
```
// Not 真值表
|   a   |  out  |
|   0   |   1   |
|   1   |   1   |
```
Not(a) = Nand(a, a);
```
|   a   |   b   |  out  |
|   0   |   0   |   1   | 
|   1   |   1   |   0   |
```

## And
有了Not Nand 之后就可以直接实现 And 毕竟 Nand = Not And
And(a, b) = Not(Nand(a, b))

## Or
```
|   a   |   b   |  out  |
|   0   |   0   |   0   |
|   0   |   1   |   1   |
|   1   |   0   |   1   |
|   1   |   1   |   1   |
```
按照规范表示法 Or(a, b) = ^ab + a^b + ab 但是直接看表 Or就是 ^ (^a + ^b)

## Xor
```
|   a   |   b   |  out  |
|   0   |   0   |   0   |
|   0   |   1   |   1   |
|   1   |   0   |   1   |
|   1   |   1   |   0   |
```
Xor(a, b) = ^ab + a^b

# 选择
看上去像是选择实际上是通过计算模拟出效果
## Multiplexor 
    out = a if sel == 0
        b otherwise

    |   a   |   b   |  sel  |  out  |
    |   0   |   0   |   0   |   0   |
    |   0   |   0   |   1   |   0   |
    |   0   |   1   |   0   |   0   |
    |   0   |   1   |   1   |   1   |
    |   1   |   0   |   0   |   1   |
    |   1   |   0   |   1   |   0   |
    |   1   |   1   |   0   |   1   |
    |   1   |   1   |   1   |   1   |
### 规范表示法  
mux(a,b,s) = ^abs + a^b^s + ab^s + abs

### 能不能更给力一点啊
mux(a,b,s) = and(or(s,a),or(^s,b))   

结果是由被选择的值决定的 我们不能让另一个值干扰到结果 另一个值应恒为1

    |   a   |   b   |  sel  |  out  |
    |   a   |   0   |   0   |   a   |
    |   a   |   1   |   0   |   a   |
    |   a   |   0   |   1   |   1   |
    |   a   |   1   |   1   |   1   |
    or(s,a)

    |   a   |   b   |  sel  |  out  |
    |   0   |   b   |   0   |   1   |
    |   1   |   b   |   0   |   1   |
    |   0   |   b   |   1   |   b   |
    |   1   |   b   |   1   |   b   |
    or(^s,b)

    |  sel  |or(s,a)| or(^s,b)|  out  |
    |   1   |   1   |   b     |   b   |
    |   0   |   a   |   1     |   a   |
    and(or(s,a),or(^s,a))
按照相同的思路 另一个值恒为0也是可行的
    |   a   |   b   |  sel  |  out  |
    |   a   |   0   |   0   |   a   |
    |   a   |   1   |   0   |   a   |
    |   a   |   0   |   1   |   0   |
    |   a   |   1   |   1   |   0   |
    and(^s,a)

    |   a   |   b   |  sel  |  out  |
    |   0   |   b   |   0   |   0   |
    |   1   |   b   |   0   |   0   |
    |   0   |   b   |   1   |   b   |
    |   1   |   b   |   1   |   b   |
    or(s,b)

    |  sel  |and(^s,a)| and(s,b) |  out  |
    |   1   |   0     |   b      |   b   |
    |   0   |   a     |   0      |   a   |
    or(and(s,b),and(^s,a))

## Mux4Way
    out = a if sel == 00
        b if sel == 01
        c if sel == 10
        d if sel == 11
通过s0决定出是(a b) 还是(c,d) 然后通过s1决定出最终的值

    Mux(s0,a,c,out=r1);
    Mux(s0,b,d,out=r2);
    Mux(s1,r1,r2,out=out);
## Demultiplexor
    {a, b} = {in, 0} if sel == 0
             {0, in} if sel == 1

    |  in   |  sel  |   a   |   b   |
    |   0   |   0   |   0   |   0   |
    |   0   |   1   |   0   |   0   |
    |   1   |   0   |   1   |   0   |
    |   1   |   1   |   0   |   1   |

有了Mux 之后 DMux 就比较容易了  

    Mux(a = in, b = 0,  sel=sel, out=a)
    Mux(a = 0,  b = in, sel=sel, out=b)

# 多位 
复制粘贴即可
