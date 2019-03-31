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
use core::borrow::{BorrowMut, Borrow};
use std::convert::TryInto;
use std::error::Error;
use std::io::{stdout, BufReader, ErrorKind, Read};

use self::tokio::codec::BytesCodec;
use bytes::BytesMut;
use futures::future;
use futures::try_ready;
use tokio::codec;
use tokio::prelude::*;
use crate::stream;
use crate::stream::ChunkStream;

/*impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::Request(error)
    }
}*/

pub struct ByteStream<R>(R);
impl<R: AsyncRead> Stream for ByteStream<R> {
    // The same as our future above:
    type Item = hyper::Chunk;
    type Error = io::Error;

    // poll is very similar to our Future implementation, except that
    // it returns an `Option<u8>` instead of a `u8`. This is so that the
    // Stream can signal that it's finished by returning `None`:
    fn poll(&mut self) -> Result<Async<Option<hyper::Chunk>>, io::Error> {
        //const BUFFER_SIZE:usize = 8 * 8 * 1024;
        const BUFFER_SIZE: usize = 4;
        use bytes::{BufMut, BytesMut};
        let mut buf = [0; BUFFER_SIZE];
        match self.0.poll_read(&mut buf) {
            Ok(Async::Ready(n)) => {
                // By convention, if an AsyncRead says that it read 0 bytes,
                // we should assume that it has got to the end, so we signal that
                // the Stream is done in this case by returning None:
                if n == 0 {
                    println!("ByteStream reached endpoint");
                    Ok(Async::Ready(None))
                } else {
                    //Ok(Async::Ready(Some(Chunk::from("Chunky!"))))
                    //println!("Chunk read[{}]", n);
                    let xbuf = buf[0];
                    let cbuf = &[xbuf];
                    //let bbuf: &mut bytes::BytesMut;
                    let mut bbuf = BytesMut::with_capacity(n);
                    //buf.iter().map(|b| bbuf.put_u8(*b));
                    for i in 0..n {
                        bbuf.put(buf[i]);
                    }

                    //let ybuf = String::from(xbuf);
                    //let zbuf = xbuf.encode()take().freeze().into();
                    //Ok(Async::Ready(Some(Chunk::from(String::from_utf8_lossy(cbuf).into_owned()))))
                    //hint:    Ok(Some(buf.take().freeze().into()))
                    let chunk = Chunk::from(bbuf.freeze());
                    Ok(Async::Ready(Some(chunk)))
                }
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
            _ => { Err(std::io::Error::new(ErrorKind::Other, "ByteStream poll_read error.")) }
        }
    }
}

//pub fn handle_request(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = std::io::Error> {
pub fn handle_request(
    req: Request<Body>,
) -> impl Future<Item = Response<Body>, Error = hyper::Error> {
    //let myfuture = ok::<_, ()>(String::from("hello"));
    //myfuture

    let fuckfuture = ok::<_, hyper::Error>({
        //let fff = File::open("p.mp3").map(file_response);
        /*let fff = File::open("p.mp3").and_then(

            Ok(())
        );
        tokio::run(fff);*/
        //fff.ru;

        /* let client = File::open("p.mp3")
        .map(|file| {
            let stream = FramedRead::new(file, ChunkDecoder);
        })
        .map_err(|err| {
            // All tasks must have an `Error` type of `()`. This forces error
            // handling and helps avoid silencing failures.
            //
            // In our example, we are only going to log the error to STDOUT.
            println!("file error = {:?}", err);
        });*/

        //tokio::spawn(client);

        //Response::new(Body::from(fff.into()))
        //Response::new(Body::from("FUCKING HYYYYYYYYYYYYYYPAAAAAAAAAAAAAAAAAAAAAAA"))

        //let stream = FramedRead::new(file, ChunkDecoder);
        //Response::new(Body::wrap_stream(stream))
        /*      let client2 = File::open("p.mp3")
        .map(|file| {
            let stream = FramedRead::new(file, ChunkDecoder);
        })
        .map_err(|err| {
            // All tasks must have an `Error` type of `()`. This forces error
            // handling and helps avoid silencing failures.
            //
            // In our example, we are only going to log the error to STDOUT.
            println!("file error = {:?}", err);
        });*/

        /*let rotz = || {
            let file = File::open("p.mp3").map_err(|err| println!("file error = {:?}", err));
            let stream = FramedRead::new(file.into_future(), ChunkDecoder);
            stream
            //Response::new(Body::wrap_stream(stream))
        };*/

        /*let hurenvotze = || {
            let data_fuck = vec!["FUCK", " ", "YOU!"];
            let stream_fuck = futures::stream::iter_ok::<_, ::std::io::Error>(data_fuck);
            data_fuck
            //Response::new(Body::wrap_stream(stream))
        };*/

        let data_fuck = vec!["FUCK", " ", "YOU!"];
        let stream_fuck = futures::stream::iter_ok::<_, ::std::io::Error>(data_fuck);

        //let byte_stream1 = ByteStream(io::stdin());
        // via https://jsdw.me/posts/rust-futures-tokio/
        /*        let byte_stream = codec::FramedRead::new(io::stdin(), FuckingCodec::new())
        //let byte_stream = codec::FramedRead::new(io::stdin(), ChunkDecoder)
            // convert our bytes buffer into a stream that emits one byte at a time:
            .map(|chunk| stream::iter_ok::<_, io::Error>( {
                //Chunk::from("hahaha")
                chunk
            }))
            // flatten our stream of streams down into one stream:
            .flatten();*/

        let command_name = "ffmpeg";
        //let command_opts = ["-i", "pipe:0", "-f", "mp3", "-acodec", "libvorbis", "-ab", "128k", "-aq", "60", "-f", "ogg", "-"];
        //"D:\Program Files\ffmpeg\bin\ffmpeg" -re -i "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg"
        // -acodec libmp3   lame -ab 128k -aq 60 -f mp3 - > bla.mp3

        let media_addr = "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg";
        //let media_addr = "https://upload.wikimedia.org/wikipedia/commons/f/f2/Median_test.ogg";
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
            "-"
        ];
        let mut ffmpeg_path = command_name;
        if cfg!(target_os = "windows") {
            ffmpeg_path = "D:/Program Files/ffmpeg/bin/ffmpeg.exe";
        }

        // Use the standard library's `Command` type to build a process and
        // then execute it via the `CommandExt` trait.
        let process = Command::new(ffmpeg_path)
            .args(&command_opts)
            //.stdin(Stdio::piped())
            .stdout(Stdio::piped());
        //.output_async();

        use std::io;
        use std::process::{Command, Output, Stdio};

        use futures::{Future, Stream};
        use tokio_process::{Child, CommandExt};

        fn print_lines(mut cat: Child) -> Box<Future<Item = (), Error = ()> + Send + 'static> {
            let stdout = cat.stdout().take().unwrap();
            let reader = io::BufReader::new(stdout);
            let lines = tokio_io::io::lines(reader);
            let cycle = lines.for_each(|l| {
                println!("Line: {}", l);
                Ok(())
            });

            let future = cycle.join(cat).map(|_| ()).map_err(|e| panic!("{}", e));

            Box::new(future)
        }

        fn async_stream2(mut cat: Child) -> Box<Future<Item = (), Error = ()> + Send + 'static> {
            /*let stdout = cat.stdout().take().unwrap();
            let reader = io::BufReader::new(stdout);

            //let dings = tokio_io::io::lines(reader.clone());

            let lines = tokio_io::io::lines(reader);
            let cycle = lines.for_each(|l| {
                println!("Line: {}", l);
                Ok(())
            });
            let future = cycle.join(cat).map(|_| ()).map_err(|e| panic!("{}", e));*/

            fn my_fut() -> impl Future<Item = (), Error = () > {
                println!("In the FUTURE: {}", 1);
                ok(())
            }

            //let future:Future<Item = (), Error = ()> = ok::<(), ()>(());
            Box::new(my_fut())
        }

