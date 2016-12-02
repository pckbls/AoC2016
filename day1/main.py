#!/usr/bin/env python3.5
# *-* coding: utf-8 *-*

import sys

if __name__ == '__main__':
    sequences = []
    for line in sys.stdin.readline().split(','):
        line = line.strip()
        sequences.append((line[0], int(line[1:])))

    history=[]
    position=[0, 0]
    orientation=0 # 0: north, 1: east, 2: south, 3: west

    for turn, distance in sequences:
        if turn == 'L':
            orientation = (orientation - 1) % 4
        elif turn == 'R':
            orientation = (orientation + 1) % 4

        for step in [1]*distance:
            if orientation == 0:
                position[1] += step
            elif orientation == 1:
                position[0] += step
            elif orientation == 2:
                position[1] -= step
            elif orientation == 3:
                position[0] -= step

            if position in history:
                break
            else:
                history.append(position[:])

        else:
            continue

        break

    print(position)

    print(abs(position[0]) + abs(position[1]))

