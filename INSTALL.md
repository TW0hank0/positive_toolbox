# 安裝positive_toolbox

**安裝positive_toolbox的要求**

系統：linux、windows
儲存空間：200MB~250MB
可用記憶體：500MB或更多

有兩種安裝方法。

### 從Release安裝 (推薦)

安裝步驟
1. 從release下載.zip壓縮檔
2. 解壓縮。
3. 進入資料夾
4. 執行 `positive_toolbox`

優點：方便、簡單

缺點：無法立即體驗最新功能

### 原始碼編譯

需要安裝rust工具鏈及`cargo-about`工具

依序執行：

```
git clone https://github.com/TW0hank0/positive_toolbox.git
cd positive_toolbox
cargo build --release
cargo run --release
```

優點：可立即體驗最新功能

缺點：需安裝rust工具鏈
