#!/usr/bin/env python3.5
# *-* coding: utf-8 *-*

import sys
import numpy

def one(columns, reverse=True):
    result = ''

    for column in columns:
        counts = {}
        for c in set(column):
            counts[c] = column.count(c)
        char, count = sorted(counts.items(), key=lambda x: x[1], reverse=reverse)[0]
        result += char

    return result

def two(columns):
    return one(columns, reverse=False)

if __name__ == '__main__':
    # load file contents
    with open(sys.argv[1]) as f:
        lines = f.read().splitlines()

    # transform into an easier to process data structure
    columns = [[] for i in range(len(lines[0]))]
    for line in lines:
        for i, c in enumerate(line):
            columns[i].append(c)

    print('Solution for part one:', one(columns))
    print('Solution for part two:', two(columns))
