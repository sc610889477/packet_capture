# packet_capture
これはプログラミング言語Rustを用いて，TCP/IP通信パケットをキャプチャするプラグラムである．

## Usage
### An example of get an interface
~~~
$ cargo run en0
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    Running `target/debug/capture en0`
Selected Interface Name: en0
$ 
~~~
or
~~~
$ cargo run 00
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    Running `target/debug/capture 00`
Failed get Interface!
$ 
~~~