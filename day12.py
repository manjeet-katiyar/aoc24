def is_inbound(n, m, x, y):
    return x >= 0 and x < n and y >= 0 and y < m


nex = [-1, -1, -1, 0, 1, 1, 1, 0]
ney = [-1, 0, 1, 1, 1, 0, -1, -1]

nes = [-1, 0, 1, 0, -1]


def dfs(graph, visited, n, m, x, y):
    visited[x][y] = True

    perimeter = 0
    area = 1

    for i in range(4):
        nx = x + nes[i]
        ny = y + nes[i + 1]
        if is_inbound(n, m, nx, ny) and graph[nx][ny] != graph[x][y]:
            perimeter += 1

        if not is_inbound(n, m, nx, ny):
            perimeter += 1

    for i in range(4):
        nx = x + nes[i]
        ny = y + nes[i + 1]

        if (
            is_inbound(n, m, nx, ny)
            and graph[nx][ny] == graph[x][y]
            and not visited[nx][ny]
        ):
            (p1, a1) = dfs(graph, visited, n, m, nx, ny)
            perimeter += p1
            area += a1

    return (perimeter, area)


def get_val(graph, n, m, x, y):
    if not is_inbound(n, m, x, y):
        return None

    return graph[x][y]


def count_edges(graph, n, m, x, y):
    a = (
        get_val(graph, n, m, x, y - 1),
        get_val(graph, n, m, x - 1, y - 1),
        get_val(graph, n, m, x - 1, y),
    )
    b = (
        get_val(graph, n, m, x, y - 1),
        get_val(graph, n, m, x + 1, y - 1),
        get_val(graph, n, m, x + 1, y),
    )
    c = (
        get_val(graph, n, m, x + 1, y),
        get_val(graph, n, m, x + 1, y + 1),
        get_val(graph, n, m, x, y + 1),
    )
    d = (
        get_val(graph, n, m, x - 1, y),
        get_val(graph, n, m, x - 1, y + 1),
        get_val(graph, n, m, x, y + 1),
    )

    curr = graph[x][y]
    # print(a, b, c, d, x, y)

    def is_valid(a, b, c):
        if b is None or b != curr:
            return (a != curr and c != curr) or (a == curr and c == curr)

        return a != curr and c != curr

    return len(list(filter(lambda x: is_valid(x[0], x[1], x[2]), [a, b, c, d])))


def dfs1(graph, visited, n, m, x, y):
    visited[x][y] = True

    perimeter = count_edges(graph, n, m, x, y)
    # print(x,  perimeter)
    area = 1

    for i in range(4):
        nx = x + nes[i]
        ny = y + nes[i + 1]

        if (
            is_inbound(n, m, nx, ny)
            and graph[nx][ny] == graph[x][y]
            and not visited[nx][ny]
        ):
            (p1, a1) = dfs1(graph, visited, n, m, nx, ny)
            perimeter += p1
            area += a1

    return (perimeter, area)


if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    f = open(filename, "r")
    input = list(filter(lambda x: x, f.read().split("\n")))

    # print(input)

    n = len(input)
    m = len(input[0])

    visited = [[False for _ in range(m)] for _ in range(n)]

    res = []
    for i in range(n):
        for j in range(m):
            if not visited[i][j]:
                res.append(dfs(input, visited, n, m, i, j))

    res1 = sum([i * j for (i, j) in res])
    print(res1)

    visited = [[False for _ in range(m)] for _ in range(n)]
    res = []
    for i in range(n):
        for j in range(m):
            if not visited[i][j]:
                res.append(dfs1(input, visited, n, m, i, j))

    # print(res)
    res2 = sum([i * j for (i, j) in res])
    print(res2)
