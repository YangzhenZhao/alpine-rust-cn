### Build

```
docker build -t alpine-rust-cn .
```

### Run Sample

```
cd ~/Desktop
cargo new myapp
docker run -it -v ~/Desktop/myapp/:/volume/ --rm alpine-rust-cn cargo run
```

输出结果:

```
Hello, world!
```
