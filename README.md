# Substrate Node Template

Cambio DNS 测试版

## Usage

1. 编译:
```bash
cargo build --release

```
2. 生成ChainSpec

```bash
./target/release/node-template build-spec > dns-test-chain.json 

```

3. 生成`raw chain spec` 

```bash

./target/release/node-template build-spec --chain=dns-test-chain.json --raw > dns-raw-chain-spec.json



```
