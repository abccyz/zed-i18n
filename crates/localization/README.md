# Localization

该 crate 为 Zed 编辑器提供基础的国际化支持：

- 负责读取 `settings` 中的界面语言首选项（默认简体中文）。
- 暴露 `translate_static`、`translate_owned` 与 `shared` 三个助手，用于在 UI 代码中获取翻译文本。
- 在应用启动时通过 `localization::init(cx)` 注册设置监听并保持全局语言状态。

## 使用方式

```
use localization::shared;

let label = Label::new(shared("command.apply_all", "Apply All"));
```

如需新增文本，只需：

1. 在 `crates/localization/src/lib.rs` 的 `ZH_CN_ENTRIES` 中注册键值对。
2. 在调用处使用相同的键并提供英文默认值。

若未来需要支持更多语言，可在 `Language` 枚举和 `ZH_CN_ENTRIES` 旁新增对应的翻译映射及设置序列化枚举值。
