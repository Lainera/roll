# They see me rolling... d20's

Ye olde favourite `/roll` command from WoW chat, with some dnd flavour

### Specs
Supports following modes of usage:

* `/roll NdS`
  rolls a dice, where N is number of times to roll, S is number of sides.

* `/roll d6`
    rolls one d6, same as rolling 1d6

* `/roll d6, 2d4`
    rolls one d6, followed by two rolls of d4

* `/roll NUMBER`
  rolls a random number in the 1 to NUMBER range

* `/roll NUMBER-NUMBER`
    rolls a number in the specified range: (a, b) => roll(diff(b, a)) + a;
    should look for the first dash after trimming and skipping first symbol, split and parse int.

* `/roll` rolls a classic 1-100 roll.
