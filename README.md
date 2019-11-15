### dev
install [rustup](http://rustup.rs)
```
cargo build
cargo run
```

### deploy
```docker
docker build -t nicked .
docker run -p 6000:6000 --rm --init nicked
```