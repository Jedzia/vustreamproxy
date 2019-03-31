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
use std::process::{Child, ChildStdout, Command, Stdio};
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

pub struct ChunkStream {
    curr: u64,
    countdown: u64,
    buffer: Vec<u8>,
    //process: Child,
    cstdout: ChildStdout,
}

impl ChunkStream {
    pub fn new() -> ChunkStream {
        let command_name = "ffmpeg";
        //let media_addr = "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg";
        let media_addr = "https://upload.wikimedia.org/wikipedia/commons/f/f2/Median_test.ogg";
        let command_opts = [
            "-y", // overwrite
            //"-re", // realtime
            "-i",
            media_addr,
            "-acodec",
            "libmp3lame",
            //"-ab",
            //"128k",
            //"-aq",
            //"60",
            "-f",
            "mp3",
            //"result.mp3", // or - for stdout
            "-",
        ];
        let mut ffmpeg_path = command_name;
        if cfg!(target_os = "windows") {
            ffmpeg_path = "D:/Program Files/ffmpeg/bin/ffmpeg.exe";
        }

        // Use the standard library's `Command` type to build a process and
        // then execute it via the `CommandExt` trait.
        let mut process = match Command::new(ffmpeg_path)
            .args(&command_opts)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            //.spawn_async()
            .spawn()
        {
            Err(why) => panic!("couldn't spawn {}: {}", command_name, why.description()),
            Ok(process) => process,
        };

        {
            use std::{thread, time};

            let mut sleep_time_i = 3;
            if cfg!(target_os = "windows") {
                sleep_time_i = 3;
            }
            let sleep_time = time::Duration::from_secs(sleep_time_i);
            //let now = time::Instant::now();
            thread::sleep(sleep_time);
        }

        let mut buffer: Vec<u8> = Vec::new();
        //let mut buffer: Vec<u8> = Vec::with_capacity(256);
        buffer.resize(1024, 0);

        let cstdout: ChildStdout = process.stdout.unwrap();
        /*match process.stdout.unwrap().read_to_end(&mut buffer) {
            Err(why) => panic!(
                "couldn't read {} stdout: {}",
                command_name,
                why.description()
            ),
            Ok(_) => println!("buffer size:[{}]", buffer.len()),
        }*/

        ChunkStream {
            curr: 1,
            countdown: 3,
            buffer,
            //process,
            cstdout,
        }
    }
}

impl Stream for ChunkStream {
    type Item = hyper::Chunk;

    // The stream will never yield an error
    type Error = hyper::Error;

    fn poll(&mut self) -> Poll<Option<hyper::Chunk>, hyper::Error> {
        /*match self.cstdout.read_to_end(&mut self.buffer) {
            Err(why) => panic!(
                "couldn't read {} stdout from '{}'",
                "ffmpeg",
                why.description()
            ),
        }*/

        //self.buffer.resize(1024, 0);

        let mut n = 0;
        let result = self.cstdout.read(&mut self.buffer);
        n = result.unwrap();
        println!("Read {} bytes from ffmpeg", n);


        /*while n == 0 {
            let result = self.cstdout.read(&mut self.buffer);
            n = result.unwrap();
            if n != 0 {
                println!("Read {} bytes from ffmpeg", n);
            }
        }*/

        self.countdown = self.countdown - 1;
        if self.countdown <= 0 {
            println!("ChunkStream reached endpoint");
            Ok(Async::Ready(None))
        } else {
            //Ok(Async::Ready(Some(42)))
            Ok(Async::Ready(Some(Chunk::from(
                " Its from the Chunk Stream ",
            ))))
        }
    }
}

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
