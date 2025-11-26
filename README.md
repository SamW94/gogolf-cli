# gogolf-cli

⛳ Welcome to the code repository for gogolf's CLI tool! gogolf is a toy project I've created to demonstrate understanding of full-stack software development, DevOps and having a *crazy* high handicap. Having said that, I will hopefully get it up-and-running as a web app at some point.

You're currently looking at the code for the *CLI tool* - that's the command line tool that is developed in parallel with the API which *will hopefully at some point be able to*:

- create unique golfers in the gogolf database
- add clubs to a golfer's bag and their yardages for that golfer
- add courses to the gogolf database, including their course ratings, slope, yardages and total par.
- calculate a golfer's handicap index, and their playing handicap for any course in the database
- do any other things I can dream up as I mindlessly top balls down the driving range

## 🏌️ Related Repositories (More Coming Soon™)

- [gogolf-api](https://github.com/SamW94/gogolf-api): gogolf's backend microservice/API
- [gogolf-web](https://github.com/SamW94/gogolf-web): the front-end


## 🛺 What's in the box? 

gogolf's CLI tool is written in Rust 🦀 - I just think it's cool. The asyncronous runtime used is [tokio](https://github.com/tokio-rs/tokio) while the HTTP client stuff is handled by [reqwest](https://github.com/seanmonstar/reqwest). A command reference can be found below. 

## 🚩 How do I run it?

### Pre-requisites

*N.B. the source code currently isn't compiled and packaged neatly for your to install. It will be in future.*

- These instructions are for Linux/Unix/WSL. I'm not writing a Windows guide, and you can't make me.
- You must have the [Rust toolchain](https://rust-lang.org/tools/install/) installed.
- You can run the CLI indepdently from the API, but it's a bit pointless, so you *should* have the [gogolf API running](https://rust-lang.org/tools/install/). 

### Using `cargo run`

1. From the root of this repository, run `cargo run` which will compile the application and run it. If successful, you'll see something like this:

    ```
    ❯ cargo run
   Compiling gogolf-cli v0.0.1 (/home/sam/github.com/SamW94/gogolf-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.76s
     Running `target/debug/gogolf-cli`
    Starting gogolf-cli...
    gogolf> 
    ```

2. Run commands (see the reference below) to do cool stuff.

## ❓ Command Reference

**Usage:**

- *help*: Displays this help message
- *golfer*: Commands for creating/finding/updating and deleting golfers

### help

Displays the help message on the command line, showing the above command reference.
```
gogolf> help
help - Displays this help message
golfer - Commands for creating/finding/updating and deleting golfers
```

### golfer

Commands for creating/finding/updating and deleting golfers
```
gogolf> golfer
No arguments supplied with the 'golfer' command. The 'gogolf golfer' command takes the following arguments:
create - Creates a golfer in the gogolf database
```

#### golfer create [username] [email address]

Creates a golfer in the gogolf database. Prompts the user for a password before sending the HTTP request to the API.

```
gogolf> golfer create test test@email.com
Type a password:
Golfer created successfully!
Server response: {"id":"62f1e229-e633-48e7-991c-5b8f7e345d00","created_at":"2025-11-23T10:09:34.448123Z","updated_at":"2025-11-23T10:09:34.448123Z","email":"test@email.com","username":"test"}
```

