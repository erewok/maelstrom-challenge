# Maelstrom Challenge

This repo is an attempt to solve all of the challenges from [Fly.io's Gossip Glimmers](https://fly.io/dist-sys/), described as "A series of distributed systems challenges".

## To Build

This is a rust project so `cargo build` will build it and `cargo run` will run it. For the moment, there is only a single argument provided to the binary, `--workload`. Try `--help` for more info:

```sh
Run a Maelstrom Challenge from Fly.io

Usage: maelstrom-challenge --workload <WORKLOAD>

Options:
  -w, --workload <WORKLOAD>  Name of the workload (challenge) to run [possible values: broadcast, echo, g-counter, g-set, kafka, lin-kv, pn-counter, txn-list-append, txn-rw-register, unique-i-ds]
  -h, --help                 Print help
  -V, --version              Print version
```

## To Run a Maelstrom Challenge

I run it with a shell script that includes the path to `cargo` and the location of this repo. The shell script looks like this:

```sh
#!/bin/bash
cd location/of/this/repo/maelstrom-challenge;
exec path/to/cargo run -- --workload ${WORKLOAD}

```

With the above, I can run `maelstrom` like this:

```sh
❯ ../maelstrom/maelstrom test -w echo --bin runner.sh --node-count 1 --time-limit 10
...

Everything looks good! ヽ(‘ー`)ノ
```