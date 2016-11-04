# Setting up Rust

This section will talk you through setting up Rust for the first time and also how to keep it up to date.

Set up depends upon your target platform. Rust runs on Windows, Linux and MacOS. In addition you might wish to cross-compile code for the consumption of another platform.

## Use Rustup

The easiest way to get started is to download and run rustup-init which you can do by visiting the [Rustup site](https://www.rustup.rs/).

The instructions differ for Windows and Unix-like systems:

* On Windows, rustup-init is an exe installer.
* On Unix \/ OS X \/ Linux rust-init is a shell script.

Either way, when you follow the instructions rustup-init will download.

The installer will download and install put rustc, cargo, and rustup in your bin directory which is `~/.cargo/bin` on Unix and `%USERPROFILE%.cargo.\bin` on Windows. It will also set your `PATH` environment variable so that you can open a terminal and type rustc, cargo, rustup etc.

Once rustup is installed you can use the tool to:

* Install additional Rust toolchains \(e.g. if you are cross-compiling or supporting multiple targets you may have more than one toolchain\)
* Change the default toolchain that is invoked when you type "rustc" or "cargo". Rustup will create symbolic links \/ scripts that invoke the appropriate toolchain
* Updates the toolchain when a new version is released
* Fetches sources and documentation

### Unix \/ Linux

The process for running rustup-init.sh is as follows:
1. Open a terminal \/ console
2. Type "curl [https:\/\/sh.rust.rs](https://sh.rust.rs) -sSf \| sh"
3. This will execute a script which will examine your environment, recommend the toolchain to download, and offer to modify your PATH.
4. Choose the option 1 to proceed. Or customize if you want to modify something
5. Wait for download to complete
6. You're done.

If you don't have curl, then you must install it first to proceed. In Linux you would invoke a command like this to install it.

* Debian \/ Ubuntu - sudo apt get curl
* Fedora \/ Redhat - sudo dnf install curl

### Windows

1. Download rustup-init.exe from rustup.rs.
2. Double click on the rust-init.exe and a console will open
3. Choose the option 1 to proceed. Or customize if you want to modify something
4. Wait for download to complete
5. You're done.

If you prefer not to go with the defaults, here are some choices you should decide upon:

1. 32\/64 bit version. Most Windows installations are going to be 64-bits these days but you may have a reason to choose 32-bit.
2. GNU or MSVC ABI. This depends on what toolchain and runtimes you wish to be compatible with.

The second choice concerns the application binary interface \(ABI\) you want Rust to be compatible with.

* If you don't care about linking to anything then choose the GNU ABI. Also choose it if you have DLLs produced by MingW \/ MSYS. The advantage of this ABI is that it is more mature.
* If you have Visual Studio installed or intend to use Rust against DLLs created with Visual Studio, that's the ABI you need. One advantage of this option is that you can debug Rust inside of Visual Studio- the compiler will produce .pdb files that allow you to step debug Rust.

### Keeping Rust up to date

New versions of Rust appear in a semi-frequent basis. If you want to update your environment to the latest version, it is as simple as this:

```
rustup update
```

## Manual installation

If you prefer manual installation of Rust then there are packages and instructions on the [Rust site](https://www.rust-lang.org/en-US/downloads.html) for this purpose.

Just be aware that Rust has a fairly fast release cycle so you probably only want to do this if you have a reason to choose a specific version of Rust and stick with it. Otherwise you may find yourself uninstalling and reinstalling a new version6 weeks later all over again.

TODO

## Setting up a debugger

### Unix \/ Linux

Debugging Rust is little different from debugging C or C++.

You must install gdb for your platform and then you may invoke it from a console or your favourite front-end to debug Rust code.

TODO rust-gdb
TODO rust-lldb

### Windows

If you have chosen Rust with the MSVC ABI then you can debug through Visual Studio. When you create a debug build of your code, the compile will also create a .pdb file to go with it. You may open your executable in Visual Studio and step debug it.

TODO If you have chosen Rust with the GNU ABI, then you must debug with GDB

## Setting up an IDE

Rust is still behind some other languages when it comes to IDE integration but there are already plugins that provide much of the functionality you need.

Popular IDEs such as Eclipse, IntelliJ, Visual Studio all have plugins that work to varying degrees of integration with Rust.

* [Rust plugin for IntelliJ](https://intellij-rust.github.io/) is under active development. This plugin has a lot of traction and is turning around new versions on a nearly weekly basis.  Offers syntax highlighting, autocomplete \(via built-in parser\), cargo builts and eventually other functionality.
* [RustDT for Eclipse](https://github.com/RustDT/RustDT) is also under active development. It adds syntax highlighting, autocomplete \(via racer\), cargo builds and rustfmt functionality to Eclipse.
* [Visual Rust plugin for Microsoft Studio](https://github.com/PistonDevelopers/VisualRust) . Offers syntax highlighting, autocompletion, interactive debugging.
* Atom is a popular editor with heaps of plugins. These plugins are very useful for Rust:
  * [language-rust](https://atom.io/packages/language-rust) provides basic syntax highlighting
  * [racer](https://atom.io/packages/racer) for autocompletion functionality
  * [atom-beautify](https://atom.io/packages/atom-beautify) invokes rustfmt to make code look pretty.
  * [build-cargo](https://atom.io/packages/build-cargo) invokes cargo for you showing errors and warnings inline.


For other editors and IDEs refer to [https:\/\/forge.rust-lang.org\/ides.html](https://forge.rust-lang.org/ides.html)

## Racer \/ Rustfmt

Some of the plugins above make use of Racer and Rustfmt.

Racer is used by some plugins to provide autocompletion functionality.

Rustfmt is a source code formatting tool that makes sure your Rust source code is pretty to look at, adding spacing, indentation and so on.

You can get both just by typing these commands and waiting for the tools to download and build themselves - they're written in Rust and built through cargo.

```
cargo install racer
cargo install rustfmt
```

