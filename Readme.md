
# Simple OS

![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/Simple_OS)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/Simple_OS)
![GitHub stars](https://img.shields.io/github/stars/CMIW/Simple_OS?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/Simple_OS?style=social)

This project is an experiment to learn Kernel and OS development concepts. It's a small operating system in the [Rust programming language](https://www.rust-lang.org/) following the [Writting an OS in Rust](https://os.phil-opp.com/) series. It only supports x86.

## Kernel Hello World!
![Hello World on qemu-system-x86_64](https://github.com/CMIW/Simple_OS/blob/main/Kernel-Hello-World!.png)

## Requirements
- Linux system
- [rustup](https://rustup.rs/)
- qemu run ```sudo apt install qemu-system-x86```

Clone the project and in the project root run the following commands.<br>

- llvm-tools-preview run ```rustup component add llvm-tools-preview```
- rust-src run ```rustup component add rust-src```
- bootimage run ```cargo install bootimage```

## Building
This project requires a nightly version of Rust because it uses some unstable features. At least nightly _2020-07-15_ is required for building. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing it.

You can build the project by running:

```
cargo build
```

To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage

```
cargo install bootimage
```

After installing, you can create the bootable disk image by running:

```
cargo bootimage
```

This creates a bootable disk image in the `target/x86_64-simple_os/debug` directory.

## Running
[QEMU]: https://www.qemu.org/
You can run the disk image in [QEMU] through:

```
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

You can also write the image to an USB stick for booting it on a real machine. On Linux, the command for this is:

```
dd if=target/x86_64-simple_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.


## Some sources
[aarch64](https://krinkinmu.github.io/2020/12/13/adding-rust-to-aarch64.html)<br>
[Booting AArch64 Linux](https://www.kernel.org/doc/html/latest/arm64/booting.html)<br>
[Raspberry Pi Bare Bones](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones#Booting_the_Kernel)<br>
[Raspberry Pi Bare Bones Rust](https://wiki.osdev.org/Raspberry_Pi_Bare_Bones_Rust)<br>
[Writing my own Bootloader](https://dev.to/frosnerd/writing-my-own-boot-loader-3mld)<br>
[os-tutorial](https://github.com/cfenollosa/os-tutorial)<br>
[Writing a Simple Operating System — from Scratch](https://www.cs.bham.ac.uk/~exr/lectures/opsys/10_11/lectures/os-dev.pdf)<br>
https://github.com/ababo/arwen<br>
[Booting AArch64 Linux](https://www.kernel.org/doc/Documentation/arm64/booting.txt)<br>
[rpi4-osdev](https://github.com/isometimes/rpi4-osdev)<br>
[rust_os](https://github.com/thepowersgang/rust_os)<br>
