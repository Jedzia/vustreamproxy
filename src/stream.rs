extern crate bytes;
extern crate futures;
//extern crate http;
extern crate hyper;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_process;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use crate::fileio::load_local_mp3_buffer;
use futures::future::ok;
use futures::Future;
use hyper::rt;
use hyper::service::service_fn;
use hyper::StatusCode;
use hyper::{Body, Chunk, Request, Response, Server};
use std::process::{Command, Stdio};
use tokio::codec::{Decoder, FramedRead};
use tokio::fs::File;
use tokio::io;
//use std::error::Error;
use self::tokio::prelude::future::IntoFuture;
use self::tokio_process::CommandExt;
use core::borrow::BorrowMut;
use std::convert::TryInto;
use std::error::Error;
use std::io::{stdout, BufReader, ErrorKind, Read};

use self::tokio::codec::BytesCodec;
use bytes::BytesMut;
use futures::future;
use futures::try_ready;
use tokio::codec;
use tokio::prelude::*;

use futures::{Async, Poll, Stream};

pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { curr: 1, next: 1 }
    }
}

impl Stream for Fibonacci {
    type Item = u64;

    // The stream will never yield an error
    type Error = ();

    fn poll(&mut self) -> Poll<Option<u64>, ()> {
        let curr = self.curr;
        let next = curr + self.next;

        self.curr = self.next;
        self.next = next;

        Ok(Async::Ready(Some(curr)))
    }
}

/*impl<R> ByteStream<R> {
    /*pub fn new(inner: R) -> ByteStream<R> {
        //ByteStream::with_capacity(1024, inner)
    }*/
    // A public constructor method
    pub fn new(contents: R) -> ByteStream<R> {
        /*ByteStream {
            0: (R)
        }*/
    }
}*/

/*impl AsyncRead for ByteStream<Chunk> {
    unsafe fn prepare_uninitialized_buffer(&self, _: &mut [u8]) -> bool {
        false
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Result::Ok(5)
    }
}*/

/*pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
    Command { inner: imp::Command::new(program.as_ref()) }
}*/

pub struct FuckReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    cap: usize,
}

impl<R: Read> FuckReader<R> {
    const DEFAULT_BUF_SIZE: usize = 8 * 1024;

    /// Creates a new `BufReader` with a default buffer capacity. The default is currently 8 KB,
    /// but may change in the future.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::io::BufReader;
    /// use std::fs::File;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("log.txt")?;
    ///     let reader = BufReader::new(f);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(inner: R) -> FuckReader<R> {
        FuckReader::with_capacity(8 * 1024, inner)
    }

    pub fn with_capacity(cap: usize, inner: R) -> FuckReader<R> {
        unsafe {
            let mut buffer = Vec::with_capacity(cap);
            buffer.set_len(cap);
            //inner.initializer().initialize(&mut buffer);
            FuckReader {
                inner,
                buf: buffer.into_boxed_slice(),
                pos: 0,
                cap: 0,
            }
        }
    }
}
