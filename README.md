# cc-tar-rs
`cc-tar-rs` is my attempt at writing the equivalent of GNU `tar` using Rust.

Inspired by [Coding Challenges](https://codingchallenges.fyi/challenges/challenge-tar/)

## What it does
The goal is to match the basic functionality of GNU `tar` for UStar formatted tarballs.

## Usage
`cc-tar-rs` can be built using `cargo` with:
`cargo build --release`

You can create a dummy tarball like so and run `cc-tar-rs` on it:
```bash
echo "file 1 contents" > file1.txt
echo "file 2 contents" > file2.txt
echo "file 3 contents" > file3.txt
echo "file 4 contents" > file4.txt

tar -cf test-archive.tar file1.txt file2.txt file3.txt file4.txt

cat ./test-archive.tar | ./target/release/cc-tar-rs -t
OR
./target/release/cc-tar-rs -t -f ./test-archive.tar
```

## Contributing
This is a personal project I'm worked on with the intention of learning Rust and learning how to work with binary and open standards. Contributions aren't welcome at this time, but maybe in future :)

## References
[Wikipedia article about tar](https://en.wikipedia.org/wiki/Tar_(computing))

[GNU tar handbook](https://www.gnu.org/software/tar/manual/)

## License
Licensed under the [Apache License Version 2.0](LICENSE)
