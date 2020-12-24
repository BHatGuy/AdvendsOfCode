import numpy as np


def read_input():
    tiles = []
    with open('input', 'r') as f:
        for tile in f.read().split('\n\n'):
            rows = tile.split('\n')
            tid = int(rows[0][5:9])
            rs = []
            for row in rows[1:]:
                if len(row) == 0:
                    continue
                rs.append(list(row))
            tiles.append((tid, np.array(rs)))
    return tiles


def rotate(m):

    return np.apply_along_axis(lambda x: np.flip(x), axis=1, arr=m.transpose())


def solve1():
    tiles = read_input()
    rotate(tiles[0][1])


def solve2():
    pass


if __name__ == "__main__":
    print(solve1())
    print(solve2())
