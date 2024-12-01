from sys import argv


def solution(input: list[str]) -> int:
    split = [tuple(map(int, i.split("   "))) for i in input]
    a, b = tuple(map(sorted, zip(*split)))
    return sum((b.count(x) * x for x in a))


def main(filename: str):
    with open(filename, "r") as f:
        lines = f.readlines()
    print(solution(lines))


if __name__ == "__main__":
    main(argv[1])
