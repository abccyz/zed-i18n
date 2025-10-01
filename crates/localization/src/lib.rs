use gpui::SharedString;
use parking_lot::RwLock;
use settings::{Settings, SettingsContent, SettingsStore};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

type TranslationMap = HashMap<&'static str, &'static str>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    SimplifiedChinese,
}

static LANGUAGE: RwLock<Language> = RwLock::new(Language::SimplifiedChinese);
static ZH_CN_TRANSLATIONS: OnceLock<TranslationMap> = OnceLock::new();

const ZH_CN_ENTRIES: &[(&str, &str)] = &[
    ("settings.group.font", "字体"),
    ("settings.group.editor", "编辑器"),
    ("settings.group.gutter", "行号栏"),
    ("setting.buffer_font_family", "编辑器字体"),
    ("setting.buffer_font_size", "编辑器字号"),
    ("setting.buffer_font_weight", "编辑器字体粗细"),
    ("setting.buffer_font_ligatures", "字体连字"),
    ("setting.inline_git_blame", "内联 Git Blame"),
    ("setting.line_numbers", "行号"),
    ("setting.relative_line_numbers", "相对行号"),
    ("option.relative_line_numbers.ascending", "递增"),
    ("option.relative_line_numbers.relative", "相对"),
    ("command.jump_to_file", "跳转到文件"),
    ("command.apply_all", "全部应用"),
    ("command.hold", "按住"),
    ("command.preview", "预览"),
    ("command.jump_to_edit", "跳转到编辑"),
    (
        "warning.conflict_with_accept_keybinding",
        "与接受快捷键冲突",
    ),
    (
        "message.conflict_accept_keybinding",
        "当前按键映射已覆盖默认的接受预测快捷键。要继续，请为 `editor::AcceptEditPrediction` 操作分配一个快捷键。",
    ),
    ("button.assign_keybinding", "分配快捷键"),
    ("button.see_docs", "查看文档"),
    ("menu.settings", "设置"),
    ("menu.settings.open_settings", "打开设置"),
    ("menu.settings.open_key_bindings", "打开键绑定"),
    ("menu.settings.open_default_settings", "打开默认设置"),
    ("menu.settings.open_default_key_bindings", "打开默认键绑定"),
    ("menu.settings.open_project_settings", "打开项目设置"),
    ("menu.settings.select_profile", "选择设置配置"),
    ("menu.settings.select_theme", "选择主题"),
    ("menu.interface_language", "界面语言"),
    ("menu.interface_language.follow_system", "跟随系统"),
    ("menu.interface_language.simplified_chinese", "简体中文"),
    ("menu.interface_language.english", "英语"),
    ("menu.zed", "Zed"),
    ("menu.zed.about", "关于 Zed"),
    ("menu.zed.check_updates", "检查更新"),
    ("menu.zed.services", "服务"),
    ("menu.zed.extensions", "扩展"),
    ("menu.zed.install_cli", "安装 CLI"),
    ("menu.zed.hide", "隐藏 Zed"),
    ("menu.zed.hide_others", "隐藏其他"),
    ("menu.zed.show_all", "显示全部"),
    ("menu.zed.quit", "退出 Zed"),
    ("menu.file", "文件"),
    ("menu.file.new", "新建"),
    ("menu.file.new_window", "新建窗口"),
    ("menu.file.open_file", "打开文件..."),
    ("menu.file.open_folder", "打开文件夹..."),
    ("menu.file.open", "打开…"),
    ("menu.file.open_recent", "打开最近..."),
    ("menu.file.open_remote", "打开远程..."),
    ("menu.file.add_folder_to_project", "添加文件夹到项目…"),
    ("menu.file.save", "保存"),
    ("menu.file.save_as", "另存为…"),
    ("menu.file.save_all", "全部保存"),
    ("menu.file.close_editor", "关闭编辑器"),
    ("menu.file.close_window", "关闭窗口"),
    ("menu.edit", "编辑"),
    ("menu.edit.undo", "撤销"),
    ("menu.edit.redo", "重做"),
    ("menu.edit.cut", "剪切"),
    ("menu.edit.copy", "复制"),
    ("menu.edit.copy_trim", "复制并修剪"),
    ("menu.edit.paste", "粘贴"),
    ("menu.edit.find", "查找"),
    ("menu.edit.find_in_project", "在项目中查找"),
    ("menu.edit.toggle_line_comment", "切换行注释"),
    ("menu.selection", "选择"),
    ("menu.selection.select_all", "全选"),
    ("menu.selection.expand", "扩展选区"),
    ("menu.selection.shrink", "收缩选区"),
    ("menu.selection.select_next_sibling", "选择下一个同级"),
    ("menu.selection.select_previous_sibling", "选择上一个同级"),
    ("menu.selection.add_cursor_above", "在上方添加光标"),
    ("menu.selection.add_cursor_below", "在下方添加光标"),
    ("menu.selection.select_next_occurrence", "选择下一个匹配"),
    ("menu.selection.move_line_up", "上移一行"),
    ("menu.selection.move_line_down", "下移一行"),
    ("menu.selection.duplicate_selection", "复制选区"),
    ("menu.view", "视图"),
    ("menu.view.zoom_in", "放大"),
    ("menu.view.zoom_out", "缩小"),
    ("menu.view.reset_zoom", "重置缩放"),
    ("menu.view.toggle_left_dock", "切换左侧停靠栏"),
    ("menu.view.toggle_right_dock", "切换右侧停靠栏"),
    ("menu.view.toggle_bottom_dock", "切换底部停靠栏"),
    ("menu.view.close_all_docks", "关闭所有停靠栏"),
    ("menu.view.editor_layout", "编辑器布局"),
    ("menu.view.split_up", "向上分屏"),
    ("menu.view.split_down", "向下分屏"),
    ("menu.view.split_left", "向左分屏"),
    ("menu.view.split_right", "向右分屏"),
    ("menu.view.project_panel", "项目面板"),
    ("menu.view.outline_panel", "大纲面板"),
    ("menu.view.collab_panel", "协作面板"),
    ("menu.view.terminal_panel", "终端面板"),
    ("menu.view.debugger_panel", "调试面板"),
    ("menu.view.diagnostics", "诊断"),
    ("menu.go", "前往"),
    ("menu.go.back", "后退"),
    ("menu.go.forward", "前进"),
    ("menu.go.command_palette", "命令面板..."),
    ("menu.go.go_to_file", "跳转到文件..."),
    ("menu.go.go_to_symbol_in_editor", "跳转到编辑器符号..."),
    ("menu.go.go_to_line_column", "跳转到行/列..."),
    ("menu.go.go_to_definition", "跳转到定义"),
    ("menu.go.go_to_declaration", "跳转到声明"),
    ("menu.go.go_to_type_definition", "跳转到类型定义"),
    ("menu.go.find_all_references", "查找所有引用"),
    ("menu.go.next_problem", "下一个问题"),
    ("menu.go.previous_problem", "上一个问题"),
    ("menu.run", "运行"),
    ("menu.run.spawn_task", "启动任务"),
    ("menu.run.start_debugger", "启动调试器"),
    ("menu.run.edit_tasks", "编辑 tasks.json..."),
    ("menu.run.edit_debug", "编辑 debug.json..."),
    ("menu.run.continue", "继续"),
    ("menu.run.step_over", "单步跳过"),
    ("menu.run.step_into", "单步进入"),
    ("menu.run.step_out", "单步跳出"),
    ("menu.run.toggle_breakpoint", "切换断点"),
    ("menu.run.edit_breakpoint", "编辑断点"),
    ("menu.run.clear_all_breakpoints", "清除所有断点"),
    ("menu.window", "窗口"),
    ("menu.window.minimize", "最小化"),
    ("menu.window.zoom", "缩放"),
    ("menu.help", "帮助"),
    ("menu.help.view_release_notes", "查看发行说明"),
    ("menu.help.view_telemetry", "查看遥测数据"),
    ("menu.help.view_dependency_licenses", "查看依赖许可"),
    ("menu.help.show_welcome", "显示欢迎页"),
    ("menu.help.give_feedback", "提交反馈..."),
    ("menu.help.documentation", "文档"),
    ("menu.help.zed_twitter", "Zed 推特"),
    ("menu.help.join_the_team", "加入团队"),
    ("prompt.restart_required.title", "需要重启"),
    (
        "prompt.restart_required.message",
        "请重启 Zed 以完全应用语言更改。",
    ),
    ("prompt.restart_required.restart_now", "立即重启"),
    ("prompt.restart_required.later", "稍后"),
];

