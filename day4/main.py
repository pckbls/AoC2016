#!/usr/bin/env python3.5
# *-* coding: utf-8 *-*

import string
import collections
import sys
import re

def one(rooms):
    sector_id_sum = 0

    for room in rooms:
        m = re.match('([a-z0-9\-]*)\[([a-z]*)\]', room)
        tmp = m.group(1).split('-')
        letters = ''.join(tmp[0:-1])
        sector_id = int(tmp[-1])
        checksum = m.group(2)

        letter_count = {}
        for c in string.ascii_lowercase:
            letter_count[c] = letters.count(c)

        # magic!
        sorted_letter_count=sorted(letter_count.items(),
                key=lambda x: (x[1] * -1, x[0]))

        calculed_checksum=''.join([x[0] for x in sorted_letter_count][0:5])

        if checksum == calculed_checksum:
            sector_id_sum += sector_id

    print(sector_id_sum)

def two(rooms):
    pass

if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        rooms = f.read().splitlines()

    print('Part one')
    one(rooms)

    print('Part two')
    two(rooms)

