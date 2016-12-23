#!/usr/bin/env python3.5
# *-* coding: utf-8 *-*

import sys
import re

class Display:

    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.matrix = [[False for x in range(width)] for y in range(height)]

    def print(self):
        for y in range(0, self.height):
            for x in range(0, self.width):
                print('#' if self.matrix[y][x] else '.', end='')
            print('')

    def count_pixels(self):
        result = 0
        for y in range(0, self.height):
            for x in range(0, self.width):
                if self.matrix[y][x]:
                    result += 1
        return result

    def change_pixel(self, x, y, enabled=True):
        if x < 0 or x > self.width or y < 0 or y > self.height:
            return
        self.matrix[y][x] = enabled

    def rect(self, A, B):
        for a in range(0, A):
            for b in range(0, B):
                self.change_pixel(x=a, y=b, enabled=True)

    def rotate_column(self, x, by):
        old_column = [self.matrix[y][x] for y in range(0, self.height)]
        for y in range(0, self.height):
            self.change_pixel(x, y, old_column[y-by])

    def rotate_row(self, y, by):
        old_row = self.matrix[y][:]
        for x in range(0, self.width):
            self.change_pixel(x, y, old_row[x-by])

if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        instructions = f.readlines()

    display = Display(width=50, height=6)
    for instruction in instructions:
        m = re.match('rect ([0-9]+)x([0-9]+)', instruction)
        if m:
            display.rect(A=int(m.group(1)), B=int(m.group(2)))
            continue

        m = re.match('rotate column x=([0-9]+) by ([0-9]+)', instruction)
        if m:
            display.rotate_column(x=int(m.group(1)), by=int(m.group(2)))
            continue

        m = re.match('rotate row y=([0-9]+) by ([0-9]+)', instruction)
        if m:
            display.rotate_row(y=int(m.group(1)), by=int(m.group(2)))
            continue

    display.print()
    print(display.count_pixels())
