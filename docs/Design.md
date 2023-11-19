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

## libp2p Rust implementation

[rust-libp2p] is the Rust implementation of the libp2p networking stack. After a connection with a
remote has been successfully established or a substream successfully opened, the next step is to
*upgrade* this connection or substream to use a protocol. This is where the `UpgradeInfo`, `InboundUpgrade`
and `OutboundUpgrade` traits come into play. The `InboundUpgrade` and `OutboundUpgrade` traits are
implemented on types that represent a collection of one or more possible protocols for respectively
an ingoing or outgoing connection or substream.

An upgrade is performed in two steps:

- A protocol negotiation step. The `UpgradeInfo::protocol_info` method is called to determine
which protocols are supported by the trait implementation. The `multistream-select` protocol
is used in order to agree on which protocol to use amongst the ones supported.

- A handshake. After a successful negotiation, the `InboundConnectionUpgrade::upgrade_inbound` or
`OutboundConnectionUpgrade::upgrade_outbound` method is called. This method will return a `Future` that
performs a handshake. This handshake is considered mandatory, however in practice it is
possible for the trait implementation to return a dummy `Future` that doesn't perform any
action and immediately succeeds.

After an upgrade is successful, an object of type `InboundConnectionUpgrade::Output` or
`OutboundConnectionUpgrade::Output` is returned. The actual object depends on the implementation and
there is no constraint on the traits that it should implement, however it is expected that it
can be used by the user to control the behaviour of the protocol.

### Security handshake issues

Overall, libp2p specifies two security protocols, TLS 1.3 and Noise. During a handshake, certificates
should be verified in order to allow the connection attempt to be aborted during the security handshake.
This requires being able to verify the peer ID after the handshake. Otherwise, we will have abnormal
behaviors, such as aborting after completing the handshake and leaving the peer not knowing what went
wrong.

In order to address this problem we introduced the `InboundSecurityUpgrade`/`OutboundSecurityUpgrade`
traits that should be implemented by transports such as Noise or TLS. Then, we modified the `Builder::authenticate`
function to accept an upgrade that implements these traits. Moreover, we implemented a state machine
in `upgrade::secure` that calls the `InboundSecurityUpgrade`/`OutboundSecurityUpgrade` traits instead
of `InboundConnectionUpgrade`/`OutboundConnectionUpgrade`. Finally, we complete the solution by providing
an implementation of the `InboundSecurityUpgrade`/`OutboundSecurityUpgrade` trait for TLS transport.


[TCP]: https://datatracker.ietf.org/doc/rfc9293
[TLS]: https://datatracker.ietf.org/doc/rfc8446
[HTTPS]: https://datatracker.ietf.org/doc/rfc9110
[libp2p]: https://docs.libp2p.io/
[TLS handshake specification]: https://github.com/libp2p/specs/blob/master/tls/tls.md
[design considerations]: https://github.com/libp2p/specs/blob/master/tls/design%20considerations.md
[rust-libp2p]: https://github.com/libp2p/rust-libp2p