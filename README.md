# positive_toolbox

![icon](./icon.png)

**中文** | [English](./READMEs/README_en.md)

最後更改：

2026-2-16

使用rust和iced製作，面向使用者的工具。

### 編譯

#### 電腦

```
cargo build --release
```

#### WASM (實驗性功能) (開發中)

```
wasm-pack build --target web --out-dir ./pkg
```

### 執行

#### 電腦

```
cargo run --release
```

#### WASM (實驗性功能) (開發中)

使用Python

```
python -m http.server 10000
```

瀏覽器開啟：

```
http://localhost:10000
```

### 協議

版權所有 (C) 2026 TW0hank0

本程式基於 GNU Affero General Public License v3 授權

第三方專案見：

[ThirdPartyLicense-Rust.html](./ThirdPartyLicense-Rust.html)

[ThirdPartyLicense-Python.html](./ThirdPartyLicense-Python.html)
