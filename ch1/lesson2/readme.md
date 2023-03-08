# Lesson 2: Cargo
File for this lesson: <i>main.rs</i> under <i>./hello_cargo/src</i> and <i>./hello_cargo/target</i> folders

## What is Cargo?
Cargo is Rust's build system and package manager, similar to Gem locks in Ruby. After you have initiate a project with Cargo, you may develop your scripts, tests, and rollout to production.


## The General Development Work Flow
First, initiate your project by:

```
cargo new <project_name>
```
<br>
Once you have initiate your project, in the project folder with your project name, you will find <i>Cargo.toml</i>, which is the configuration file for your project. <i>TOML</i> is the format for Cargo's configuration.
<br><br>
Then, you may head to the <i>src</i> folder to find <i>main.rs</i>. This is where you will develop your code. Once you are done, you can build your project with the following command:

```
cargo build
```
<br>
<b>Be sure to run all cargo command in the directory where the TOML file is located</b>
<br><br>
This command creates an executable file in <i>./target/debug/\<project_name\></i> folder, then you can run the executable file by either of the following command:

```
# Run directly
./target/debug/<project_name>
# Run via Cargo
cargo run
```

<br>
You can also quickly checks your code without producing an executable, it speeds up the preocess to make sure the code compiles:

```
cargo check
```

<br>
When your project is finally finished and ready for release, you may run the following to rollout your code to production:

```
cargo build --release
```

<br><br>
Once it is release, it will release the code to the <i>./target/release</i> folder. And you may run the executable by either of the following command:

```
# Run directly
./target/release/<project_name>
# Run via Cargo
cargo run --release
```
## Example Script
In this lesson, the project with the project name <i>hello_cargo</i> has been initiated. You may build the project with the script saved in the <i>./hello_cargo/src</i> folder and rollout to production with the following command:

```
# Build the project
cargo build

# Run the code
./target/debug/hello_cargo

# Rollout to production
cargo build --release

# Run the code in production environment
./target/release/hello_cargo
```
