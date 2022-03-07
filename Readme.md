
# Simple OS

![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/Simple_OS)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/Simple_OS)
![GitHub stars](https://img.shields.io/github/stars/CMIW/Simple_OS?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/Simple_OS?style=social)

This project is a small operating system in the [Rust programming language](https://www.rust-lang.org/) following the [Writting an OS in Rust](https://os.phil-opp.com/) series.

## Requirements
- Linux system
- [rustup](https://rustup.rs/)
- qemu run ```sudo apt install qemu-system-x86```

Clone the project and in the project root run the following commands.<br>

- llvm-tools-preview run ```rustup component add llvm-tools-preview```
- rust-src run ```rustup component add rust-src```
- bootimage run ```cargo install bootimage```

## Building
Linux<br>
```cargo bootimage```

After building, if there where no errors, run ```qemu-system-x86_64 -drive format=raw,file=target/x86_64-simple_os/debug/bootimage-simple_os.bin```

## Kernel Hello World!
![Hello World on qemu-system-x86_64](https://github.com/CMIW/Simple_OS/blob/main/Kernel-Hello-World!.png)
