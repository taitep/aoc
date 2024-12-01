from sys import argv


def solution(input: list[str]) -> int:
    pass


def main(filename: str):
    with open(filename, "r") as f:
        lines = f.readlines()
    print(solution(lines))


if __name__ == "__main__":
    main(argv[1])
