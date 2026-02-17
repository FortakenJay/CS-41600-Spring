# Multilingual Algorithms Project

*insert project description here*

We are using Ubuntu 24.04 as our operating system.

## Git Setup

Clone this project with Git via HTTPS or SSH: 

HTTPS: `git clone https://github.com/FortakenJay/CS-41600-Spring.git`

SSH: `git clone git@github.com:FortakenJay/CS-41600-Spring.git`

## Language Setup

Below is the instructions on how to install the required programs for each language.
You may need to use `sudo apt update` to refresh the available packages list in apt.

### Java

Install the Java Runtime Environment (JRE): `sudo apt install default-jre`

Check whether the JRE was properly installed: `java -version`

It should output the following: 

```
openjdk version "21.0.10" 2026-01-20
OpenJDK Runtime Environment (build 21.0.10+7-Ubuntu-124.04)
OpenJDK 64-Bit Server VM (build 21.0.10+7-Ubuntu-124.04, mixed mode, sharing)
```

Install the Java Developer Kit (JDK) for Java 21: 
`sudo apt install openjdk-21-jdk-headless`

Check whether the JDK was properly installed: `javac -version`

It should print this line: `javac 21.0.10`

### Go

Use Snap to install Go v1.25: `sudo snap install go --channel=1.25/stable --classic`

To verify that Go was correctly installed, run: `go version`

You should get an output like: `go version go1.25.6 linux/amd64`

You can run a standalone Go program like this: `go run Go/Main.go`

### Julia

To install Julia on Ubuntu, simply run this shell command: 
`curl -fsSL https://install.julialang.org | sh`
The default configuration will likely work just fine.

> If you get a warning about the snap version of curl, you may need to use 
`sudo snap remove curl` and `sudo apt install curl` 
to replace it with the native version instead.

After installation, in any new shell process you can use the command `julia` 
to start an interactive session, or provide it with a script file to 
run it non-interactively.

### Python

Python should already be installed. Use the command `python3 --version` 
to check your version. Make sure you're using Python 3.12 or later.

### Rust

To install the rustup tool for Rust version management, use this command:
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Note: this might take a while!

> If you get a warning about the snap version of curl, you may need to use 
`sudo snap remove curl` and `sudo apt install curl` 
to replace it with the native version instead.

Use `rustc --version` to check that the installation worked correctly.

You can compile a standalone Rust file like this: `rustc Rust/Main.rs -o Rust/Main`

