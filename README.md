# Usage

Using httpie I want a 400 response that takes 2 seconds

`http :6000/test\?statuscode=400\&wait=200`

Using curl I want a 200 response that takes 5 seconds

`curl localhost:6000/test\?statuscode=200\&wait=5000`


### dev
install [rustup](http://rustup.rs)
```
cargo build
cargo run
```

file watcher mode

`catflap -- cargo watch -x run`

### deploy
```docker
docker build -t nicked .
docker run -p 6000:6000 --rm --init nicked
```