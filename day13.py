if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    input = list(filter(lambda x: x, file.read().split("\n")))

    parsed = list(
        map(
            lambda x: list(
                map(
                    lambda y: int(y.strip().split("+")[1])
                    if "+" in y
                    else int(y.strip().split("=")[1]),
                    x.split(":")[1].strip().split(","),
                )
            ),
            input,
        )
    )
    data = [parsed[i : i + 3] for i in range(0, len(parsed), 3)]

    res1 = 0
    for [[a, b], [c, d], [e, f]] in data:
        # ax  + cy = e
        # bx + dy = f
        # x = (e - cy) / a
        # be - bcy + ady = fa
        # y = (fa - be) / (ad - bc)
        y = (f * a - b * e) / (a * d - b * c)
        x = (e - c * y) / a

        if x.is_integer() and y.is_integer():
            res1 += x * 3 + y

    print(int(res1))

    res2 = 0
    for [[a, b], [c, d], [e, f]] in data:
        e += 10000000000000
        f += 10000000000000
        y = (f * a - b * e) / (a * d - b * c)
        x = (e - c * y) / a

        if x.is_integer() and y.is_integer():
            res2 += x * 3 + y

    print(int(res2))