        fn async_stream(mut cat: Child) -> Box<Future<Item = (), Error = ()> + Send + 'static> {
            /*let stdout = cat.stdout().take().unwrap();
            let reader = io::BufReader::new(stdout);

            //let dings = tokio_io::io::lines(reader.clone());

            let lines = tokio_io::io::lines(reader);
            let cycle = lines.for_each(|l| {
                println!("Line: {}", l);
                Ok(())
            });
            let future = cycle.join(cat).map(|_| ()).map_err(|e| panic!("{}", e));*/

            fn my_fut() -> impl Future<Item = (), Error = () > {
                println!("In the FUTURE: {}", 1);
                ok(())
            }

            //let future:Future<Item = (), Error = ()> = ok::<(), ()>(());
            Box::new(my_fut())
        }

        /*let mut cmd = Command::new(ffmpeg_path);
        cmd.args(&command_opts);
        cmd.stdout(Stdio::piped());
        let future = async_stream(cmd.spawn_async().expect("failed to spawn command"));
        tokio::spawn(future);*/


        /*// the trait `futures::Stream` is not implemented for `dyn futures::Future<Item=(), Error=()> + std::marker::Send`
        fn my_fut() -> impl Future<Item = Stream<Item =(), Error=()> + 'static, Error = () > {
            println!("In the FUTURE: {}", 1);
            ok(())
        }*/

        /*let std_file = std::fs::File::open("p.mp3").unwrap();
        let file = tokio::fs::File::from_std(std_file);
        let byte_stream2: ByteStream<File> = ByteStream(file);
        let test = Response::new(Body::wrap_stream(fib));*/

        /*impl AsyncRead for File {
            unsafe fn prepare_uninitialized_buffer(&self, _: &mut [u8]) -> bool {
                false
            }
        }*/

        //let byte_stream2 = ByteStream(io::stdin());
        //let file: tokio::fs::File = File::open("p.mp3").map_err(|err| println!("file error = {:?}", err)).into();
        // x let std_file = std::fs::File::open("p.mp3").unwrap();
        // x let file = tokio::fs::File::from_std(std_file);
        //let byte_stream2 = ByteStream(file);

        //let xxxxx: tokio_io::AsyncRead = file;

        //let mut child = cmd.spawn_async().expect("failed to spawn command");
        //let childout = child.stdout().expect( "Fuck ya" );

        //use std::io;
        use std::process::ChildStdout;
        //use std::process::Stdio;
        //use std::io::BufReader;

        // Use the standard library's `Command` type to build a process and
        // then execute it via the `CommandExt` trait.
        let child = Command::new(ffmpeg_path)
            .args(&command_opts)
            //.stdin(Stdio::piped())
            .stdout(Stdio::piped());
            //.output_async();
            //  .spawn()
            //.expect(&format!("Error starting {}", ffmpeg_path));

        //let byte_stream2 = ByteStream(child.stdout.unwrap());
        //let mut reader = BufReader::new(child.stdout.unwrap());
        //let mut reader = FuckReader::new(child.stdout.unwrap());
        //let mut buf = Vec::new();
        //reader.read_to_end(&mut buf);
        //Response::new(Body::from(buf))

        /*{
            use std::{thread, time};

            let mut sleep_time_i = 60;
            if cfg!(target_os = "windows") {
                sleep_time_i = 3;
            }
            let sleep_time = time::Duration::from_secs(sleep_time_i);
            //let now = time::Instant::now();
            thread::sleep(sleep_time);
        }*/

        // working direct tokio ByteStream example
        //let std_file = std::fs::File::open("result.mp3").unwrap();
     //   let std_file = std::fs::File::open("p.mp3").unwrap();
     //   let file = tokio::fs::File::from_std(std_file);

        //let filex = tokio::fs::File::framed(file, ChunkDecoder);
        //let blalalad = filex.

        let fib = ChunkStream::new(media_addr);
        Response::new(Body::wrap_stream(fib))

        //let byte_stream2: ByteStream<ChildStdout> = ByteStream(child.stdout.unwrap());
        //let byte_stream2: ByteStream<File> = ByteStream(file);
        //Response::new(Body::wrap_stream(byte_stream2))
    });
    return fuckfuture;

    //let mut result = Result::Ok(());
    //let custom_error = Error::new(ErrorKind::Other, "oh no!");
    //let fff: OrElse<Map<OpenFuture<&str>, fn(File) -> Response<Body>>, Result<Response<Body>, Error>, fn(Error) -> Result<Response<Body>, Error>> = File::open("p.mp3").map(file_response)
    //    let fff:Request<Body> = File::open("p.mp3").map(file_response)
    //.or_else(|_| status_response(hyper::http::StatusCode::NOT_FOUND));
    //.or_else(|_| ok::<_, hyper::Error>(Response::new(Body::from("ERRrrroooooooooooooooor"))));
    //.or_else(|e| e.into());

    // .or_else(|e| result.unwrap());
    ;

    //let xxx = fff.into_stream();

    //Ok(());

    //return fff;

    /**/
    let myfuture = ok::<_, hyper::Error>({
        let command_name = "ffmpeg";
        //let command_opts = ["-i", "pipe:0", "-f", "mp3", "-acodec", "libvorbis", "-ab", "128k", "-aq", "60", "-f", "ogg", "-"];
        //"D:\Program Files\ffmpeg\bin\ffmpeg" -re -i "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg"
        // -acodec libmp3   lame -ab 128k -aq 60 -f mp3 - > bla.mp3

        //let media_addr = "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg";
        let media_addr = "https://upload.wikimedia.org/wikipedia/commons/f/f2/Median_test.ogg";
        let command_opts = [
            "-i",
            media_addr,
            "-acodec",
            "libmp3lame",
            "-ab",
            "128k",
            "-aq",
            "60",
            "-f",
            "mp3",
            "-",
        ];
        let mut ffmpeg_path = command_name;
        if cfg!(target_os = "windows") {
            ffmpeg_path = "D:/Program Files/ffmpeg/bin/ffmpeg.exe";
        }

        // Use the standard library's `Command` type to build a process and
        // then execute it via the `CommandExt` trait.
        let process = match Command::new(ffmpeg_path)
            .args(&command_opts)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn {}: {}", command_name, why.description()),
            Ok(process) => process,
        };

        use std::io;
        use std::process::{Command, Output, Stdio};

        use futures::{Future, Stream};
        use tokio_process::{Child, CommandExt};

        fn print_lines(mut cat: Child) -> Box<Future<Item = (), Error = ()> + Send + 'static> {
            let stdout = cat.stdout().take().unwrap();
            let reader = io::BufReader::new(stdout);
            let lines = tokio_io::io::lines(reader);
            let cycle = lines.for_each(|l| {
                println!("Line: {}", l);
                Ok(())
            });

            let future = cycle.join(cat).map(|_| ()).map_err(|e| panic!("{}", e));

            Box::new(future)
        }

        /*let future = process.map_err(|e| panic!("failed to collect output: {}", e))
        .map(|output: Output| {
            //assert!(output.status.success());
            //assert_eq!(output.stdout, b"hello world\n");

            let stdout = output.stdout;
            let reader = io::BufReader::new(stdout);
           // let lines = tokio_io::io::lines(reader);
           // let cycle = lines.for_each(|l| {
           //     println!("Line: {}", l);
          //      Ok(())
         //   });

        });*/

        /*let mut cmd = Command::new(ffmpeg_path);
        cmd.args(&command_opts);
        cmd.stdout(Stdio::piped());
        let future = print_lines(cmd.spawn_async().expect("failed to spawn command"));
        tokio::spawn(future);*/

        // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
        let mut buffer: Vec<u8> = Vec::new();

        match process.stdout.unwrap().read_to_end(&mut buffer) {
            Err(why) => panic!(
                "couldn't read {} stdout: {}",
                command_name,
                why.description()
            ),
            Ok(_) => println!("buffer size:[{}]", buffer.len()),
        }

        //**/response.body_mut() = Body::from(buffer);
        //return Box::new( future::ok(response));

        //let stre = process.stdout.unwrap().read_vectored();

        //let stream = FramedRead::new(file, ChunkDecoder);
        //Response::new(Body::wrap_stream(stream))

        //let buffer = load_local_mp3_buffer();
        let body = Body::from(buffer);
        Response::new(body)

        //for i in 0..10 {
        //}
        //Response::new(Body::from("tokioooooooo"))
    });
    return myfuture;
    /**/

    /*|| {
        Response::new(Body::from("Fuck yaaaaaaaaaaaa"))
    }*/
}

