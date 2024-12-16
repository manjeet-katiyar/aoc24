import heapq


def printx(mat):
    print("\n".join("".join(i) for i in mat))


def is_valid(n, m, i, j):
    return i >= 0 and j >= 0 and i < n and j < m


DIRS = [-1, 0, 1, 0, -1]
SCORES = {
    1: {
        (-1, 0): (1001, 0),
        (0, 1): (1, 1),
        (1, 0): (1001, 3),
        (0, -1): (3001, 2),
    },
    2: {
        (-1, 0): (1001, 0),
        (0, 1): (3001, 1),
        (1, 0): (1001, 3),
        (0, -1): (1, 2),
    },
    0: {
        (-1, 0): (1, 0),
        (0, 1): (1001, 1),
        (1, 0): (3001, 3),
        (0, -1): (1001, 2),
    },
    3: {
        (-1, 0): (3001, 0),
        (0, 1): (1001, 1),
        (1, 0): (1, 3),
        (0, -1): (1001, 2),
    },
}

if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    input = list(filter(lambda x: x, file.read().split("\n")))

    graph = [[j for j in i] for i in input]
    n = len(graph)
    m = len(graph[0])

    si, sj, ei, ej = 0, 0, 0, 0
    for i in range(n):
        for j in range(m):
            if graph[i][j] == "S":
                si, sj = i, j

            if graph[i][j] == "E":
                ei, ej = i, j

    visited = set()
    parents = {}
    li = [(0, si, sj, -1, -1, 1)]
    heapq.heapify(li)

    while len(li):
        (S, I, J, pi, pj, dir) = heapq.heappop(li)
        if not is_valid(n, m, I, J):
            continue

        visited.add((I, J))
        for i in range(4):
            di = DIRS[i]
            dj = DIRS[i + 1]

            ni = I + di
            nj = J + dj

            (score, new_dir) = SCORES[dir][(di, dj)]
            if is_valid(n, m, ni, nj) and graph[ni][nj] != "#":
                if (ni, nj) not in visited:
                    heapq.heappush(li, (S + score, ni, nj, I, J, new_dir))

                if (ni, nj) not in parents:
                    parents[(ni, nj)] = set()
                parents[(ni, nj)].add((S + score, I, J, new_dir))

    res1 = min(map(lambda x: x[0], parents[(ei, ej)]))
    print("Res1: ", res1)

    visited = set()

    def mark(i, j, s, dir):
        if graph[i][j] == "#":
            return

        if i == si and j == sj:
            return

        visited.add((i, j, dir))

        # print(i, j, s, dir)
        # print(parents[(i, j)])
        for ps, pi, pj, pdir in parents[(i, j)]:
            score_to_dir = (
                list(filter(lambda x: x[1] == dir, SCORES[pdir].values()))[0][0] - 1
            )

            if ps == s - score_to_dir:
                graph[pi][pj] = "O"
                mark(pi, pj, ps - 1, pdir)

    for s, pi, pj, d in parents[(ei, ej)]:
        if s == res1:
            graph[ei][ej] = "O"
            mark(ei, ej, res1, d)

    # printx(graph)
    res2 = 0
    for i in range(n):
        for j in range(m):
            if graph[i][j] == "O":
                res2 += 1

    print("Res2: ", res2)
