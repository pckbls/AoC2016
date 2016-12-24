#!/usr/bin/env python3.5
# *-* coding: utf-8 *-*

import sys
import re

def one(data, recursive=False):
    result = 0

    i = 0
    while i < len(data):
        m = re.search('^\(([0-9]+)x([0-9]+)\)', data[i:])
        if m:
            i += len(m.group(0))
            if recursive:
                result += one(data[i:i+int(m.group(1))], recursive=True) * int(m.group(2))
            else:
                result += int(m.group(1)) * int(m.group(2))
            i += int(m.group(1))
            continue

        if data[i] != '\n':
            result += 1

        i += 1

    return result

if __name__ == '__main__':
    with open(sys.argv[1]) as f:
        contents = f.read()

    print("Part one", one(contents, recursive=False))
    print("Part two", one(contents, recursive=True))
