from collections import OrderedDict

def parse_string(ite, c):
    s = b""
    while c != b':':
        s += c
        c = next(ite)

    length = int(s)
    s = b""
    for _ in range(length):
        s += next(ite)

    return s

def parse_int(ite):
    s = b""
    while True:
        c = next(ite)
        if c == b'e':
            return int(s)
        s += c

def parse_dict(ite):
    r = OrderedDict()
    while True:
        c = next(ite)
        if c == b'e':
            return r

        key = parse_element(ite, c)
        val = parse_element(ite)
        r[key] = val

def parse_list(ite):
    r = []
    while True:
        c = next(ite)
        if c == b'e':
            return r

        r.append(parse_element(ite, c))

def parse_element(ite, p=None):
    r = None
    c = p or next(ite)

    if c == b'd':
        r = parse_dict(ite)
    elif c == b'l':
        r = parse_list(ite)
    elif c == b'i':
        r = parse_int(ite)
    elif c.isdigit():
        r = parse_string(ite, c)
    else:
        print(c)

    return r

def bytes_iter(s):
    return map(lambda x: bytes([x]), s)

def loads(s):
    return parse_element(bytes_iter(s))

def load(f):
    return parse_element(bytes_iter(f.read()))

def dumps(bencode):
    if isinstance(bencode, dict):
        s = b'd'
        v = b"".join([dumps(x) for item in bencode.items()
                          for x in item])
    elif isinstance(bencode, list):
        s =  b'l'
        v =  b"".join(map(dumps, bencode))
    elif isinstance(bencode, int):
        s = b'i'
        v = bytes(str(bencode), 'utf8')
    else:
        return bytes(str(len(bencode)), 'utf8') + b':' + bytes(bencode)

    return s + v + b'e'

if __name__ == '__main__':
    print("d5:helloi42e2:hili1ei2eee")
    s = b"d5:helloi42e2:hili1ei2eee"
    r = loads(s)
    print('->', r)

    with open('ubuntu-16.04-server-amd64.iso.torrent', 'rb') as f:
        meta = load(f)

    print("{'hello': 42, 'hi': [1, 2]}")
    b = dumps(r)
    print('->', b)

    print(dumps(meta[b'info']))
