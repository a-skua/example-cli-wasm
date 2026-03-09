use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use wstd::io::{AsyncRead, AsyncWrite};
use wstd::net::TcpStream;

/// hyper の IO トレイトを WASI の TcpStream にブリッジするアダプター
pub struct WasiIo {
    stream: TcpStream,
}

impl WasiIo {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
}

impl hyper::rt::Read for WasiIo {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: hyper::rt::ReadBufCursor<'_>,
    ) -> Poll<Result<(), io::Error>> {
        let this = self.get_mut();
        let capacity = unsafe { buf.as_mut().len() };
        let mut tmp = vec![0u8; capacity];
        let mut stream = &this.stream;
        let result = {
            let mut fut = AsyncRead::read(&mut stream, &mut tmp);
            // SAFETY: fut はスタック上のローカル変数でありムーブされない
            let pinned = unsafe { Pin::new_unchecked(&mut fut) };
            pinned.poll(cx)
        };
        match result {
            Poll::Ready(Ok(n)) => {
                buf.put_slice(&tmp[..n]);
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl hyper::rt::Write for WasiIo {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = self.get_mut();
        let mut stream = &this.stream;
        let mut fut = AsyncWrite::write(&mut stream, buf);
        // SAFETY: fut はスタック上のローカル変数でありムーブされない
        let pinned = unsafe { Pin::new_unchecked(&mut fut) };
        pinned.poll(cx)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        let this = self.get_mut();
        let mut stream = &this.stream;
        let mut fut = AsyncWrite::flush(&mut stream);
        // SAFETY: fut はスタック上のローカル変数でありムーブされない
        let pinned = unsafe { Pin::new_unchecked(&mut fut) };
        pinned.poll(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}
