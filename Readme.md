# Roll the dice

Ye olde favourite `/roll` command from WoW chat.
### Specs
Supports following modes of usage:
- `/roll NdS` rolls a dice, where N is number of times to roll, S is number of sides.
- `/roll NUMBER NUMBER` rolls a random number in the provided range
- `/roll` rolls a classic 1-100 roll.

### Moving parts
- Arguments list processor
- Die roller
- Range roller