fn zh_cn_translations() -> &'static TranslationMap {
    ZH_CN_TRANSLATIONS.get_or_init(|| {
        let mut map = HashMap::with_capacity(ZH_CN_ENTRIES.len());
        for (key, value) in ZH_CN_ENTRIES {
            map.insert(*key, *value);
        }
        map
    })
}

pub fn translate_static(key: &str, default: &'static str) -> Cow<'static, str> {
    match *LANGUAGE.read() {
        Language::English => Cow::Borrowed(default),
        Language::SimplifiedChinese => zh_cn_translations()
            .get(key)
            .map(|value| Cow::Borrowed(*value))
            .unwrap_or_else(|| Cow::Borrowed(default)),
    }
}

pub fn translate_owned(key: &str, default: &str) -> String {
    match *LANGUAGE.read() {
        Language::English => default.to_string(),
        Language::SimplifiedChinese => zh_cn_translations()
            .get(key)
            .copied()
            .unwrap_or(default)
            .to_string(),
    }
}

pub fn shared(key: &str, default: &'static str) -> SharedString {
    match translate_static(key, default) {
        Cow::Borrowed(value) => SharedString::new_static(value),
        Cow::Owned(value) => SharedString::from(value),
    }
}

pub fn set_language(language: Language) {
    *LANGUAGE.write() = language;
}

pub fn current_language() -> Language {
    *LANGUAGE.read()
}

pub fn init(cx: &mut gpui::App) {
    LocalizationSettings::register(cx);
    apply_settings(cx);

    cx.observe_global::<SettingsStore>(|cx| {
        apply_settings(cx);
    })
    .detach();
}

fn apply_settings(cx: &mut gpui::App) {
    let settings = LocalizationSettings::get_global(cx);
    set_language(settings.language);
}

#[derive(Clone, Copy, Debug)]
pub struct LocalizationSettings {
    language: Language,
}

impl LocalizationSettings {
    pub fn language(&self) -> Language {
        self.language
    }
}

impl Settings for LocalizationSettings {
    fn from_settings(content: &SettingsContent, _: &mut gpui::App) -> Self {
        use settings::UiLanguagePreference;

        let preference = content
            .ui_language
            .unwrap_or(UiLanguagePreference::SimplifiedChinese);

        let language = match preference {
            UiLanguagePreference::SimplifiedChinese | UiLanguagePreference::Auto => {
                Language::SimplifiedChinese
            }
            UiLanguagePreference::English => Language::English,
        };

        Self { language }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_GUARD: Mutex<()> = Mutex::new(());

    #[test]
    fn defaults_to_simplified_chinese() {
        let _guard = TEST_GUARD.lock().unwrap();
        let original = current_language();
        set_language(Language::SimplifiedChinese);

        assert_eq!(
            translate_owned("command.apply_all", "Apply All"),
            "全部应用"
        );

        set_language(original);
    }

    #[test]
    fn english_returns_default_text() {
        let _guard = TEST_GUARD.lock().unwrap();
        let original = current_language();
        set_language(Language::English);

        assert_eq!(
            translate_owned("command.apply_all", "Apply All"),
            "Apply All"
        );

        set_language(original);
    }

    #[test]
    fn missing_key_falls_back_to_default() {
        let _guard = TEST_GUARD.lock().unwrap();
        let original = current_language();
        set_language(Language::SimplifiedChinese);

        assert_eq!(translate_owned("missing.key", "Fallback"), "Fallback");

        set_language(original);
    }

    #[test]
    fn restart_prompt_translations_present() {
        let _guard = TEST_GUARD.lock().unwrap();
        let original = current_language();
        set_language(Language::SimplifiedChinese);

        assert_eq!(
            translate_owned("prompt.restart_required.restart_now", "Restart Now",),
            "立即重启"
        );

        set_language(original);
    }

    #[test]
    fn menu_translations_present() {
        let _guard = TEST_GUARD.lock().unwrap();
        let original = current_language();
        set_language(Language::SimplifiedChinese);

        assert_eq!(translate_owned("menu.file.save", "Save"), "保存");
        assert_eq!(translate_owned("menu.view.zoom_in", "Zoom In"), "放大");
        assert_eq!(
            translate_owned("menu.help.documentation", "Documentation"),
            "文档"
        );

        set_language(original);
    }
}
