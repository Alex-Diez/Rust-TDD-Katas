# TDD katas

Here I have different tdd katas written with [Rust](https://www.rust-lang.org/)
One day I realize that I can learn how to write difficult programs or modules
just rewriting it every day for half an hour.

Thanks to my [Team Leader](https://github.com/Dmitry404) for suggestion to write katas every day :D

# Run tests
cargo test **kata_name::day_#**

# Project structure

##1 Implementations of katas
```
src/
 |
 |- kata_1_name
 |- kata_2_name
 |   |-iter_1
 |   |  |-day_1.rs
 |   |  |-day_2.rs
 |   |  |-...
 |   |  --day_N.rs
 |   |
 |   |-iter_2
 |   |-...
 |   -- iter_N
 |
 |- ...
 -- kata_N_name
```
##2 Tests (notice that not all tests are passed)
```
test/
 |
 |- kata_1_name
 |- kata_2_name
 |   |-iter_1
 |   |  |-day_1.rs
 |   |  |-day_2.rs
 |   |  |-...
 |   |  --day_N.rs
 |   |
 |   |-iter_2
 |   |-...
 |   -- iter_N
 |
 |- ...
 -- kata_N_name
```

##3 Problems formalization (notice that no each kata has formalization)
```
formals/
 |
 |- kata_1_name
 |- kata_2_name
 |   |-iter_1
 |   |  |-day_1.rs
 |   |  |-day_2.rs
 |   |  |-...
 |   |  --day_N.rs
 |   |
 |   |-iter_2
 |   |-...
 |   -- iter_N
 |
 |- ...
 -- kata_N_name
```
