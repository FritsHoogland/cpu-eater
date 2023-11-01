# cpu-eater

This is a very simple rust project for performing activity on a computer system.
The activity is done by performing addition and subtraction of a variable in an endless loop:
```
loop
{
    _x += 1;
    _x -= 1;
}
```
Because modern computer systems commonly have multiple CPUs, threading is used to spawn threads for performing the activity.

The number of threads must be passed to the `cpu-eater` executable, otherwise it will panick:
```
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:9:21
```

To run it, simply use cargo:
```
$ cargo r --release 2
    Finished release [optimized] target(s) in 0.00s
     Running `/home/postgres/cpu-eater/target/release/cpu-eater 2`
threadid: 14449
threadid: 14450
```
Or compile it and run the executable:
```
$ cargo build --release
    Finished release [optimized] target(s) in 0.00s
$ ./target/release/cpu-eater 2
threadid: 14458
threadid: 14457
```
When `cpu-eater` is run, it shows the thread id's of the threads that it spawned.
These can be used for further investigation.

To cancel running `cpu-eater`, press control-c.
```
```
