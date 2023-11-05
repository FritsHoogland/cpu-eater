use std::{thread,
          env,
          process,
          time::Instant,
          iter::repeat_with,
          sync::{Arc,
                 atomic::{AtomicU64,Ordering}
          },
};
use os_id::thread as osthread;

const COUNTER_STEP: usize = 100_000;

fn main()
{
    // collect number of threads from first argument, and panic if it doesn't exist.
    let args: Vec<String> = env::args().collect();
    let nthreads = &args[1].parse::<usize>().unwrap();

    // vector for join handles
    let mut threads = vec![];
    // created vector for the counters for the threads.
    let counter_vector: Vec<_> =
        repeat_with(|| Arc::new(AtomicU64::new(0)))
            .take(*nthreads)
            .collect();

    let timer = Instant::now();
    for nr in 0..*nthreads
    {
        let counter_vector_clone = counter_vector.clone();
        threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
        {
            println!("threadid: {:?}", osthread::get_raw_id());
            let mut _x=0;
            let mut loop_counter=0;
            loop
            {
                _x += 1;
                _x -= 1;
                loop_counter +=1;
                if loop_counter == COUNTER_STEP
                {
                    let _ = counter_vector_clone[nr].fetch_add(1,Ordering::Relaxed);
                    loop_counter = 0;
                }
            }
        }));
    };

    let _ = ctrlc::set_handler(move ||
    {
        println!();
        let time_spent_millis = timer.elapsed().as_millis();
        let mut total = 0;
        for (nr, number) in counter_vector.iter().map(|number| number.load(Ordering::Relaxed)).enumerate()
        {
            total += number as u128/time_spent_millis;
            println!("thread number: {}, steps: {}, steps/ms: {}, ", nr, number as u128, number as u128/time_spent_millis);
        };
        println!("Total steps/ms: {}, total time of test: {:?}", total, timer.elapsed());
        process::exit(0);
    });


    for thread in threads
    {
        let _ = thread.expect("Getting thread handle failed").join();
    };
}
