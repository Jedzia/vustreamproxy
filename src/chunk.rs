extern crate bytes;
extern crate futures;
//extern crate http;
extern crate hyper;
extern crate tokio;

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


pub

fn handle_request(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = hyper::Error> {

    //let myfuture = ok::<_, ()>(String::from("hello"));
    //myfuture

    let myfuture = ok::<_, hyper::Error>(
        {
            let chunks = load_local_mp3_buffer();
            Response::new(Body::from(chunks))
        });
    myfuture

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
