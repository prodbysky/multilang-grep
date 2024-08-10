#!/usr/bin/env python

import sys


def main():
    args: list[str | None] = sys.argv

    file_name: str = args[1]
    needle: str = args[2]

    for line in open(file_name, 'r'):
        if needle in line:
            print(line, end='')


if __name__ == "__main__":
    main()
