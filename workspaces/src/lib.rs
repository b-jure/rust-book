//Cargo Workspaces

/*
In Chapter 12, we built a package that included a binary crate and a library crate. 
As your project develops, you might find that the library crate continues to get bigger and you want to split up your package further into multiple library crates. 
In this situation, Cargo offers a feature called workspaces that can help manage multiple related packages that are developed in tandem.


Creating a Workspace
A workspace is a set of packages that share the same Cargo.lock and output directory. 
Let’s make a project using a workspace—we’ll use trivial code so we can concentrate on the structure of the workspace.

There are multiple ways to structure a workspace; we’re going to show one common way. 
We’ll have a workspace containing a binary and two libraries. The binary, which will provide the main functionality, will depend on the two libraries. 
One library will provide an add_one function, and a second library an add_two function. These three crates will be part of the same workspace. 
We’ll start by creating a new directory for the workspace:

    $ mkdir add
    $ cd add

Next, in the add directory, we create the Cargo.toml file that will configure the entire workspace. 
This file won’t have a [package] section or the metadata we’ve seen in other Cargo.toml files. 
Instead, it will start with a [workspace] section that will allow us to add members to the workspace by specifying the path to the package with our binary crate; 
in this case, that path is adder:

    [workspace]

    members = [
        "adder",
    ]

Next, we’ll create the adder binary crate by running cargo new within the add directory:

    $ cargo new adder
        Created binary (application) `adder` package

At this point, we can build the workspace by running cargo build. The files in your add directory should look like this:

    ├── Cargo.lock
    ├── Cargo.toml
    ├── adder
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── target

The workspace has one target directory at the top level for the compiled artifacts to be placed into; the adder package doesn’t have its own target directory. 
Even if we were to run cargo build from inside the adder directory, the compiled artifacts would still end up in add/target rather than add/adder/target. 
Cargo structures the target directory in a workspace like this because the crates in a workspace are meant to depend on each other. 
If each crate had its own target directory, each crate would have to recompile each of the other crates in the workspace to have the artifacts in its own target directory. 
By sharing one target directory, the crates can avoid unnecessary rebuilding.
*/


//Creating the Second Package in the Workspace

/*
Next, let’s create another member package in the workspace and call it add_one. Change the top-level Cargo.toml to specify the add_one path in the members list:

    [workspace]

    members = [
        "adder",
        "add_one",
    ]



Then generate a new library crate named add_one:

    $ cargo new add_one --lib
        Created library `add_one` package



Your add directory should now have these directories and files:

    ├── Cargo.lock
    ├── Cargo.toml
    ├── add_one
    │   ├── Cargo.toml
    │   └── src
    │       └── lib.rs
    ├── adder
    │   ├── Cargo.toml
    │   └── src
    │       └── main.rs
    └── target



In the add_one/src/lib.rs file, let’s add an add_one function:
Now that we have another package in the workspace, we can have the adder package with our binary depend on the add_one package, that has our library. 
First, we’ll need to add a path dependency on add_one to adder/Cargo.toml.

    [dependencies]
    add_one = { path = "../add_one" }

Cargo doesn’t assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships between the crates.



Next, let’s use the add_one function from the add_one crate in the adder crate. 
Open the adder/src/main.rs file and add a use line at the top to bring the new add_one library crate into scope. 
Then change the main function to call the add_one function.

    use add_one;

    fn main() {
        let num = 10;
        println!(
            "Hello, world! {} plus one is {}!",
            num,
            add_one::add_one(num)
        );
    }



Let’s build the workspace by running cargo build in the top-level add directory!

    $ cargo build
    Compiling add_one v0.1.0 (file:///projects/add/add_one)
    Compiling adder v0.1.0 (file:///projects/add/adder)
        Finished dev [unoptimized + debuginfo] target(s) in 0.68s



To run the binary crate from the add directory, we can specify which package in the workspace we want to run by using the -p argument and the package name with cargo run:

    $ cargo run -p adder
        Finished dev [unoptimized + debuginfo] target(s) in 0.0s
        Running `target/debug/adder`
    Hello, world! 10 plus one is 11!

This runs the code in adder/src/main.rs, which depends on the add_one crate.
*/


//Depending on an External Package in a Workspace

