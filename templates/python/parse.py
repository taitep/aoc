def parse_input(text: str):
    pass


def parse_file(filename: str):
    with open(filename, "r") as f:
        contents = f.readall()
    return parse_input(contents)


def main():
    from pprint import pprint
    from sys import argv

    pprint(parse_file(argv[1]))
