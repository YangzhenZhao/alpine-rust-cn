### Usage

#### ubuntu

**Build**

```
cd ubuntu
docker build -t muslrust-cn .
```

**Run Test**

```
cd ubuntu-test
sh test.sh
```

#### alpine

**Build**

```
cd alpine
docker build -t alpine-rust-cn .
```

**Run Test**

```
cd alpine-test

cd helloworld
cargo clean
docker run -it -v ${PWD}:/volume/ --rm alpine-rust-cn cargo run --release
docker run -it -v ${PWD}:/volume/ --rm alpine-rust-cn cargo run --release --target x86_64-unknown-linux-musl
```

**注意**: 使用 alpine-rust-cn 编译出来的可执行文件经常会出问题, 请**谨慎**使用

### References

<a href="https://github.com/clux/muslrust" target="_blank">https://github.com/clux/muslrust</a>   
<a href="https://wiki.ubuntu.com/Releases" target="_blank">https://wiki.ubuntu.com/Releases</a>      
<a href="https://github.com/emk/rust-musl-builder" target="_blank">https://github.com/emk/rust-musl-builder</a>      
