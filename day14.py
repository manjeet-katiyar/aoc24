if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    input = list(filter(lambda x: x, file.read().split("\n")))

    parsed = list(
        map(
            lambda x: list(
                map(
                    lambda z: list(map(lambda a: int(a), z.split(","))),
                    map(lambda y: y.split("=")[1], x.split(" ")),
                )
            ),
            input,
        )
    )

    n = 103
    m = 101

    def calculate(s):
        a, b, c, d = 0, 0, 0, 0
        e, f = 0, 0
        mx = m // 2
        my = n // 2
        for [[x, y], [dx, dy]] in parsed:
            nx = (x + s * dx) % m
            ny = (y + s * dy) % n

            if nx < mx and ny < my:
                a += 1
            elif nx < mx and ny > my:
                b += 1
            elif nx > mx and ny < my:
                c += 1
            elif nx > mx and ny > my:
                d += 1

            if nx == mx:
                e += 1
            if ny == my:
                f += 1

        return (a, b, c, d, e, f)

    def printx(s):
        mat = [[0 for _ in range(m)] for _ in range(n)]
        for [[x, y], [dx, dy]] in parsed:
            nx = (x + s * dx) % m
            ny = (y + s * dy) % n
            mat[ny][nx] = 1

        mtt = "\n".join(["".join(map(lambda j: "*" if j else " ", i)) for i in mat])
        print(mtt)

    (a, c, b, d, _, _) = calculate(100)
    print(a * b * c * d)

    temp = []
    for i in range(0, 10000):
        (_, _, _, _, e, f) = calculate(i)
        temp.append((e, f, i))

    for _, _, i in sorted(temp, reverse=True)[:5]:
        printx(i)
        print(i)
        print("-----" * 20)
