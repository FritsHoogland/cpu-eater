use std::{thread,
          env,
          process,
          time::{Instant,Duration},
          iter::repeat_with,
          hint::black_box,
          sync::{Arc,
                 atomic::{AtomicU64,Ordering}
          },
};
use os_id::thread as osthread;

const COUNTER_STEP: usize = 1_000;

#[repr(align(64))]
struct AlignedAtomicU64(AtomicU64);

fn main()
{
    // collect number of threads from first argument, and panic if it doesn't exist.
    let args: Vec<String> = env::args().collect();
    let nthreads = if args.len() > 1
    {
        args[1].parse::<usize>().unwrap()
    }
    else
    {
        1
    };
    let sleep_time = if args.len() > 2
    {
        args[2].parse::<usize>().unwrap()
    }
    else
    {
        0
    };


    // vector for join handles
    let mut threads = vec![];
    // created vector for the counters for the threads.
    let counter_vector: Vec<_> =
        repeat_with(|| Arc::new(AlignedAtomicU64(AtomicU64::new(0))))
            .take(nthreads)
            .collect();

    /*
    static counter_vector: [AlignedAtomicU64; nthreads] = for _ in 0..nthreads {
        AlignedAtomicU64(AtomicU64::new(0));
    };

     */

    let timer = Instant::now();
    for nr in 0..nthreads
    {
        let counter_vector_clone = counter_vector.clone();
        threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
        {
            println!("threadid: {:?}", osthread::get_raw_id());
            let mut _x=0;
            let mut loop_counter=0;
            loop
            {
                _x = black_box(black_box(_x) + black_box(1));
                _x = black_box(black_box(_x) - black_box(1));
                loop_counter +=1;
                if loop_counter == COUNTER_STEP
                {
                    let _ = counter_vector_clone[nr].0.fetch_add(1,Ordering::Relaxed);
                    loop_counter = 0;
                }
            }
        }));
    };

    if sleep_time > 0
    {
        thread::sleep(Duration::from_secs(sleep_time.try_into().unwrap()));
        print_statistics_and_terminate(timer, counter_vector.clone());
    }

    ctrlc::set_handler(move ||
    {
        print_statistics_and_terminate(timer, counter_vector.clone());
    }).expect("Error setting ctrlc handler");


    for thread in threads
    {
        let _ = thread.expect("Getting thread handle failed").join();
    };
}

fn print_statistics_and_terminate(
    timer: Instant,
    counter_vector: Vec<Arc<AlignedAtomicU64>>,
)
{
        println!();
        let time_spent_millis = timer.elapsed().as_millis();
        let mut total = 0;
        for (nr, number) in counter_vector.iter().map(|number| number.0.load(Ordering::Relaxed)).enumerate()
        {
            total += number as u128/time_spent_millis;
            println!("thread number: {}, steps: {}, steps/ms: {}, ", nr, number as u128, number as u128/time_spent_millis);
        };
        println!("Total steps/ms: {}, total time of test: {:?}", total, timer.elapsed());
        process::exit(0);
}
