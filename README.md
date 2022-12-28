# About Hier Language
Hier is my second attempt at making a programming language (previous was GoldByte, but it had bad architecture, so it was easier to start from the  beginning). This time I decided to do actual research (Thanks for Crafting Interpreters). It runs code by interpreting AST, so it is not very fast, but it currently works and writing a bytecode VM is a possibility. There are issues with it, and it is not perfect, but with time it will improve.

# Usage
Hier is written in Rust, so you will need to install its toolchain, if you don't have it. Go to www.rust-lang.org/learn/get-started for help.
Clone this repo to your machine:
```
git clone https://github.com/wiktorwojcik112/hier.git
```
Enter hier directory and run this to build an executable:
```
cargo build --release
```
The executable will appear in target/release. Go into this directory to run it.
You can run REPL by entering:
```
./hier repl
```
You can run a Hier code from command line and print its result using:
```
./hier run "(print 123)"
```
To run a file, enter:
```
./hier file some_hier_code.hier
```

# About language
Hier is divided into 2 crates: library (libhier) , where language is implemented, and executable (hier), which is a CLI tool for running it.
If you want to learn more about the language, you can check the [libhier](https://github.com/wiktorwojcik112/libhier).