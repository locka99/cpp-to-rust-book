# Setting up Rust

This section will talk you through setting up Rust for the first time and also how to keep it up to date.

Getting started is incredibly easy but some details vary upon your target platform. Rust runs on Windows, Linux and MacOS. In addition you might wish to cross-compile code for the consumption of another platform.

## Use Rustup

The easiest way to get started is to download and run `rustup-init` which you can do by visiting the [Rustup site](https://www.rustup.rs/).

The instructions differ for Windows and Unix-like systems:

* On Windows, rustup-init is an exe installer.
* On Unix / OS X / Linux rust-init is a shell script.

Either way, when you follow the instructions the installer will download and install put rustc, cargo, and rustup in your bin directory which is `~/.cargo/bin` on Unix and `%USERPROFILE%.cargo.\bin` on Windows. 

It will also set your `PATH` environment variable so that you can open a terminal and type rustc, cargo, rustup etc.

Once `rustup` is installed you can also use the tool for maintenance:

* Install additional Rust toolchains \(e.g. if you are cross-compiling or supporting multiple targets you may have more than one toolchain\)
* Change the default toolchain that is invoked when you type `rustc` or `cargo`. Rustup will create symbolic links / scripts that invoke the appropriate toolchain
* Update the toolchain when a new version of Rust is released
* Fetch source and documentation

### Unix / Linux

The process for running `rustup-init.sh` is as follows:

1. Open a terminal / console
2. Type "curl https://sh.rustup.rs -sSf | sh"
3. This will download and execute a script which will examine your environment, recommend the toolchain to download, and offer to modify your `PATH` environment variable.
4. Choose the option 1 to proceed. Or customize if you want to modify something
5. Wait for download to complete
6. You're done.

If you don't have curl, then you must install it first to proceed, or save the [shell script](https://sh.rustup.rs) from a browser to disk and execute that. 

To install `curl` in Linux you would invoke a command like this to install it.

* Debian / Ubuntu - `sudo apt get curl`
* Fedora / Redhat - `sudo dnf install curl`

### Windows

1. Download rustup-init.exe from rustup.rs.
2. Double click on the rust-init.exe and a console will open
3. Choose the option 1 to proceed. Or customize if you want to modify something
4. Wait for download to complete
5. You're done.

If you prefer not to go with the defaults, here are some choices you should decide upon:

1. 32/64 bit version. Most Windows installations are going to be 64-bits these days but you may have a reason to choose 32-bit.
2. GNU or MSVC ABI. This depends on what toolchain and runtimes you wish to be compatible with.

The second choice concerns the application binary interface \(ABI\) you want Rust to be compatible with.

* If you don't care about linking to anything then choose the GNU ABI. Also choose it if you have DLLs produced by MingW / MSYS. The advantage of this ABI is that it is more mature.
* If you have Visual Studio installed or intend to use Rust against DLLs created with Visual Studio, that's the ABI you need. One advantage of this option is that you can debug Rust inside of Visual Studio- the compiler will produce .pdb files that allow you to step debug Rust.

### Keeping Rust up to date

New versions of Rust appear in a semi-frequent basis. If you want to update your environment to the latest version, it is as simple as this:

```
rustup update
```

Sometimes `rustup` will need an update of its own in which case you type:

```
rustup self update
```

### Adding Rust source

Rustup installs a rust toolchain but if you're writing code or debugging you probably
should also get the Rust source code so you can step into it or look at the implementation:

```
rustup component add rust-src
```

## Manual installation

If you prefer manual installation of Rust then there are packages and instructions on the [Rust site](https://www.rust-lang.org/en-US/downloads.html).

Just be aware that Rust has a fairly rapid release cycle so you probably only want to do this if you have a reason to choose a specific version of Rust and stick with it.

Otherwise you may find yourself uninstalling and reinstalling a new version 6 weeks later all over again.

## Setting up a debugger

### Unix / Linux

Debugging Rust is little different from debugging C or C++.

You must install gdb for your platform and then you may invoke it from a console or your favourite front-end to debug Rust code.

On Linux systems you would normally install gdb from a package with one of these commands:

```
sudo apt-get install gdb
# or
sudo dnf install gdb
```

You may also prefer to use lldb which is a companion project to LLVM (the backend compiler used by Rust). Refer to the [lldb website](http://lldb.llvm.org/) for information on using it.

Rust comes with a few scripts that wrap gdb and lldb to provide pretty-printing to assist with debugging. When debugging, you can invoke `rust-gdb` or `rust-lldb` to use them.

### Windows

If you have chosen Rust with the MSVC ABI then you can debug through Visual Studio with some limitations. When you create a debug build of your code, the compile will also create a .pdb file to go with it. You may open your executable in Visual Studio and step debug it, inspect variables and so on. 

#### GDB

GDB on Windows is available through MSYS / MingW distributions.

For example downloads of the TDM-GCC distribution of MSYS can be found [here](http://tdm-gcc.tdragon.net/download). At the time of writing this, there is a standalone gdb-7.9.1-tdm64-2.zip containing the Choose the 32 or 64-bit version according to your Rust environment.

Extract the zip file to a directory, e.g. `C:\tools\gdb-7.9.1-tdm64-2` and add a value to your `PATH` environment variable:

```
set PATH=%PATH%;C:\tools\gdb-7.9.1-tdm64-2\bin\
```

You can invoke `gdb` from the command line but more normally you'd prefer a front end.

At the time of writing, perhaps the best option is Visual Studio Code which has plugins for debugging with GDB and for Rust development. So you can edit and debug from the same IDE.

##### Pretty printer

Rust supplies a pretty printer for variable inspection that you can add to the GDB.   The pretty printer is a script written in Python that GDB will invoke to display variables. 

First ensure you have Python 2.7 installed in your path.

The script is bundled with the Rust source code so you need to have installed that first.

If you installed it with `rustup` then it can be found in your `%USERPROFILE%\.rustup` directory:

e.g.

```
c:\users\MyName\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\src\rust\src\etc
```

Otherwise it can be found wherever you unzipped your Rust source code under `src\rust\src\etc`.

Note the fully qualified path its under and edit `C:\tools\gdb-7.9.1-tdm64-2\bin\gdbinit` to insert the path using *forward* slashes.

```
python
print "---- Loading Rust pretty-printers ----"
 
sys.path.insert(0, "C:/users/MyName/.rustup\toolchains/stable-x86_64-pc-windows-gnu/lib/rustlib/src/rust/src/etc")
import gdb_rust_pretty_printing
gdb_rust_pretty_printing.register_printers(gdb)
 
end
```

## Setting up an IDE

Rust is still behind some other languages when it comes to IDE integration but there are already plugins that provide much of the functionality you need.

Popular IDEs such as Eclipse, IntelliJ, Visual Studio all have plugins that work to varying degrees of integration with Rust.

* [Visual Studio Code](https://code.visualstudio.com/) (not to be confused with Visual Studio) is a cross-platform programming editor and has a lot of plugins. It can be set up into a complete Rust development environment by following this [tutorial](https://sherryummen.in/2016/09/02/debugging-rust-on-windows-using-visual-studio-code/).
* [Rust plugin for IntelliJ IDEA](https://intellij-rust.github.io/) is under active development. This plugin has a lot of traction and is turning around new versions on a nearly weekly basis.  Offers syntax highlighting, autocomplete \(via built-in parser\), cargo builts and eventually other functionality. [IntelliJ](https://www.jetbrains.com/idea/download/#section=windows) is a commercial product but it comes in a community edition which is sufficient for development.
* [Visual Rust plugin for Microsoft Studio](https://github.com/PistonDevelopers/VisualRust) . Offers syntax highlighting, autocompletion, interactive debugging.
* [RustDT for Eclipse](https://github.com/RustDT/RustDT) is also under active development. It adds syntax highlighting, autocomplete \(via racer\), cargo builds and rustfmt functionality to Eclipse.
* Atom is a popular editor with heaps of plugins. These plugins are very useful for Rust:
  * [language-rust](https://atom.io/packages/language-rust) provides basic syntax highlighting
  * [racer](https://atom.io/packages/racer) for autocompletion functionality
  * [atom-beautify](https://atom.io/packages/atom-beautify) invokes rustfmt to make code look pretty.
  * [build-cargo](https://atom.io/packages/build-cargo) invokes cargo for you showing errors and warnings inline.

For other editors and IDEs refer to the [Rust and IDEs](https://forge.rust-lang.org/ides.html) page on the Rust website.

## Racer / Rustfmt

Some of the plugins above make use of Racer and Rustfmt.

Racer is used by some plugins to provide autocompletion functionality.

Rustfmt is a source code formatting tool that makes sure your Rust source code is pretty to look at, adding spacing, indentation and so on.

You can get both just by typing these commands and waiting for the tools to download and build themselves - they're written in Rust and built through cargo.

```
cargo install racer
cargo install rustfmt
```
