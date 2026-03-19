<h1 align="center">axvisor_api</h1>

<p align="center">面向 ArceOS Hypervisor 组件的基础 API</p>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/axvisor_api.svg)](https://crates.io/crates/axvisor_api)
[![Docs.rs](https://docs.rs/axvisor_api/badge.svg)](https://docs.rs/axvisor_api)
[![Rust](https://img.shields.io/badge/edition-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-GPL--3.0--or--later%20OR%20Apache--2.0%20OR%20MulanPSL--2.0-blue.svg)](https://github.com/arceos-hypervisor/axvisor_api/blob/main/LICENSE)

</div>

[English](README.md) | 中文

# Introduction

`axvisor_api` 为运行在 ArceOS 上的 AxVisor 组件提供统一的 API 层。它用由过程宏生成的函数式接口替代基于泛型的 API 注入方式，使底层 hypervisor 组件能够访问 host、内存、时间、体系结构和 VMM 服务，而无需直接依赖底层内核实现。

该库导出两个核心过程宏：

- **`api_def`** - 定义 API trait，并生成可直接调用的包装函数
- **`api_impl`** - 为 API trait 注册实现

同时还暴露五个主要 API 模块：

- **`memory`** - 页帧分配、地址转换以及 `PhysFrame`
- **`time`** - tick/时间转换与定时器注册接口
- **`vmm`** - VM/vCPU 上下文查询与中断注入接口
- **`host`** - 宿主侧辅助 API
- **`arch`** - 体系结构相关辅助 API

## Quick Start

### Requirements

- Rust nightly 工具链
- Rust 组件：rust-src、clippy、rustfmt

```bash
# 安装 rustup（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 nightly 工具链与所需组件
rustup install nightly
rustup component add rust-src clippy rustfmt --toolchain nightly
```

### Run Check and Test

```bash
# 1. 进入仓库目录
cd axvisor_api

# 2. 代码检查
./scripts/check.sh

# 3. 运行测试
./scripts/test.sh
```

## Integration

### Installation

将以下依赖加入 `Cargo.toml`：

```toml
[dependencies]
axvisor_api = "0.3.0"
```

### Example

```rust
use axvisor_api::__priv;

pub mod some_demo {
    use memory_addr::MemoryAddr;
    pub use memory_addr::PhysAddr;

    #[axvisor_api::api_def]
    pub trait SomeDemoIf {
        fn some_func() -> PhysAddr;
        fn another_func(addr: PhysAddr);
    }

    pub fn provided_func() -> PhysAddr {
        some_func().add(0x1000)
    }
}

struct SomeDemoImpl;

#[axvisor_api::api_impl]
impl some_demo::SomeDemoIf for SomeDemoImpl {
    fn some_func() -> memory_addr::PhysAddr {
        memory_addr::pa!(0x42)
    }

    fn another_func(addr: memory_addr::PhysAddr) {
        let _ = addr;
    }
}

fn main() {
    let addr = some_demo::some_func();
    some_demo::another_func(addr);
    let next = some_demo::provided_func();
    assert_eq!(next.as_usize(), 0x1042);
}
```

### Documentation

生成并查看 API 文档：

```bash
cargo doc --no-deps --open
```

在线文档： [docs.rs/axvisor_api](https://docs.rs/axvisor_api)

# Contributing

1. Fork 仓库并创建分支
2. 本地运行检查：`./scripts/check.sh`
3. 本地运行测试：`./scripts/test.sh`
4. 提交 PR 并通过 CI 检查

# License

本项目采用 GPL-3.0-or-later OR Apache-2.0 OR MulanPSL-2.0 多重许可发布。详见 [LICENSE](LICENSE)。
