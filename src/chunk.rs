extern crate bytes;
extern crate futures;
//extern crate http;
extern crate hyper;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_process;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use futures::Future;
use hyper::rt;
use hyper::service::service_fn;
use hyper::StatusCode;
use hyper::{Body, Chunk, Request, Response, Server};
use tokio::codec::{Decoder, FramedRead};
use tokio::fs::File;
use tokio::io;
use futures::future::ok;
use crate::fileio::load_local_mp3_buffer;
use std::process::{Command, Stdio};
//use std::error::Error;
use std::io::{Read, ErrorKind, stdout};
use core::borrow::BorrowMut;
use std::error::Error;
use std::convert::TryInto;
use self::tokio_process::CommandExt;


/*impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error::Request(error)
    }
}*/

//pub fn handle_request(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = std::io::Error> {
pub fn handle_request(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {

    //let myfuture = ok::<_, ()>(String::from("hello"));
    //myfuture

    let fuckfuture = ok::<_, hyper::Error>(
        {
            //let fff = File::open("p.mp3").map(file_response);
            /*let fff = File::open("p.mp3").and_then(

                Ok(())
            );
            tokio::run(fff);*/
            //fff.ru;

            let client = File::open("p.mp3")
                .map(|file| {
                    let stream = FramedRead::new(file, ChunkDecoder);

                })
                .map_err(|err| {
                    // All tasks must have an `Error` type of `()`. This forces error
                    // handling and helps avoid silencing failures.
                    //
                    // In our example, we are only going to log the error to STDOUT.
                    println!("file error = {:?}", err);
                });

            tokio::spawn(client);

            //Response::new(Body::from(fff.into()))
            Response::new(Body::from("FUCKING HYYYYYYYYYYYYYYPAAAAAAAAAAAAAAAAAAAAAAA"))
        });
    //return fuckfuture;

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
    let myfuture = ok::<_, hyper::Error>(
        {


            let command_name = "ffmpeg";
            //let command_opts = ["-i", "pipe:0", "-f", "mp3", "-acodec", "libvorbis", "-ab", "128k", "-aq", "60", "-f", "ogg", "-"];
            //"D:\Program Files\ffmpeg\bin\ffmpeg" -re -i "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg"
            // -acodec libmp3   lame -ab 128k -aq 60 -f mp3 - > bla.mp3

            //let media_addr = "https://cdn.netzpolitik.org/wp-upload/2019/02/NPP169-Worum-geht-es-eigentlich-bei-der-ePrivacy-Reform.ogg";
            let media_addr = "https://upload.wikimedia.org/wikipedia/commons/f/f2/Median_test.ogg";
            let command_opts = ["-i", media_addr,
                "-acodec", "libmp3lame", "-ab", "128k", "-aq", "60", "-f", "mp3", "-"];
            let mut ffmpeg_path = command_name;
            if cfg!(target_os = "windows") {
                ffmpeg_path = "D:/Program Files/ffmpeg/bin/ffmpeg.exe";
            }

            // Use the standard library's `Command` type to build a process and
            // then execute it via the `CommandExt` trait.
            let process = Command::new(ffmpeg_path)
                .args(&command_opts)
                //.stdin(Stdio::piped())
                //.stdout(Stdio::piped())
                .output_async();
                //{
//                    Err(why) => panic!("couldn't spawn {}: {}", command_name, why.description()),
  //                  Ok(process) => process,
    //            };

            use std::io;
            use std::process::{Command, Stdio, Output};

            use futures::{Future, Stream};
            use tokio_process::{CommandExt, Child};


            fn print_lines(mut cat: Child) -> Box<Future<Item = (), Error = ()> + Send + 'static> {
                let stdout = cat.stdout().take().unwrap();
                let reader = io::BufReader::new(stdout);
                let lines = tokio_io::io::lines(reader);
                let cycle = lines.for_each(|l| {
                    println!("Line: {}", l);
                    Ok(())
                });

                let future = cycle.join(cat)
                    .map(|_| ())
                    .map_err(|e| panic!("{}", e));

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

            let mut cmd = Command::new(ffmpeg_path);
            cmd.args(&command_opts);
            cmd.stdout(Stdio::piped());
            let future = print_lines(cmd.spawn_async().expect("failed to spawn command"));
            tokio::spawn(future);
            /*// The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
            let mut buffer: Vec<u8> = Vec::new();
            match process.stdout.unwrap().read_to_end(&mut buffer) {
                Err(why) => panic!("couldn't read {} stdout: {}", command_name, why.description()),
                Ok(_) => println!("buffer size:[{}]", buffer.len()),
            }*/

            //**/response.body_mut() = Body::from(buffer);
            //return Box::new( future::ok(response));

            //let stre = process.stdout.unwrap().read_vectored();




            //let buffer = load_local_mp3_buffer();
            //Response::new(Body::from(buffer))
            Response::new(Body::from("tokioooooooo"))
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
