# Another simple brainfuck interpreter (written in Rust)

## Run it through cargo
```
cargo run --release path/to/file.bf
```
## Independent executable
Build the executable with:
```
cargo build --release
```
Then run it directly with:
```
path/to/executable -- path/to/file.bg
```
You could also add the executable to `PATH` and access it from anywhere in your cli
### Path
Note that the path to the `.bg` file can also be relative
