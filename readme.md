# MowItNow

Objective of this technical exercise is to implement the given specifications in any high-level language.

I chose to use Rust 🦀

I started by definning what the mower needs like lawn's size, its position in this garden, etc. After drawning a garden and the mower's movements I determined the directions based on current orientation.  

```
Lawn's size:

5 |   |   |   |   |   |   |
---------------------------
4 |   |   |   |   |   |   |
---------------------------
3 |   |   |   |   |   |   |
---------------------------
2 |   |   |   |   |   |   |
---------------------------
1 |   |   |   |   |   |   |
---------------------------
0 |   |   |   |   |   |   |
---------------------------
  | 0 | 1 | 2 | 3 | 4 | 5 |


Orientation:

   N
   |
W--o--E
   |
   S


New orientation based on current orientation (Left - Current - Right):
W-N-E
N-E-S
S-W-N
E-S-W


New position based on orientation:
N -> (x, y++)
E -> (x++, y)
W -> (x--, y)
S -> (x, y--)

```

At this point it was clear for me that is was not useful to implement an object for the lawn. It was obvious mower needed the garden's size instead. Later it may be helpful to implement methods so that handle its position in this area.

Rust provides what you need to write object-oriented code like `struct`, `trait` and `enum`. Iterator and `match` are also useful commands I like to use.

The next improvements I see:
- implementing a CLI,
- handling the mower's position so that avoid leaving the area, 
- handling error,
- ~~translating commands in english (G->L, D->R),~~
and perhaps another helpful commands.