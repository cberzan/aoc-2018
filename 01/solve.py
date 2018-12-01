#!/usr/bin/env python

import sys


if __name__ == "__main__":
    result = 0
    for line in sys.stdin:
        result += int(line)
    print result
