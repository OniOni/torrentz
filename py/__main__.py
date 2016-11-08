import socket
import ssl

from . import client

s = socket.socket()
# secure_socket = ssl.wrap_socket(s, ssl_version=ssl.PROTOCOL_SSLv2)

# pi ('192.168.1.12', 51413)
s.connect(('localhost', 55496))

with open('ubuntu-16.10-desktop-amd64.iso.torrent', 'rb') as f:
    c = client.Client(f)

msg = c.mk_handshake_msg()
print(msg)
s.send(msg)
r = s.recv(4096)
c.handle_handshake(r)
