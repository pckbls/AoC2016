#!/usr/bin/env python3.5
# *-* coding: utf-8 *-*

import sys

def count(triangles):
    valid = 0
    invalid = 0

    for a, b, c in triangles:
        if a+b>c and a+c>b and b+c>a:
            valid += 1
        else:
            invalid += 1

    print('Valid {}, Invalid {}'.format(valid, invalid))

def one(lines):
    count(lines)

def two(lines):
    triangles = []
    for i in range(0, len(lines), 3):
        triangles.append([lines[i][0], lines[i+1][0], lines[i+2][0]])
        triangles.append([lines[i][1], lines[i+1][1], lines[i+2][1]])
        triangles.append([lines[i][2], lines[i+1][2], lines[i+2][2]])

    count(triangles)

if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        lines = [[int(y) for y in x.split()] for x in f.read().splitlines()]

    print('Part one')
    one(lines)

    print('Part two')
    two(lines)

