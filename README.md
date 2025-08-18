# Pinocchio Tapedrive

## Steps to start

### 1. clone the repo

```bash
git clone git@github.com:Turbin3/pinocchio-tapedrive.git
```

### 2. Build program

```bash
cargo build-sbf
```

### 3. Running Tests

```bash
cargo test --features test-default
```

### 4. Running Benchmarks

```bash
cargo bench --features bench-default
```

### 5. Client Generation

```bash
# Generate TypeScript client
./gen-client.sh typescript

# Generate Rust client
./gen-client.sh rust

```
