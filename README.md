# stat-rs

stat-rs is a copy of linux command [stat](https://man7.org/linux/man-pages/man1/stat.1.html)

## Example

stat-rs is only for unix-like system

````shell
# build stat-rs with release mode
$ cargo build --release

# run stat-rs
$ target/release/stat-rs Cargo.toml
  File: Cargo.toml
  Size: 204             Blocks: 8       IO Blocks: 4096 regular file
Device: b301h/45825d    Inode: 3410721  Links: 1
Access: (0100644/-rw-r--r--)    Uid: (  1000/   xxl)    Gid: (  1001/   xxl)
Access: 2021-07-04T13:19:22.924079072+00:00
Modify: 2022-01-20T12:58:01.804614602+00:00
Change: 2022-01-20T12:58:01.804614602+00:00
 Birth: -
````
