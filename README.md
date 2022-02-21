# packet_capture
これはプログラミング言語Rustを用いて，TCP/IP通信パケットをキャプチャするプラグラムである．

## Usage
### An example of interface
~~~
$ cargo run en0
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
    Running `target/debug/capture en0`
[en0]: TCP Packet: 10.70.70.xxx:xxx > 10.70.70.xxx:xxx; length: 142
[en0]: TCP Packet: 10.70.70.xxx:xxx > 10.70.70.xxx:xxx; length: 32
[en0]: TCP Packet: 10.70.70.xxx:xxx > xxxxxxxxxxxx:443; length: 87
[en0]: TCP Packet: xxxxxxxxxxxx:xxx > 10.70.70.xxx:xxx; length: 32
(略)
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