def parse_file(filename: str):
    with open(filename, "r") as f:
        contents = f.readall()
    return parse_input(contents)


def parse_input(text: str):
    pass
