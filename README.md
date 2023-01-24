# rust-programming

#### Create a cargo package
```
cargo new hello_cargo
     Created binary (application) `hello_cargo` package
```
#### Build and execute your projec
```
cargo run     
   Compiling hello_cargo v0.1.0 (/Users/anmolpal/projects/systemsProgramming552/rust-programming/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/hello_cargo`
Hello, world!
```

##### Complie and run a rust program
```
(base) anmolpal@Anmols-Air src % pwd
/Users/anmolpal/projects/systemsProgramming552/rust-programming/hello_cargo/src
(base) anmolpal@Anmols-Air src % ls 
total 8
drwxr-xr-x  3 anmolpal  staff   96 Jan 24 16:07 .
-rw-r--r--  1 anmolpal  staff   45 Jan 24 16:07 main.rs
drwxr-xr-x  6 anmolpal  staff  192 Jan 24 16:10 ..
(base) anmolpal@Anmols-Air src % rustc main.rs 
(base) anmolpal@Anmols-Air src % ./main 
Hello, world!
```