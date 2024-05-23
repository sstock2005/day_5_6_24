
// today we are going to calculate a SHA256 sum of files

use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};
use text_io::read;

fn compute_sha256<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> 
{
    let mut buffer_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop 
    {
        let count = buffer_reader.read(&mut buffer)?;
        if count == 0
        {
            break;
        }
        context.update(&buffer[..count])
    }

    Ok((context.finish(), filepath))
}

fn main() -> Result<(), Error>
{
    let thread_pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    println!("Directory:");
    let directory: String = read!("{}\n");

    for file in WalkDir::new(directory)
       .follow_links(true)
       .into_iter()
       .filter_map(|e| e.ok())
       .filter(|e| !e.path().is_dir()) {
           let path = file.path().to_owned();
           let tx = tx.clone();
           thread_pool.execute(move || {
            let digest = compute_sha256(path);
            tx.send(digest).expect("Could not send data!");
           });
       }

       drop(tx);
       for t in rx.iter(){
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
       }
       Ok(())
}