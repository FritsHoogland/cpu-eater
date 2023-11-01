use std::thread;
use std::env;
use os_id::thread as osthread;

//const NTHREADS: usize = 9;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nthreads = &args[1];

    let mut threads = vec![];

    for _nr in 0..nthreads.parse::<i32>().unwrap() {
        threads.push(thread::spawn(|| {
            println!("threadid: {:?}", osthread::get_raw_id());
            let mut _x=0;
            loop
            {
                _x += 1;
                _x -= 1;
            }
        }));
    };

    for thread in threads {
        let _ = thread.join();
    };
}
