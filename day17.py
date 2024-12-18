import heapq


def printx(mat):
    print("\n".join("".join(i) for i in mat))


def is_valid(n, m, i, j):
    return i >= 0 and j >= 0 and i < n and j < m


DIRS = [-1, 0, 1, 0, -1]


def get_combo(A, B, C, op):
    if op < 4:
        return op

    if op == 4:
        return A

    if op == 5:
        return B

    if op == 6:
        return C

    raise Exception("Invalid")


def part1(input, A, B, C):
    IP = 0
    STEPS = 2

    PROGRAM = list(map(lambda x: int(x), input[3].split(": ")[1].split(",")))

    res = []
    while IP < len(PROGRAM) - 1:
        opcode = PROGRAM[IP]
        operand = PROGRAM[IP + 1]

        match opcode:
            case 0:
                A = A // (2 ** (get_combo(A, B, C, operand)))
            case 1:
                B = B ^ operand
            case 2:
                B = get_combo(A, B, C, operand) % 8
            case 3:
                if A != 0:
                    IP = operand
                    continue
            case 4:
                B = B ^ C
            case 5:
                res.append(get_combo(A, B, C, operand) % 8)
            case 6:
                B = A // (2 ** (get_combo(A, B, C, operand)))
            case 7:
                C = A // (2 ** (get_combo(A, B, C, operand)))

        IP += STEPS

    return ",".join(map(lambda x: str(x), res))


def part2(input, A, B, C):
    IP = 0
    STEPS = 2

    PROGRAM = list(map(lambda x: int(x), input[3].split(": ")[1].split(",")))

    res = []
    while IP < len(PROGRAM) - 1:
        opcode = PROGRAM[IP]
        operand = PROGRAM[IP + 1]

        match opcode:
            case 0:
                A = A // (2 ** (get_combo(A, B, C, operand)))
            case 1:
                B = B ^ operand
            case 2:
                B = get_combo(A, B, C, operand) % 8
            case 3:
                if A != 0:
                    IP = operand
                    continue
            case 4:
                B = B ^ C
            case 5:
                res.append(get_combo(A, B, C, operand) % 8)
            case 6:
                B = A // (2 ** (get_combo(A, B, C, operand)))
            case 7:
                C = A // (2 ** (get_combo(A, B, C, operand)))

        IP += STEPS

    return ",".join(map(lambda x: str(x), res))


if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    input = list(filter(lambda x: x, file.read().split("\n")))

    A, B, C = (
        int(input[0].split(": ")[1]),
        int(input[1].split(": ")[1]),
        int(input[2].split(": ")[1]),
    )
    print("Res1: ", part1(input, A, B, C))
    print("Res1: ", part1(input, 0, B, C))
