use futures::{prelude::*, stream::FusedStream};
use libp2p_core::multiaddr::Multiaddr;
use libp2p_core::transport::{ListenerId, Transport, TransportError, TransportEvent};
use std::{
    error::Error,
    fmt, io,
    pin::Pin,
    task::{Context, Poll},
};

/// Creates a new [`Boxed`] transport from the given transport.
pub(crate) fn boxed<T>(transport: T) -> Boxed<T::Output>
where
    T: Transport + Send + Unpin + 'static,
    T::Error: Send + Sync,
    T::Dial: Send + 'static,
    T::ListenerUpgrade: Send + 'static,
{
    Boxed {
        inner: Box::new(transport) as Box<_>,
    }
}

/// A `Boxed` transport is a `Transport` whose `Dial`, `Listener`
/// and `ListenerUpgrade` futures are `Box`ed and only the `Output`
/// type is captured in a type variable.
pub struct Boxed<O> {
    inner: Box<dyn Abstract<O> + Send + Unpin>,
}

type Dial<O> = Pin<Box<dyn Future<Output = io::Result<O>> + Send>>;
type ListenerUpgrade<O> = Pin<Box<dyn Future<Output = io::Result<O>> + Send>>;

trait Abstract<O> {
    fn listen_on(
        &mut self,
        id: ListenerId,
        addr: Multiaddr,
    ) -> Result<(), TransportError<io::Error>>;
    fn remove_listener(&mut self, id: ListenerId) -> bool;
    fn dial(&mut self, addr: Multiaddr) -> Result<Dial<O>, TransportError<io::Error>>;
    fn dial_as_listener(&mut self, addr: Multiaddr) -> Result<Dial<O>, TransportError<io::Error>>;
    fn address_translation(&self, server: &Multiaddr, observed: &Multiaddr) -> Option<Multiaddr>;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<TransportEvent<ListenerUpgrade<O>, io::Error>>;
}

impl<T, O> Abstract<O> for T
where
    T: Transport<Output = O> + 'static,
    T::Error: Send + Sync,
    T::Dial: Send + 'static,
    T::ListenerUpgrade: Send + 'static,
{
    fn listen_on(
        &mut self,
        id: ListenerId,
        addr: Multiaddr,
    ) -> Result<(), TransportError<io::Error>> {
        Transport::listen_on(self, id, addr).map_err(|e| e.map(box_err))
    }

    fn remove_listener(&mut self, id: ListenerId) -> bool {
        Transport::remove_listener(self, id)
    }

    fn dial(&mut self, addr: Multiaddr) -> Result<Dial<O>, TransportError<io::Error>> {
        let fut = Transport::dial(self, addr)
            .map(|r| r.map_err(box_err))
            .map_err(|e| e.map(box_err))?;
        Ok(Box::pin(fut) as Dial<_>)
    }

    fn dial_as_listener(&mut self, addr: Multiaddr) -> Result<Dial<O>, TransportError<io::Error>> {
        let fut = Transport::dial_as_listener(self, addr)
            .map(|r| r.map_err(box_err))
            .map_err(|e| e.map(box_err))?;
        Ok(Box::pin(fut) as Dial<_>)
    }

    fn address_translation(&self, server: &Multiaddr, observed: &Multiaddr) -> Option<Multiaddr> {
        Transport::address_translation(self, server, observed)
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<TransportEvent<ListenerUpgrade<O>, io::Error>> {
        self.poll(cx).map(|event| {
            event
                .map_upgrade(|upgrade| {
                    let up = upgrade.map_err(box_err);
                    Box::pin(up) as ListenerUpgrade<O>
                })
                .map_err(box_err)
        })
    }
}

impl<O> fmt::Debug for Boxed<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoxedTransport")
    }
}

impl<O> Transport for Boxed<O> {
    type Output = O;
    type Error = io::Error;
    type ListenerUpgrade = ListenerUpgrade<O>;
    type Dial = Dial<O>;

    fn listen_on(
        &mut self,
        id: ListenerId,
        addr: Multiaddr,
    ) -> Result<(), TransportError<Self::Error>> {
        self.inner.listen_on(id, addr)
    }

    fn remove_listener(&mut self, id: ListenerId) -> bool {
        self.inner.remove_listener(id)
    }

    fn dial(&mut self, addr: Multiaddr) -> Result<Self::Dial, TransportError<Self::Error>> {
        self.inner.dial(addr)
    }

    fn dial_as_listener(
        &mut self,
        addr: Multiaddr,
    ) -> Result<Self::Dial, TransportError<Self::Error>> {
        self.inner.dial_as_listener(addr)
    }

    fn address_translation(&self, server: &Multiaddr, observed: &Multiaddr) -> Option<Multiaddr> {
        self.inner.address_translation(server, observed)
    }

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<TransportEvent<Self::ListenerUpgrade, Self::Error>> {
        Pin::new(self.inner.as_mut()).poll(cx)
    }
}

impl<O> Stream for Boxed<O> {
    type Item = TransportEvent<ListenerUpgrade<O>, io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Transport::poll(self, cx).map(Some)
    }
}

impl<O> FusedStream for Boxed<O> {
    fn is_terminated(&self) -> bool {
        false
    }
}

fn box_err<E: Error + Send + Sync + 'static>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}
