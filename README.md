<h4 align="right"><a href="https://github.com/tw93/Pake">上游仓库</a></h4>
<p align="center">
    <img src=https://gw.alipayobjects.com/zos/k/fa/logo-modified.png width=138/>
</p>
<h1 align="center">Pake</h1>
<p align="center"><strong>一键打包网页生成轻量桌面应用，支持 macOS、Windows 和 Linux</strong></p>

## 特征

- 🎐 **体积小巧**：相比 Electron 应用小近 20 倍，通常只有 5M 左右
- 🚀 **性能优异**：基于 Rust Tauri，比传统 JS 框架更快，内存占用更少
- ⚡ **使用简单**：命令行一键打包，无需复杂配置
- 📦 **功能丰富**：支持快捷键透传、沉浸式窗口、拖拽、样式定制、去广告
- 🌐 **原生浏览器模式**：`--native` 模式使用本地 Chrome 以 `--app` 方式启动，绕过 WebView 指纹检测

## 快速开始

- **开发者**：安装 [CLI 工具](docs/cli-usage_CN.md) 后一行命令打包任意网站
- **高级用户**：本地克隆项目进行 [定制开发](#定制开发)，或查看 [高级用法](docs/advanced-usage_CN.md)
- **遇到问题**：查看 [常见问题](docs/faq_CN.md)

## 命令行一键打包

```bash
# 安装 Pake CLI
pnpm install -g pake-cli

# 基础用法（WebView 模式）- 自动获取网站图标
pake https://github.com --name GitHub

# 高级用法：自定义选项
pake https://weekly.tw93.fun --name Weekly --icon https://cdn.tw93.fun/pake/weekly.icns --width 1200 --height 800 --hide-title-bar
```

首次打包需要安装环境会比较慢，后续很快。完整参数说明查看 [CLI 使用指南](docs/cli-usage_CN.md)。

## 原生浏览器模式（--native）

部分网站（如小红书）会检测 WebView 指纹并拦截访问。`--native` 模式使用本地安装的 Chrome 浏览器以 `--app` 方式启动，拥有真实的浏览器指纹，可以绕过这类检测。

```bash
# 使用 --native 模式打包
pake https://www.xiaohongshu.com/explore --native --name XiaoHongShu

# 产物是一个独立的轻量二进制（~500KB），运行时调用本地 Chrome
```

### WebView 模式 vs Native 模式

| | WebView 模式（默认） | Native 模式（--native） |
|---|---|---|
| 引擎 | 系统 WebView（WebKit/WebView2） | 本地 Chrome `--app` |
| 产物大小 | ~5MB | ~500KB |
| 浏览器指纹 | WebView 指纹，可能被检测 | 真实 Chrome 指纹 |
| 依赖 | 无额外依赖 | 需要安装 Chrome |
| 功能 | 完整（托盘、快捷键、菜单等） | 基础（窗口启动） |

### 数据目录

两种模式使用独立的数据目录，互不干扰：

```
~/.config/{AppName}/webview/     ← WebView 模式
~/.config/{AppName}/chrome/      ← Native 模式
```

### 注意事项

- 仅支持 Chrome（不支持 Edge/Firefox）
- 如果未安装 Chrome，运行时会提示错误并以退出码 127 退出
- macOS 上会自动生成 `.app` 包，Windows/Linux 生成独立可执行文件

## 定制开发

需要 Rust `>=1.85` 和 Node `>=22`，详细安装指南参考 [Tauri 文档](https://tauri.app/start/prerequisites/)。

```bash
# 安装依赖
pnpm i

# 本地开发[右键可打开调试模式]
pnpm run dev

# 打包应用
pnpm run build
```

想要样式定制、功能增强、容器通信等高级玩法，查看 [高级用法文档](docs/advanced-usage_CN.md)。

<details>
<summary>🏂 <b>快捷键说明</b></summary>

<br/>

| Mac                                                       | Windows/Linux                                       | 功能                |
| --------------------------------------------------------- | --------------------------------------------------- | ------------------- |
| <kbd>⌘</kbd> + <kbd>[</kbd>                               | <kbd>Ctrl</kbd> + <kbd>←</kbd>                      | 返回上一个页面      |
| <kbd>⌘</kbd> + <kbd>]</kbd>                               | <kbd>Ctrl</kbd> + <kbd>→</kbd>                      | 去下一个页面        |
| <kbd>⌘</kbd> + <kbd>↑</kbd>                               | <kbd>Ctrl</kbd> + <kbd>↑</kbd>                      | 自动滚动到页面顶部  |
| <kbd>⌘</kbd> + <kbd>↓</kbd>                               | <kbd>Ctrl</kbd> + <kbd>↓</kbd>                      | 自动滚动到页面底部  |
| <kbd>⌘</kbd> + <kbd>r</kbd>                               | <kbd>Ctrl</kbd> + <kbd>r</kbd>                      | 刷新页面            |
| <kbd>⌘</kbd> + <kbd>w</kbd>                               | <kbd>Ctrl</kbd> + <kbd>w</kbd>                      | 隐藏窗口,非退出     |
| <kbd>⌘</kbd> + <kbd>-</kbd>                               | <kbd>Ctrl</kbd> + <kbd>-</kbd>                      | 缩小页面            |
| <kbd>⌘</kbd> + <kbd>=</kbd>                               | <kbd>Ctrl</kbd> + <kbd>=</kbd>                      | 放大页面            |
| <kbd>⌘</kbd> + <kbd>0</kbd>                               | <kbd>Ctrl</kbd> + <kbd>0</kbd>                      | 重置页面缩放        |
| <kbd>⌘</kbd> + <kbd>L</kbd>                               | <kbd>Ctrl</kbd> + <kbd>L</kbd>                      | 复制当前页面网址    |
| <kbd>⌘</kbd> + <kbd>⇧</kbd> + <kbd>⌥</kbd> + <kbd>V</kbd> | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd>   | 粘贴并匹配样式      |
| <kbd>⌘</kbd> + <kbd>⇧</kbd> + <kbd>H</kbd>                | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>H</kbd>   | 回到首页            |
| <kbd>⌘</kbd> + <kbd>⌥</kbd> + <kbd>I</kbd>                | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>I</kbd>   | 开启调试 (仅开发版) |
| <kbd>⌘</kbd> + <kbd>⇧</kbd> + <kbd>⌫</kbd>                | <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>Del</kbd> | 清除缓存并重启      |

此外还支持双击头部全屏切换，拖拽头部移动窗口，Mac 用户支持手势返回和前进。

</details>
