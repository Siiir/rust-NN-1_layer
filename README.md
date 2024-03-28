## Download
1. `git clone https://github.com/Siiir/rust-perceptron`
   
## Run
3. `cd rust-perceptron`
4. `cat ./data/unclassified_irises.csv | cargo run --release`
### Above approach requires
1. `cargo` that is usually installed with [rustup](https://www.rust-lang.org/tools/install)

## Help (passing arguments to app)
You can also pass arguments to the app after --, which is cargo's way to separate cargo args from app args.  
Try: `cargo r -r -- --help`
