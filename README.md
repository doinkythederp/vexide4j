# vexide4j

[![Build status](https://github.com/doinkythederp/vexide4j/actions/workflows/build.yml/badge.svg)](https://github.com/doinkythederp/vexide4j/actions/workflows/build.yml)

> Proof-of-concept JDK running on VEX V5 robot

## How to get it running

If you can run a normal Vexide project, this will seem similar! Just make changes to the Java files in `src/main/java` or Rust files in `src/` and run `cargo pros upload --slot <N>`. Take a look at the Vexide template on the `vexide` GitHub if you need more help.

IntelliJ is probably a good choice for editing the Java files, but anything supporting Gradle should work.

## Binary sizes

The Java standard library is pretty big for a V5, so most of it is ommitted by default. If you run your program and it says you're missing a class, then add it to the list in `rt_dependencies.txt` and they will be copied from `lib/` into the binary uploaded to your robot so your Java program can access them.
