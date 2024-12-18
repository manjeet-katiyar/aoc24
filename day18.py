import heapq


def printx(mat):
    print("\n".join("".join(i) for i in mat))


def is_valid(n, m, i, j):
    return i >= 0 and j >= 0 and i < n and j < m


DIRS = [-1, 0, 1, 0, -1]


def bfs(graph, si, sj, ei, ej):
    q = [(0, si, sj)]
    visited = set()

    while q:
        (H, I, J) = heapq.heappop(q)

        if I == ei and J == ej:
            return H

        if (I, J) not in visited:
            visited.add((I, J))
            for k in range(4):
                ni = I + DIRS[k]
                nj = J + DIRS[k + 1]

                if (
                    is_valid(n, n, ni, nj)
                    and graph[ni][nj] != "#"
                    and (ni, nj) not in visited
                ):
                    heapq.heappush(q, (H + 1, ni, nj))
    return -1


if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    input = list(filter(lambda x: x, file.read().split("\n")))

    n = 71

    m = 1024
    graph = [["." for _ in range(n)] for _ in range(n)]
    for item in input[:m]:
        [i, j] = map(lambda x: int(x), item.split(","))
        graph[j][i] = "#"

    print("Res1: ", bfs(graph, 0, 0, n - 1, n - 1))

    lo, hi = 0, len(input) - 1
    while lo < hi:
        m = (lo + hi) // 2

        graph = [["." for _ in range(n)] for _ in range(n)]
        for item in input[:m]:
            [i, j] = map(lambda x: int(x), item.split(","))
            graph[j][i] = "#"

        h = bfs(graph, 0, 0, n - 1, n - 1)
        if h == -1:
            hi = m
        else:
            lo = m + 1

    print(input[lo - 1])