/*

Notice that the workspace has only one Cargo.lock file at the top level of the workspace rather than having a Cargo.lock in each crate’s directory. 
This ensures that all crates are using the same version of all dependencies. 
If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml files, 
Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock. 
Making all crates in the workspace use the same dependencies means the crates in the workspace will always be compatible with each other. 
Let’s add the rand crate to the [dependencies] section in the add_one/Cargo.toml file to be able to use the rand crate in the add_one crate:
Filename: add_one/Cargo.toml

    [dependencies]
    rand = "0.8.3"

We can now add use rand; to the add_one/src/lib.rs file, and building the whole workspace by running cargo build in the add directory will bring in and compile the rand crate. 
We will get one warning because we aren’t referring to the rand we brought into scope:

    $ cargo build
        Updating crates.io index
    Downloaded rand v0.8.3
    --snip--
    Compiling rand v0.8.3
    Compiling add_one v0.1.0 (file:///projects/add/add_one)
    warning: unused import: `rand`
    --> add_one/src/lib.rs:1:5
    |
    1 | use rand;
    |     ^^^^
    |
    = note: `#[warn(unused_imports)]` on by default

    warning: 1 warning emitted

    Compiling adder v0.1.0 (file:///projects/add/adder)
        Finished dev [unoptimized + debuginfo] target(s) in 10.18s


The top-level Cargo.lock now contains information about the dependency of add_one on rand. 
However, even though rand is used somewhere in the workspace, we can’t use it in other crates in the workspace unless we add rand to their Cargo.toml files as well. 
For example, if we add use rand; to the adder/src/main.rs file for the adder package, we’ll get an error:

    $ cargo build
    --snip--
    Compiling adder v0.1.0 (file:///projects/add/adder)
    error[E0432]: unresolved import `rand`
    --> adder/src/main.rs:2:5
    |
    2 | use rand;
    |     ^^^^ no external crate `rand`


To fix this, edit the Cargo.toml file for the adder package and indicate that rand is a dependency for it as well. 
Building the adder package will add rand to the list of dependencies for adder in Cargo.lock, but no additional copies of rand will be downloaded. 
Cargo has ensured that every crate in every package in the workspace using the rand package will be using the same version. 
Using the same version of rand across the workspace saves space because we won’t have multiple copies and ensures 
that the crates in the workspace will be compatible with each other.
*/


//Adding a Test to a Workspace

/*

For another enhancement, let’s add a test of the add_one::add_one function within the add_one crate:

    pub fn add_one(x: i32) -> i32 {
        x + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            assert_eq!(3, add_one(2));
        }
    }


Now run cargo test in the top-level add directory:

    $ cargo test
    Compiling add_one v0.1.0 (file:///projects/add/add_one)
    Compiling adder v0.1.0 (file:///projects/add/adder)
        Finished test [unoptimized + debuginfo] target(s) in 0.27s
        Running target/debug/deps/add_one-f0253159197f7841

    running 1 test
    test tests::it_works ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

        Running target/debug/deps/adder-49979ff40686fa8e

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Doc-tests add_one

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s



Running cargo test in a workspace structured like this one will run the tests for all the crates in the workspace.

We can also run tests for one particular crate in a workspace from the top-level directory by using the -p flag and specifying the name of the crate we want to test:

    $ cargo test -p add_one
        Finished test [unoptimized + debuginfo] target(s) in 0.00s
        Running target/debug/deps/add_one-b3235fea9a156f74

    running 1 test
    test tests::it_works ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Doc-tests add_one

    running 0 tests

    test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


This output shows cargo test only ran the tests for the add_one crate and didn’t run the adder crate tests.

If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately. 
The cargo publish command does not have an --all flag or a -p flag, so you must change to each crate’s directory and run cargo 
publish on each crate in the workspace to publish the crates.
*/



//Installing Binaries from Crates.io with cargo install

/*
The cargo install command allows you to install and use binary crates locally. 
This isn’t intended to replace system packages; it’s meant to be a convenient way for Rust developers to install tools that others have shared on crates.io.

Note that you can only install packages that have binary targets. 
A binary target is the runnable program that is created if the crate has a src/main.rs file or another file specified as a binary, 
as opposed to a library target that isn’t runnable on its own but is suitable for including within other programs.

Usually, crates have information in the README file about whether a crate is a library, has a binary target, or both.

All binaries installed with cargo install are stored in the installation root’s bin folder. 
If you installed Rust using rustup.rs and don’t have any custom configurations, this directory will be $HOME/.cargo/bin. 
Ensure that directory is in your $PATH to be able to run programs you’ve installed with cargo install.

For example, in Chapter 12 we mentioned that there’s a Rust implementation of the grep tool called ripgrep for searching files. 
If we want to install ripgrep, we can run the following:

    $ cargo install ripgrep
        Updating crates.io index
    Downloaded ripgrep v11.0.2
    Downloaded 1 crate (243.3 KB) in 0.88s
    Installing ripgrep v11.0.2
    --snip--
    Compiling ripgrep v11.0.2
        Finished release [optimized + debuginfo] target(s) in 3m 10s
    Installing ~/.cargo/bin/rg
    Installed package `ripgrep v11.0.2` (executable `rg`)

The second-to-last line of the output shows the location and the name of the installed binary, which in the case of ripgrep is rg. 
As long as the installation directory is in your $PATH, as mentioned previously, you can then run rg --help and start using a faster, rustier tool for searching files!
*/


//Extending Cargo with Custom Commands

/*
Cargo is designed so you can extend it with new subcommands without having to modify Cargo. 
If a binary in your $PATH is named cargo-something, you can run it as if it was a Cargo subcommand by running cargo something.

Custom commands like this are also listed when you run cargo --list. 
Being able to use cargo install to install extensions and then run them just like the built-in Cargo tools is a super convenient benefit of Cargo’s design!
*/