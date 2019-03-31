https://github.com/hyperium/hyper/issues/1468

I think you're in luck. (And me; I've been wanting this too.) Looks like 0.12.x is going to add it. See Body::wrap_stream on master and #1438.
@scottlamb
scottlamb commented on Mar 20, 2018

btw, even now on 0.11.x, you don't have to use Body at all; you can instead use something like hyper::Response<Box<Stream<Item = Vec<u8>, Error = Error> + Send>>. The downside is that you may be giving up compatibility with libraries/frameworks that expect Body.



from / GET, test stream generator

            /*// Write a string to the `stdin` of `wc`.
            //
            // `stdin` has type `Option<ChildStdin>`, but since we know this instance
            // must have one, we can directly `unwrap` it.
            match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
                Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
                Ok(_) => println!("sent pangram to wc"),
            }*/

            // Because `stdin` does not live after the above calls, it is `drop`ed,
            // and the pipe is closed.
            //
            // This is very important, otherwise `wc` wouldn't start processing the
            // input we just sent.

            // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
            /*let mut s = String::new();
            match process.stdout.unwrap().read_to_string(&mut s) {
                Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
                Ok(_) => print!("wc responded with:\n{}", s),
            }*/

            //*response.body_mut() = Body::from(load_local_mp3_buffer());

            //let fbuffer = filebuffer::FileBuffer::open("p.mp3").expect("failed to open file");
            //let rustShitFUCK = fbuffer.leak();
            //let fuckingBuffer: Vec<u8> = rustShitFUCK.iter().cloned().collect();
            //*response.body_mut() = Body::from(rustShitFUCK);
            //*response.body_mut() = Body::from("Try POSTing data to /echo");

pipe example

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog\n";

fn run_pipe_example() -> () {
    let mut wcpath = "wc";
    if cfg!(target_os = "windows") {
        wcpath = "C:/msys64/usr/bin/wc.exe";
    }

    // Spawn the `wc` command
    let process = match Command::new(wcpath)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why.description()),
        Ok(process) => process,
    };

    // Write a string to the `stdin` of `wc`.
    //
    // `stdin` has type `Option<ChildStdin>`, but since we know this instance
    // must have one, we can directly `unwrap` it.
    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why.description()),
        Ok(_) => println!("sent pangram to wc"),
    }

    // Because `stdin` does not live after the above calls, it is `drop`ed,
    // and the pipe is closed.
    //
    // This is very important, otherwise `wc` wouldn't start processing the
    // input we just sent.

    // The `stdout` field also has type `Option<ChildStdout>` so must be unwrapped.
    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why.description()),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}

load mp3 

fn load_local_mp3_buffer() -> Vec<u8> {
    //let mut f = File::open("./p.mp3").expect("failed to open mp3 file!");
    let mut filepath = "/media/hdd/jedzia/rust/p.mp3";
    if cfg!(target_os = "windows") {
        filepath = "p.mp3";
    }

    let mut f = File::open(filepath).expect("failed to open mp3 file!");
    //let mut f = File::open(filepath);
    //if let Err(err) = f{
    //    error!("failed to open mp3 file! {}", err);
    //}
    let mut buffer: Vec<u8> = Vec::new();
    f.read_to_end(&mut buffer)
        .expect("failed to read mp3 file.");
    buffer
}
//static mut buffer: Vec<u8> = Vec<u8>;
