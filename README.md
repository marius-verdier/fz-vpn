## Pure rust VPN

### Project structure
1. Tunnels

UDP Tunnels with [QUIC protocol](https://en.wikipedia.org/wiki/QUIC) : QUIC proto compels to secure the connection, and aims to be better than TCP.

2. CLI

`fpvpn -saddr SOURCE_ADDRESS -daddr DESTINATION_ADDRESS -cert CERT_PATH -key KEY_PATH`
