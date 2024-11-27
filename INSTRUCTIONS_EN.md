MowItNow
========

The repository contains a Rust 🦀 implementation of the "MowItNow" mower exercise.

<b>Exercise Statement:</b>

MowItNow has decided to develop an automatic lawn mower designed for rectangular surfaces. The mower can be programmed to traverse the entire surface.

The position of the mower is represented by a combination of coordinates (x, y) and a letter indicating its orientation based on the English cardinal notation (N, E, W, S). The lawn is divided into a grid to simplify navigation.

For example, the mower's position might be "0, 0, N", meaning it is located at the bottom-left corner of the lawn, oriented north.

To control the mower, a simple sequence of letters is sent. Possible letters are "D", "G", and "A". "D" and "G" rotate the mower 90° to the right or left, respectively, without moving it. "A" means advancing the mower by one square in its current facing direction, without changing its orientation.

If the position after a move is outside the lawn, the mower does not move, retains its orientation, and processes the next command.

We assume that the square directly north of position (x, y) has coordinates (x, y+1).

To program the mower, an input file is provided with the following structure:

- The first line corresponds to the coordinates of the top-right corner of the lawn, with the bottom-left corner assumed to be (0,0).
- The rest of the file is used to control all deployed mowers. Each mower has two related lines:
  - The first line specifies the mower's initial position and orientation. The position and orientation are given as two numbers and a letter, separated by spaces.
  - The second line is a series of instructions directing the mower to explore the lawn. The instructions are a sequence of characters without spaces.

Each mower moves sequentially, meaning the second mower does not move until the first has completed its entire set of instructions.

When a mower completes a set of instructions, it communicates its final position and orientation.

<b>OBJECTIVE</b>

Design and write a program in Rust 🦀 that implements the above specifications and passes the following test.

<b>TEST</b>

The following input file is provided:
```
5 5
1 2 N
GAGAGAGAA
3 3 E
AADAADADDA
```
The expected result (final positions of the mowers) is:
```
1 3 N
5 1 E
```