### rust实现裸机程序[基于riscv64]
- 1、创建一个project
```
    cargo new os
```

- 2、在项目中创建rust-toolchain.toml文件来规定需要的工具链, 写入内容

```
    [toolchain]
    profile = "minimal"
    channel = "nightly"
    components = ["rust-src", "rustfmt", "clippy"]
    targets = ["riscv64imac-unknown-none-elf"] # "x86_64-unknown-none", "riscv64gc-unknown-none-elf", "aarch64-unknown-none-softfloat"
```

- 3、创建.cargo目录以及在.cargo目录下创建 config 文件; 在这里主要设置使用的连接器脚本
```
# 编译的目标平台
[build]
target = "riscv64imac-unknown-none-elf"

# 使用我们的 linker script 来进行链接
[target.riscv64imac-unknown-none-elf]
rustflags = [
    "-C", "link-arg=-Tsrc/linker.ld",
]
```
