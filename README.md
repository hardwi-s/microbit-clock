# microbit-clock
Implements a binary clock using microbit LED matrix. The display is in BCD digits with each column a digit - e.g.
13:47 is represented as

0 0 0 0 0

0 0 0 0 0

0 0 0 1 1

0 1 0 0 1

1 1 0 0 1

1 3 : 4 7

The centre column is a pulsing hyphen.
