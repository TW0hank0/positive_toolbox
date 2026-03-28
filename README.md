# positive_toolbox

![icon](./icon.png)

**中文** | [English](./READMEs/README_en.md)

最後更改：

2026-2-16

使用rust和iced製作，面向使用者的工具。

### 編譯

執行：

```
uv venv
uv sync --all-extras
uv run build_script.py
```

#### WASM (實驗性功能) (開發中)

需要：wasm-pack

```
wasm-pack build --target web --out-dir ./pkg
```

### 協議

版權所有 (C) 2026 TW0hank0

本程式基於 GNU Affero General Public License v3 授權

第三方專案授權見：

- [ThirdPartyLicense-Rust.html](./auto_generated/ThirdPartyLicense-Rust.html)

- [ThirdPartyLicense-Python.html](./ThirdPartyLicense-Python.html)

- [ThirdPartyLicense-Rust.md](./auto_generated/ThirdPartyLicense-Rust.md)
