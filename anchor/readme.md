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
solana config get
  Config File: /home/liujianyu/.config/solana/cli/config.yml
  RPC URL: https://api.mainnet-beta.solana.com                           # 主网URL
  WebSocket URL: wss://api.mainnet-beta.solana.com/ (computed)
  Keypair Path: /home/liujianyu/.config/solana/id.json 
  Commitment: confirmed 
```

设置测试网的URL

```shell
solana config set --url https://api.devnet.solana.com
  Config File: /home/liujianyu/.config/solana/cli/config.yml
  RPC URL: https://api.devnet.solana.com                                        # 测试网URL
  WebSocket URL: wss://api.devnet.solana.com/ (computed)
  Keypair Path: /home/liujianyu/.config/solana/id.json 
  Commitment: confirmed 

```

设置 Solana CLI 使用本地节点

```
solana config set --url http://127.0.0.1:8899
solana-test-validator
```

设置第三方的RPC节点（由于网络问题，直接设置为devnet的话，将无法访问
```shell
solana config set --url https://young-restless-bridge.solana-devnet.quiknode.pro/64df14141046ed00f4320d627db7e1119aef0b52
  Config File: /home/liujianyu/.config/solana/cli/config.yml
  RPC URL: https://young-restless-bridge.solana-devnet.quiknode.pro/64df14141046ed00f4320d627db7e1119aef0b52 
  WebSocket URL: wss://young-restless-bridge.solana-devnet.quiknode.pro/64df14141046ed00f4320d627db7e1119aef0b52 (computed)
  Keypair Path: /home/liujianyu/.config/solana/id.json 
  Commitment: confirmed 
```

然后在 Anchor.toml 的配置文件中，修改 cluster 信息
Anchor.toml    provider
将 cluster 的值从 "Devnet" 改为你自定义的 RPC URL, 这样，Anchor 会使用你指定的 RPC 节点，而不是默认的 api.devnet.solana.com
```
[provider]
# cluster = "Devnet"                                    # 使用的网络环境(Localnet/Devnet/Mainnet)
cluster = "https://young-restless-bridge.solana-devnet.quiknode.pro/64df14141046ed00f4320d627db7e1119aef0b52"
wallet = "~/.config/solana/id.json"                     # 钱包密钥对文件路径
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
3. anchor clean                         # 清除target目录下已编译的内容
4. anchor test                          # 执行程序的测试套件

5. anchor deploy                        # 部署项目
5.1 anchor deploy --env devnet          # 部署到开发测试网
5.2 anchor deploy --env mainnet-beta    # 部署到主网
5.3 anchor deploy -p voting             # -p 只部署指定的合约文件, 比如这里在部署 voting 合约
5.4 anchor deploy -p voting --provider.cluster https://young-restless-bridge.solana-devnet.quiknode.pro/64df14141046ed00f4320d627db7e1119aef0b52  # 强制指定 RPC
```

### ANCHOR TEST
anchor test, 在 Anchor 中，默认情况下运行 anchor test 会部署工作区中的所有程序（合约），然后运行测试文件
如果你只想运行指定的测试文件（例如 tests/voting.ts），而不部署其他合约，可以通过以下方法实现：

使用 --skip-deploy 选项
anchor test 命令支持 --skip-deploy 选项，可以跳过部署步骤，直接运行测试文件。你需要确保合约已经部署到目标网络（例如 Devnet 或 Localnet），然后运行以下命令

--skip-deploy：跳过合约的重新部署，但仍会检查编译。
--skip-local-validator：不启动新的 solana-test-validator，直接连接到现有的本地测试链

```
anchor test tests/voting.ts --skip-deploy
anchor test tests/voting.ts --skip-deploy --skip-local-validator    # if using the local net

anchor test --skip-deploy
anchor test --skip-deploy --skip-local-validator    # if using the local net
```

pay attention: the above command will trigger all the .ts scripts in the tests folder, because there is a configuration in the Anchor.toml

```
[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
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


### TEST FRAMEWORK
yarn add anchor-bankrun


### PROGRAM ID
可以通过 anchor keys list 获取每个程序的 ID，并替换到 Anchor.toml 文件中
```
anchor keys list
  count_increment: H5FMw8u5VcRq28jEujYZ4tQJgUxy94kUVBLvrAYHkecc
  spl_token: 7VE1KT8XXssnwBcYA3eZbmi4ZCkVPWzQZam63ZDQfQQn
  guess_random_number: 9MHvXBo6NCYGj1t6LvswH9PBqapAPj9jviyUJkQATt6d
  voting: 6ghmiYfqXdugFBdkMruXdhH6qD4rFoL1z7KeuxWeVEYo
```

如果尚未为每个程序生成 Program ID，可以通过以下命令生成:
生成的密钥对会保存在对应的子目录下，例如 target/deploy/count_increment-keypair.json。然后在 Anchor.toml 中添加对应的 Program ID
```
anchor keys generate
```