from functools import reduce
from collections import defaultdict, Counter
from pprint import pprint
import functools
import operator
from tabulate import tabulate


def get_input(name="input.txt"):
    lines = []
    first = ""
    with open(name, "r") as f:
        first = f.readline().strip()
        f.readline()
        lines = f.readlines()
    lines = [l.strip() for l in lines]

    numbers = [int(f) for f in first.split(",")]
    boards = []
    curr_board = []
    for l in lines:
        if l == "":
            assert len(curr_board) == 5
            boards.append(curr_board)
            curr_board = []
        else:
            row = list(
                filter(lambda x: x is not None,
                       [int(x) if x != "" else None for x in l.split(" ")]))
            assert len(row) == 5
            curr_board.append(row)

    return numbers, boards


def mark(number, board):
    return list(
        map(lambda row: [None if x == number else x for x in row], board))


def check_baord(board):
    # check rows
    if any(map(lambda row: all(map(lambda x: x is None, row)), board)):
        return True

    for i in range(len(board)):
        acc = True
        for j in range(len(board)):
            acc &= board[j][i] is None
        if acc:
            return True

    return False


def print_board(board):
    print(tabulate(board))

def solve1(inp):
    numbers, boards = inp
    for num in numbers:
        marked_boards = []
        for board in boards:
            marked = mark(num, board)
            if check_baord(marked):
                s = sum(filter(lambda x: x is not None, functools.reduce(operator.iconcat, marked, [])))
                return s * num
            marked_boards.append(marked)
        boards = marked_boards


def solve2(inp):
    numbers, boards = inp
    for num in numbers:
        marked_boards = []
        for board in boards:
            marked = mark(num, board)
            if check_baord(marked):
                if len(boards) == 1:
                    s = sum(filter(lambda x: x is not None, functools.reduce(operator.iconcat, marked, [])))
                    return s * num
            else:
                marked_boards.append(marked)
        boards = marked_boards


if __name__ == "__main__":
    print(solve1(get_input()))
    print(solve2(get_input()))