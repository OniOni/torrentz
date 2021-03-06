import hashlib

from . import bencode

PROTO_STR = b"BitTorrent protocol"
PROTO_STR_LEN = bytes("{}".format(chr(len(PROTO_STR))), 'utf8')


class Client(object):

    def __init__(self, metafile):
        self._meta = metafile
        self._decoded = bencode.load(metafile)

    def mk_handshake_msg(self):
        s = hashlib.sha1()
        s.update(bencode.dumps(self._decoded[b'info']))

        digest = s.digest()
        return (PROTO_STR_LEN + PROTO_STR +
                bytes(chr(0) * 8, 'utf') +
                digest +
                b"-MS0000-000000000000")

    def handle_handshake(self, msg):
        print(msg)

    def mk_announce_msg(self):
        pass


if __name__ == '__main__':
    with open('ubuntu-16.04-server-amd64.iso.torrent', 'rb') as f:
        c = Client(f)

    print(c.mk_handshake_msg())
