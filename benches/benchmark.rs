use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::thread;
use std::{iter::repeat_with,sync::{Arc,atomic::{AtomicU64,Ordering,AtomicBool}}};

const COUNTER_STEP: usize = 1_000;
const LOOP_TOTAL: usize = 1_000_000;

fn no_shared_multi_thread(
   nr_threads: usize,
)
{
  let mut threads = vec![];

  for nr in 0..nr_threads
  {
      threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
      {
          let mut _x=0;
          let mut loop_counter=0;
          let mut _loop_total=0;
          for _ in 1..LOOP_TOTAL
          {
              _x = black_box(black_box(_x) + black_box(1));
              _x = black_box(black_box(_x) - black_box(1));
              loop_counter +=1;
              if loop_counter == COUNTER_STEP
              {
                  _loop_total += 1;
                  loop_counter = 0;
              }
          }
      }));
  };

  for thread in threads
  {
      let _ = thread.expect("Getting thread handle failed").join();
  };
}
fn shared_atomicu64_multi_thread(
   nr_threads: usize,
)
{
  let mut threads = vec![];
  let counter_vector: Vec<_> =
      repeat_with(|| Arc::new(AtomicU64::new(0)))
          .take(nr_threads)
          .collect();

  for nr in 0..nr_threads
  {
      let counter_vector_clone = counter_vector.clone();
      threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
      {
          let mut _x=0;
          let mut loop_counter=0;
          for _ in 1..LOOP_TOTAL
          {
              _x = black_box(black_box(_x) + black_box(1));
              _x = black_box(black_box(_x) - black_box(1));
              loop_counter +=1;
              if loop_counter == COUNTER_STEP
              {
                  let _ = counter_vector_clone[nr].fetch_add(1,Ordering::Relaxed);
                  loop_counter = 0;
              }
          }
      }));
  };

  for thread in threads
  {
      let _ = thread.expect("Getting thread handle failed").join();
  };
}
fn shared_atomicbool_multi_thread(
   nr_threads: usize,
)
{
  let mut threads = vec![];
  let stop_bool = Arc::new(AtomicBool::new(false));

  for nr in 0..nr_threads
  {
      let stop_bool_clone = stop_bool.clone();
      threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
      {
          let mut _x=0;
          let mut loop_counter=0;
          for _ in 1..LOOP_TOTAL
          {
              _x = black_box(black_box(_x) + black_box(1));
              _x = black_box(black_box(_x) - black_box(1));
              loop_counter +=1;
              if loop_counter == COUNTER_STEP
              {
                  loop_counter = 0;
                  if stop_bool_clone.load(Ordering::Relaxed) { break };
              }
          }
      }));
  };

  for thread in threads
  {
      let _ = thread.expect("Getting thread handle failed").join();
  };
}

fn benchmark_no_shared( 
    criterion: &mut Criterion,
)
{
    let mut group = criterion.benchmark_group("test");
    //group.measurement_time(std::time::Duration::from_secs(30));
    group.sample_size(500);
    group.bench_function("no shared 1 thread", |benchmark| benchmark.iter(|| no_shared_multi_thread(1)));
    group.bench_function("no shared 2 thread", |benchmark| benchmark.iter(|| no_shared_multi_thread(2)));
    group.bench_function("no shared 4 thread", |benchmark| benchmark.iter(|| no_shared_multi_thread(4)));
    group.bench_function("no shared 6 thread", |benchmark| benchmark.iter(|| no_shared_multi_thread(6)));
    group.bench_function("atomicu64 1 thread", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(1)));
    group.bench_function("atomicu64 2 thread", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(2)));
    group.bench_function("atomicu64 4 thread", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(4)));
    group.bench_function("atomicu64 6 thread", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(6)));
    group.bench_function("atomicbool 1 thread", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(1)));
    group.bench_function("atomicbool 2 thread", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(2)));
    group.bench_function("atomicbool 4 thread", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(4)));
    group.bench_function("atomicbool 6 thread", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(6)));
    group.finish();
}

criterion_group!(benches, benchmark_no_shared);
criterion_main!(benches);
