def parse_string(ite, length):
    s = ""
    for _ in range(length):
        s += next(ite)

    return s

def parse_int(ite):
    s = ""
    while True:
        c = next(ite)
        if c == 'e':
            return int(s)
        s += c

def parse_dict(ite):
    r = {}
    while True:
        c = next(ite)
        if c == 'e':
            return r

        key = parse_element(ite, c)
        val = parse_element(ite)
        r[key] = val

def parse_list(ite):
    r = []
    while True:
        c = next(ite)
        if c == 'e':
            return r

        r.append(parse_element(ite, c))

def parse_element(ite, p=None):
    r = None
    c = p or next(ite)

    if c == 'd':
        r = parse_dict(ite)
    elif c == 'l':
        r = parse_list(ite)
    elif c == 'i':
        r = parse_int(ite)
    elif c.isnumeric():
        r = parse_string(ite, int(c))

    return r

def loads(s):
    return parse_element(iter(s))


if __name__ == '__main__':
    s = "d5helloi42e2hili1ei2eee" # {'hello': 42, 'hi': [1, 2]}
    r = loads(s)
    print(s, '->', r)
