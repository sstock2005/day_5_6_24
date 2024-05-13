
// today we are transmitting data between two pipelines!

use std::{thread, time};
use crossbeam_channel::unbounded;

fn main()
{
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            for i in 0..n_msgs 
            {
                snd.send(i).unwrap(); // send message to reciever
                thread::sleep(time::Duration::from_millis(100)); // wait 100 milliseconds
            }
        });
    }).unwrap();

    for _ in 0..n_msgs
    {
        let msg = rcv.recv().unwrap();
        println!("Recieved {} from other pipeline!", msg);
    }
}