## installation

### Dependencies

```shell
1. sudo apt-get update
2. sudo apt-get install -y build-essential pkg-config libudev-dev llvm libclang-dev protobuf-compiler libssl-dev
```

如果报错：

下列软件包有未满足的依赖关系：
libudev-dev : 依赖: libudev1 (= 249.11-0ubuntu3.7) 但是 249.11-0ubuntu3.12 正要被安装
E: 无法修正错误，因为您要求某些软件包保持现状，就是它们破坏了软件包间的依赖关系。

那么直接把 libudev-dev 这个移除即可。

在下面的安装过程中，推荐指定版本安装，以免不必要的麻烦。版本汇总：
```shell
liujianyu@Ubuntu:~/vscode$ rustc --version
  rustc 1.78.0 (9b00956e5 2024-04-29)

liujianyu@Ubuntu:~/vscode$ cargo --version
  cargo 1.78.0 (54d8815d0 2024-03-26)

liujianyu@Ubuntu:~/vscode$ solana --version
  solana-cli 2.0.21 (src:99ac0105; feat:607245837, client:Agave)

liujianyu@Ubuntu:~/vscode$ avm --version
  avm 0.30.0

liujianyu@Ubuntu:~/vscode$ anchor --version
  anchor-cli 0.30.1
```

### RUST

```shell
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
1. curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.78.0   # 安装指定版本
2. . "$HOME/.cargo/env"
3. rustc --version
```

### SOLANA CLI

```shell
1. sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
1. sh -c "$(curl -sSfL https://release.anza.xyz/v2.0.21/install)"   # 安装指定版本
2. export PATH="/home/liujianyu/.local/share/solana/install/active_release/bin:$PATH"
3. solana --version
```

### ANCHOR CLI
Anchor 版本管理工具(Anchor Version Manager) avm，如果你熟悉 Nodejs，他就像管理 nodejs 版本的 nvm

```shell
1. cargo install --git https://github.com/coral-xyz/anchor avm --force
1. cargo install --git https://github.com/coral-xyz/anchor avm --tag v0.30.0 --force  # 安装指定版本

2. avm --version

3. avm install latest         
3. avm install 0.30.1         # 安装指定版本
4. avm use latest             # avm use 0.30.1

5. anchor --version
```

## config

```shell
liujianyu@Ubuntu:~$ solana config get
  Config File: /home/liujianyu/.config/solana/cli/config.yml
  RPC URL: https://api.mainnet-beta.solana.com                           # 主网URL
  WebSocket URL: wss://api.mainnet-beta.solana.com/ (computed)
  Keypair Path: /home/liujianyu/.config/solana/id.json 
  Commitment: confirmed 
```

设置测试网的URL

```shell
liujianyu@Ubuntu:~$ solana config set --url https://api.devnet.solana.com
  Config File: /home/liujianyu/.config/solana/cli/config.yml
  RPC URL: https://api.devnet.solana.com                                        # 测试网URL
  WebSocket URL: wss://api.devnet.solana.com/ (computed)
  Keypair Path: /home/liujianyu/.config/solana/id.json 
  Commitment: confirmed 
```

## basic command

### 生成测试钱包

```shell
liujianyu@Ubuntu:~$ solana address
  Error: No default signer found, run "solana-keygen new -o /home/liujianyu/.config/solana/id.json" to create a new one

liujianyu@Ubuntu:~$ solana-keygen new -o /home/liujianyu/.config/solana/id.json
  Wrote new keypair to /home/liujianyu/.config/solana/id.json
  ==============================================================================
  pubkey: 96NjZXgj5xx72miy6Vqaqo6KtLfnvwEiEXoaN83f8NAm
  ==============================================================================
  Save this seed phrase and your BIP39 passphrase to recover your new keypair:
  east ribbon fame guide begin kidney relax shoulder nothing step program peanut
  ==============================================================================
```

### 测试钱包

```shell
liujianyu@Ubuntu:~$ solana address
  96NjZXgj5xx72miy6Vqaqo6KtLfnvwEiEXoaN83f8NAm

liujianyu@Ubuntu:~$ solana balance
  0 SOL
```

## TEST ACCOUNT
grocery catalog wreck million staff victory trust antique live fine yard twelve


### 领取水龙头
```
https://faucet.solana.com             # for sol
https://spl-token-faucet.com/         # for spl token
```

```shell
liujianyu@Ubuntu:~$ solana balance
  5 SOL
```

## 区块链浏览器
https://explorer.solana.com/
https://solscan.io/



## ANCHOR

完整的anchor.toml配置文件中可配置内容见网址：https://www.anchor-lang.com/docs/manifest

```shell
1. anchor init my-project               # 新建一个项目模板, 包含了 demo 代码
2. anchor build                         # 编译项目
3. anchor test                          # 执行程序的测试套件
4. anchor deploy                        # 部署项目
4.1 anchor deploy --env devnet          # 部署到开发测试网
4.2 anchor deploy --env mainnet-beta    # 部署到主网
```

### 项目结构
```
my_project/
├── Anchor.toml
├── programs/
│   └── my_program/
│       ├── Cargo.toml
│       ├── src/
│       │   └── lib.rs
│       └── tests/
│           └── program_test.rs
├── target/
└── tests/
    └── integration_test.rs
```
这是一个简化的结构，提供了一个基本的框架，使你能够开始编写、测试和部署程序
● Anchor.toml： 项目的配置文件，包含项目的基本信息、依赖关系和其他配置项。
● programs目录： 包含你的程序的目录。在这个例子中，有一个名为my_program的子目录。
● Cargo.toml： 程序的Rust项目配置文件。
● src目录： 包含实际的程序代码文件，通常是lib.rs，在实际的项目中我们会根据模块划分，拆的更细。
● tests目录： 包含用于测试程序的测试代码文件。
● target目录： 包含构建和编译生成的文件。
● tests目录： 包含整合测试代码文件，用于测试整个项目的集成性能。






修改 Anchor.toml 配置文件
1. cluster = "Localnet"  --->
