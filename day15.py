def normalize_path(path):
    new_path = []
    prev = None
    ct = 0

    for c in path + "*":
        if prev is None:
            prev = c
            ct = 1
        elif c == prev:
            ct += 1
        else:
            new_path.append((ct, prev))
            prev = c
            ct = 1

    return new_path


def get_dir(dir):
    if dir == ">":
        return (0, 1)

    if dir == "<":
        return (0, -1)

    if dir == "^":
        return (-1, 0)

    return (1, 0)


def is_valid(n, m, i, j):
    return i >= 0 and j >= 0 and i < n and j < m


def move_rob(mat, n, m, ri, rj, ct, dir):
    (di, dj) = get_dir(dir)
    hi, hj = ri, rj
    while ct > 0 and is_valid(n, m, hi + di, hj + dj) and mat[hi + di][hj + dj] != "#":
        ni = hi + di
        nj = hj + dj

        prev = "."
        if mat[ni][nj] == "O":
            while is_valid(n, m, ni, nj) and mat[ni][nj] == "O":
                prev = mat[ni][nj]
                ni += di
                nj += dj

        if not is_valid(n, m, ni, nj) or mat[ni][nj] == "#":
            break

        if mat[ni][nj] == ".":
            mat[ni][nj] = prev
            mat[hi][hj] = "."
            ri = hi + di
            rj = hj + dj
            mat[ri][rj] = "@"
            hi, hj = ri, rj
        else:
            break

        ct -= 1

    return (ri, rj)


def move_rob1(mat, n, m, ri, rj, ct, dir):
    (di, dj) = get_dir(dir)
    hi, hj = ri, rj
    if dir in ["<", ">"]:
        while (
            ct > 0 and is_valid(n, m, hi + di, hj + dj) and mat[hi + di][hj + dj] != "#"
        ):
            ni = hi + di
            nj = hj + dj

            if mat[ni][nj] in ["[", "]"]:
                while is_valid(n, m, ni, nj) and mat[ni][nj] in ["[", "]"]:
                    ni += di
                    nj += dj

            if not is_valid(n, m, ni, nj) or mat[ni][nj] == "#":
                break

            ni = hi + di
            nj = hj + dj
            prev = "."
            if mat[ni][nj] in ["[", "]"]:
                while is_valid(n, m, ni, nj) and mat[ni][nj] in ["[", "]"]:
                    temp = mat[ni][nj]
                    mat[ni][nj] = prev
                    prev = temp
                    ni += di
                    nj += dj

            if mat[ni][nj] == ".":
                mat[ni][nj] = prev
                mat[hi][hj] = "."
                ri = hi + di
                rj = hj + dj
                mat[ri][rj] = "@"
                hi, hj = ri, rj
            else:
                break

            ct -= 1

        return (ri, rj)
    else:

        def coor(i, j):
            if mat[i][j] == "[":
                return (i, j, i, j + 1)
            return (i, j - 1, i, j)

        def move(x, y):
            if not is_valid(n, m, x, y):
                return False

            if mat[x][y] == "#":
                return False

            if mat[x][y] == ".":
                return True

            (a, b, c, d) = coor(x, y)
            ba, bb, bc, bd = a, b, c, d

            a += di
            b += dj
            c += di
            d += dj

            if not move(a, b):
                return False

            if not move(c, d):
                return False

            mat[ba][bb] = "."
            mat[bc][bd] = "."
            mat[a][b] = "["
            mat[c][d] = "]"
            return True

        def can_move(x, y):
            if not is_valid(n, m, x, y):
                return False

            if mat[x][y] == "#":
                return False

            if mat[x][y] == ".":
                return True

            (a, b, c, d) = coor(x, y)

            a += di
            b += dj
            c += di
            d += dj

            if not can_move(a, b):
                return False

            if not can_move(c, d):
                return False

            return True

        while ct > 0:
            if can_move(ri + di, rj + dj) and move(ri + di, rj + dj):
                mat[ri][rj] = "."
                ri += di
                rj += dj
                mat[ri][rj] = "@"
                ct -= 1
                # printx(mat)
            else:
                break

        return (ri, rj)


def printx(mat):
    print("\n".join("".join(i) for i in mat))


def part1(input):
    piv = 0
    for ind, item in enumerate(input):
        if all(c == "#" for c in item):
            piv = ind

    mat = [[j for j in i] for i in input[: piv + 1]]
    n = len(mat)
    m = len(mat[0])
    path = normalize_path("".join(input[piv + 1 :]))

    ri, rj = 0, 0
    for i in range(n):
        for j in range(m):
            if mat[i][j] == "@":
                ri, rj = i, j
                break

        if ri != 0 and rj != 0:
            break

    for ct, dir in path:
        (ri, rj) = move_rob(mat, n, m, ri, rj, ct, dir)

    res1 = 0
    for i in range(n):
        for j in range(m):
            if mat[i][j] == "O":
                res1 += 100 * i + j

    return res1


def part2(input):
    piv = 0
    for ind, item in enumerate(input):
        if all(c == "#" for c in item):
            piv = ind

    for ind in range(len(input[: piv + 1])):
        input[ind] = input[ind].replace("#", "##")
        input[ind] = input[ind].replace(".", "..")
        input[ind] = input[ind].replace("@", "@.")
        input[ind] = input[ind].replace("O", "[]")

    mat = [[j for j in i] for i in input[: piv + 1]]

    n = len(mat)
    m = len(mat[0])
    path = normalize_path("".join(input[piv + 1 :]))

    ri, rj = 0, 0
    for i in range(n):
        for j in range(m):
            if mat[i][j] == "@":
                ri, rj = i, j
                break

        if ri != 0 and rj != 0:
            break

    for ct, dir in path:
        (ri, rj) = move_rob1(mat, n, m, ri, rj, ct, dir)
        # print(ct, dir)
        # printx(mat)

    res = 0
    for i in range(n):
        for j in range(m):
            if mat[i][j] == "[":
                res += 100 * i + j

    return res


if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    input = list(filter(lambda x: x, file.read().split("\n")))

    print(part1(input))
    print(part2(input))
