use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::thread;
use std::{iter::repeat_with,sync::{Arc,atomic::{AtomicU64,Ordering,AtomicBool}}};

const COUNTER_STEP: u64 = 1_000;
const LOOP_TOTAL: usize = 1_000_000;

fn no_shared_op_multi_thread(
   nr_threads: usize,
)
{
  let mut threads = vec![];

  for nr in 0..nr_threads
  {
      threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
      {
          let mut _x: u64=0;
          let mut loop_counter: u64 =0;
          let mut _loop_total: u64 =0;
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
fn no_shared_multi_thread(
   nr_threads: usize,
)
{
  let mut threads = vec![];

  for nr in 0..nr_threads
  {
      threads.push(thread::Builder::new().name(format!("cpu-eater-w-{}",nr)).spawn(move ||
      {
          let mut _x: u64=0;
          let mut loop_counter: u64 =0;
          let mut _loop_total: u64 =0;
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
          assert_eq!(loop_counter, 999);
          assert_eq!(_loop_total, 999);
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
          let mut _x: u64=0;
          let mut loop_counter: u64 =0;
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
          assert_eq!(loop_counter, 999);
          assert_eq!(counter_vector_clone[nr].load(Ordering::Relaxed), 999);
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
          let mut _x: u64=0;
          let mut loop_counter: u64 =0;
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
          assert_eq!(loop_counter, 999);
          assert_eq!(stop_bool_clone.load(Ordering::Relaxed), false);
      }));
  };

  for thread in threads
  {
      let _ = thread.expect("Getting thread handle failed").join();
  };
}

fn benchmark_no_shared_optim( 
    criterion: &mut Criterion,
)
{
    let mut group = criterion.benchmark_group("non shared optim");
    //group.sample_size(1000);
    //group.measurement_time(std::time::Duration::from_secs(30));
    group.bench_function("1", |benchmark| benchmark.iter(|| no_shared_op_multi_thread(1)));
    group.bench_function("2", |benchmark| benchmark.iter(|| no_shared_op_multi_thread(2)));
    group.bench_function("4", |benchmark| benchmark.iter(|| no_shared_op_multi_thread(4)));
    group.bench_function("6", |benchmark| benchmark.iter(|| no_shared_op_multi_thread(6)));
    group.finish();
}

fn benchmark_no_shared_nonoptim(
    criterion: &mut Criterion,
)
{
    let mut group = criterion.benchmark_group("non shared nonoptim");
    //group.sample_size(1000);
    //group.measurement_time(std::time::Duration::from_secs(30));
    group.bench_function("1", |benchmark| benchmark.iter(|| no_shared_multi_thread(1)));
    group.bench_function("2", |benchmark| benchmark.iter(|| no_shared_multi_thread(2)));
    group.bench_function("4", |benchmark| benchmark.iter(|| no_shared_multi_thread(4)));
    group.bench_function("6", |benchmark| benchmark.iter(|| no_shared_multi_thread(6)));
    group.finish();
}

fn benchmark_shared_atomicu64(
    criterion: &mut Criterion,
)
{
    let mut group = criterion.benchmark_group("shared atomicu64");
    //group.sample_size(1000);
    //group.measurement_time(std::time::Duration::from_secs(30));
    group.bench_function("1", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(1)));
    group.bench_function("2", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(2)));
    group.bench_function("4", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(4)));
    group.bench_function("6", |benchmark| benchmark.iter(|| shared_atomicu64_multi_thread(6)));
    group.finish();
}

fn benchmark_shared_atomicbool(
    criterion: &mut Criterion,
)
{
    let mut group = criterion.benchmark_group("shared atomicbool");
    //group.sample_size(1000);
    //group.measurement_time(std::time::Duration::from_secs(30));
    group.bench_function("1", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(1)));
    group.bench_function("2", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(2)));
    group.bench_function("4", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(4)));
    group.bench_function("6", |benchmark| benchmark.iter(|| shared_atomicbool_multi_thread(6)));
    group.finish();
}

criterion_group!(benches, benchmark_no_shared_optim, benchmark_no_shared_nonoptim, benchmark_shared_atomicu64, benchmark_shared_atomicbool);
//criterion_group!(benches, benchmark_no_shared_nonoptim);
//criterion_group!(benches, benchmark_shared_atomicu64);
//criterion_group!(benches, benchmark_shared_atomicbool);
criterion_main!(benches);
