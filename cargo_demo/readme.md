1. https://wendao76.blog.csdn.net/article/details/156536222
2. cargo run -- arg1 arg2  # 程序接收 args: ["arg1", "arg2"]
3. cargo check       # 快速检查语法和类型错误（比 build 快，不生成可执行文件）
4. 测试文件默认放在 src/lib.rs（库项目）或 tests/ 目录（集成测试）。
cargo test       # 运行所有测试（单元测试+集成测试）
cargo test test_name  # 运行指定名称的测试
cargo test -- --nocapture  # 显示测试中 print! 的输出
5. cargo clean       # 删除 target/ 目录（清理所有编译缓存）
6. 
# 添加依赖（自动更新 Cargo.toml）
cargo add serde --features "derive"  # 带特性添加
cargo add tokio --dev               # 添加为 dev-dependency

# 移除依赖
cargo rm serde

# 更新依赖到兼容版本（根据 Cargo.toml 的版本规则）
cargo update          # 更新所有依赖
cargo update -p serde  # 仅更新 serde

# 查看依赖树（排查冲突）
cargo tree            # 显示依赖层级
cargo tree -d          # 显示重复依赖（版本冲突时）

7.  Cargo 配置系统：自定义构建行为
8. 交叉编译
使用 cargo build --target <target> 编译到其他平台（需提前安装目标平台工具链）：
# 安装目标平台（如 ARM Linux）
rustup target add armv7-unknown-linux-gnueabihf

# 交叉编译
cargo build --target armv7-unknown-linux-gnueabihf --release
