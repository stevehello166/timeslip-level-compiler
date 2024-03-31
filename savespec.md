each entry is broken up by the / symbol in hex(2F)
fileformat: .timesliplvl

Magic Number
(8 bytes starts at version 01)
ASCII
TIMESLP01
Hex
54 49 4D 53 4C 50 30 31

two bytes of nothing:
00 00

Followed by the dimensions of the map in hex
0F 2F 0F
15/15 

two bytes of nothing:
00 00

room number:
00 01

two bytes of nothing:
00 00

followed by a section splitter
53 45 43 54

followed by a section number
01

byte of nothing:
00 

followed by a 4 character code 
ENTY

two bytes of nothing:
00 00

followed by a list of the spawn locations of different entities
ASCII
[(4!12/14),(4!5/14)]
Hex
5B 28 04 21 04 2F 01 29 2c 28 1A 21 0B 2F 08 29 5D

two bytes of nothing:
00 00

Followed section splitter
53 45 43 54

followed by a section number
02

 byte of nothing:
00 

followed by a 4 character code 
DECO


two bytes of nothing:
00 00

followed by a list of decorations
ASCII
[(4!12/14),(4!5/14)]
