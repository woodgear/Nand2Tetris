# CPU

当a位发生变化时唯一变得就是A/M 所以可以用Mux切换 
X: A/M Y:D

所有的功能ALU都已经实现了

    |   | zy | ny | zx | nx | f | no |
    | 0 | 1  | 0  | 1  | 0  | 1 | 0  |
    | 1 | 1  | 1  | 1  | 1  | 1 | 1  |
    | -1| 1  | 1  | 1  | 0  | 1 | 0  |  -1 = -1 + 0
    | D | 0  | 0  | 1  | 1  | 0 | 0  |  Y = -1 & Y
    | A | 1  | 1  | 0  | 0  | 0 | 0  |  X = -1 & X
    | D | 0  | 0  | 1  | 1  | 0 | 1  |
    | A | 1  | 1  | 0  | 0  | 0 | 1  |
    | -D| 0  | 1  | 1  | 0  | 1 | 0  | -Y = 
    | -A| 1  | 0  | 0  | 1  | 1 | 0  | -X
    |D+1| 0  | 0  | 1  | 1  | 1 | 1  | Y+1 
    |A+1| 1  | 1  | 0  | 0  | 1 | 1  | X+1
    |D-1| 0  | 0  | 0  | 1  | 1 | 0  | Y-1
    |A-1| 1  | 1  | 0  | 0  | 1 | 0  | X-1
    |D+A| 0  | 0  | 0  | 0  | 1 | 0  | X+Y
    |D-A| 0  | 1  | 0  | 0  | 1 | 1  | Y-X Y+(-X)
    |A-D| 0  | 0  | 0  | 1  | 1 | 1  | X-Y X+(-Y)
    |A&D| 0  | 0  | 0  | 0  | 0 | 0  | X&Y
    |A|D| 0  | 1  | 0  | 1  | 0 | 1  | X|Y

$0 = 0 + 0$

$1 = \overline { \overline 0 + \overline 0 }$

$-1 = \overline {  0 + 0 }$

$X = X + 0$

$Y = Y + 0$

$X + Y = X + Y$

$X - Y = \overline {  \overline X + Y }$

$\because-Y=\overline Y +1$

$\therefore \overline {  \overline X + Y } = \overline {  (-X-1)+Y } = -(-X-1+Y)-1 = X-Y$

$-Y = \overline{Y-1 }$

$\because-Y=\overline Y +1$

$\therefore \overline{Y-1 } = -(Y-1)-1 = -Y+1-1 =-Y$

所以 c1=zy c2=ny c3=zx c4=nx c5=f c6=no

# CPU
在写的过程中会发现 选择实际上是根本不存在的 但是确实是有选择的 一切的选择都基于Mux 不论是 无时序的 还是有时序的(多了个DFF) 下层结构对上提供抽象 由一个一个原子组合成更大的原子 再继续组合 从下而上  

另外我认为组合芯片的过程实际上写意大利面条式的代码差不多

选择 a?b:c 实际上就是

$$
f(a,b,s)= 
\begin{cases} 
a,\text{if s=0}\\
b,\text{if s=1}
\end{cases}
$$
a,b 的位数实际上无关紧要的 (一般都是总线宽度)
再加上通过lockFlag 来存储寄存器 那么一切的基石就已经很稳妥了

Mux DFF 通过其构建出选择逻辑 CPU 实际上就很简单了
