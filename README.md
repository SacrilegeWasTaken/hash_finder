
# Setup Rust Environment: Ensure Rust and Cargo are installed on your system.

***Clone the Repository:***
```sh
git clone [your-repository-url]
cd hash_finder
```

***Build the Program:***
```sh
cargo build --release
```

***Run the Program:***
```sh
./target/release/hash_finder -n 3 -f 6
```

***Or for the run with release flag:***
```sh
cargo run --release -- -n 3 -f 6
```

# Flag Explanation:
- The -n flag specifies how many trailing zeros the hash should have.
- The -f flag specifies how many results to find before stopping.
