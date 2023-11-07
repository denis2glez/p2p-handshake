## TCP and TLS
Transmission Control Protocol ([TCP]) is one of the foundations of the Internet protocol suite and was
developed in the 1970s. While Transport Layer Security ([TLS]) is a cryptographic protocol widely used
in applications such as email, messaging instant and voice over IP, its use to secure [HTTPS] remains
the most publicly visible.

## libp2p 

[libp2p] is a peer-to-peer (P2P) networking framework that enables the development of P2P applications.
It consists of a collection of protocols, specifications, and libraries that facilitate P2P communication
between network participants, known as "peers".

## TCP and TLS in libp2p

Establishing a libp2p connection on top of TCP takes a few steps, upgrading the underlying connection:

1. Dial a TCP connection to the remote node and perform the 3-way-handshake.
2. Negotiate the TLS 1.3 security protocol (or Noise), and then perform the chosen cryptographic
   handshake. The connection is now encrypted and the peers have verified each others' peer IDs.
3. Apply a stream multiplexer (yamux or mplex).

## libp2p TLS handshake

The libp2p handshake uses TLS 1.3 (and higher). Endpoints should not negotiate lower TLS versions.
During the handshake, peers authenticate each other's identity as described in the [TLS handshake specification].
This means that servers should require client authentication during the TLS handshake, and should abort
a connection attempt if the client fails to provide the requested authentication information. When
negotiating the use of this handshake dynamically, it must be identified with the protocol ID `/tls/1.0.0`.

Based on the [design considerations] around the TLS protocol, there are two main requirements that prevent
the common way of running a TLS handshake by simply using the host key to create a self-signed certificate.
1. The use of different key types: RSA, ECDSA, Ed25519, Secp256k1, etc.
2. The need to be able to send the key type along with the key.


[TCP]: https://datatracker.ietf.org/doc/rfc9293
[TLS]: https://datatracker.ietf.org/doc/rfc8446
[HTTPS]: https://datatracker.ietf.org/doc/rfc9110
[libp2p]: https://docs.libp2p.io/
[TLS handshake specification]: https://github.com/libp2p/specs/blob/master/tls/tls.md
[design considerations]: https://github.com/libp2p/specs/blob/master/tls/design%20considerations.md