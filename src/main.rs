
// parallel pipelines today!

// crates being used
extern crate crossbeam;
extern crate crossbeam_channel;

// imports
use std::thread; // for threads
use std::time::Duration; // for timers
use crossbeam_channel::bounded; // to bind threads

fn main()
{
    // sender and reciever 1 and 2
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    
    // messages and workers
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // producer thread
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // close the channel
            drop(snd1);
        });

        // parallel processing by 2 threads
        for _ in 0..n_workers {
            // send to sink, recieve from source
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());

            // spawn workers in seperate threads
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // recieve until channel closes
                for msg in recvr.iter(){
                    println!("worker {:?} recieved {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // close the channel, otherwise the sink will never exit for-loop
        drop(snd2);

        // sink
        for msg in rcv2.iter()
        {
            println!("sink recieved {}", msg);
        }
    }).unwrap();
}