/*fn handle_request3(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {
    File::open(extract_path(&req))
        .map(file_response)
        //.or_else(|_| io::Error::new(io::ErrorKind::Other, "Bla"))
        .or_else(|e| Err(e))
}*/

/*fn handle_request2(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = http::Error> {
    File::open(extract_path(&req))
        .map(file_response)
        .or_else(|_| status_response(StatusCode::NOT_FOUND))
}*/

fn file_response(file: File) -> Response<Body> {
    let stream = FramedRead::new(file, ChunkDecoder);
    Response::new(Body::wrap_stream(stream))
}

fn status_response(status: StatusCode) -> hyper::http::Result<Response<Body>> {
    Response::builder().status(status).body(Body::empty())
}

fn extract_path(req: &Request<Body>) -> PathBuf {
    // Remove the first character
    PathBuf::from(req.uri().path().trim_left_matches('/'))
}

struct ChunkDecoder;

impl Decoder for ChunkDecoder {
    type Item = Chunk;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut bytes::BytesMut) -> Result<Option<Chunk>, io::Error> {
        let len = buf.len();
        if len > 0 {
            Ok(Some(buf.take().freeze().into()))
        } else {
            Ok(None)
        }
    }
}

/*pub struct FuckingCodec(());

impl FuckingCodec {
    /// Creates a new `BytesCodec` for shipping around raw bytes.
    pub fn new() -> FuckingCodec { FuckingCodec(())  }
}

impl Decoder for FuckingCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Chunk>, io::Error> {
        if buf.len() > 0 {
            let len = buf.len();
            //Ok(Some(buf.split_to(len)))
            Ok(Option::from(Chunk::from("Scheisse!! ")))
        } else {
            Ok(None)
        }
    }
}*/
