use collab_ui::collab_panel;
use gpui::{Menu, MenuItem, OsAction};
use localization;
use terminal_view::terminal_panel;
use zed_actions::ToggleFocus as ToggleDebugPanel;

pub fn app_menus() -> Vec<Menu> {
    use zed_actions::Quit;

    let settings_menu_items = || {
        vec![
            MenuItem::action(
                localization::shared("menu.settings.open_settings", "Open Settings"),
                super::OpenSettings,
            ),
            MenuItem::action(
                localization::shared("menu.settings.open_key_bindings", "Open Key Bindings"),
                zed_actions::OpenKeymapEditor,
            ),
            MenuItem::action(
                localization::shared(
                    "menu.settings.open_default_settings",
                    "Open Default Settings",
                ),
                super::OpenDefaultSettings,
            ),
            MenuItem::action(
                localization::shared(
                    "menu.settings.open_default_key_bindings",
                    "Open Default Key Bindings",
                ),
                zed_actions::OpenDefaultKeymap,
            ),
            MenuItem::action(
                localization::shared(
                    "menu.settings.open_project_settings",
                    "Open Project Settings",
                ),
                super::OpenProjectSettings,
            ),
            MenuItem::action(
                localization::shared("menu.settings.select_profile", "Select Settings Profile..."),
                zed_actions::settings_profile_selector::Toggle,
            ),
            MenuItem::action(
                localization::shared("menu.settings.select_theme", "Select Theme..."),
                zed_actions::theme_selector::Toggle::default(),
            ),
            MenuItem::submenu(Menu {
                name: localization::shared("menu.interface_language", "Interface Language"),
                items: vec![
                    MenuItem::action(
                        localization::shared(
                            "menu.interface_language.follow_system",
                            "Follow System",
                        ),
                        zed_actions::SetUiLanguage {
                            language: "auto".into(),
                        },
                    ),
                    MenuItem::action(
                        localization::shared(
                            "menu.interface_language.simplified_chinese",
                            "Simplified Chinese",
                        ),
                        zed_actions::SetUiLanguage {
                            language: "zh-CN".into(),
                        },
                    ),
                    MenuItem::action(
                        localization::shared("menu.interface_language.english", "English"),
                        zed_actions::SetUiLanguage {
                            language: "en-US".into(),
                        },
                    ),
                ],
            }),
        ]
    };

    let mut menus = vec![
        Menu {
            name: localization::shared("menu.zed", "Zed"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.zed.about", "About Zed"),
                    zed_actions::About,
                ),
                MenuItem::action(
                    localization::shared("menu.zed.check_updates", "Check for Updates"),
                    auto_update::Check,
                ),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: localization::shared("menu.settings", "Settings"),
                    items: settings_menu_items(),
                }),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu(
                    localization::shared("menu.zed.services", "Services"),
                    gpui::SystemMenuType::Services,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.zed.extensions", "Extensions"),
                    zed_actions::Extensions::default(),
                ),
                #[cfg(not(target_os = "windows"))]
                MenuItem::action(
                    localization::shared("menu.zed.install_cli", "Install CLI"),
                    install_cli::InstallCliBinary,
                ),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action(
                    localization::shared("menu.zed.hide", "Hide Zed"),
                    super::Hide,
                ),
                #[cfg(target_os = "macos")]
                MenuItem::action(
                    localization::shared("menu.zed.hide_others", "Hide Others"),
                    super::HideOthers,
                ),
                #[cfg(target_os = "macos")]
                MenuItem::action(
                    localization::shared("menu.zed.show_all", "Show All"),
                    super::ShowAll,
                ),
                MenuItem::separator(),
                MenuItem::action(localization::shared("menu.zed.quit", "Quit Zed"), Quit),
            ],
        },
        Menu {
            name: localization::shared("menu.file", "File"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.file.new", "New"),
                    workspace::NewFile,
                ),
                MenuItem::action(
                    localization::shared("menu.file.new_window", "New Window"),
                    workspace::NewWindow,
                ),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action(
                    localization::shared("menu.file.open_file", "Open File..."),
                    workspace::OpenFiles,
                ),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        localization::shared("menu.file.open_folder", "Open Folder...")
                    } else {
                        localization::shared("menu.file.open", "Open…")
                    },
                    workspace::Open,
                ),
                MenuItem::action(
                    localization::shared("menu.file.open_recent", "Open Recent..."),
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                ),
                MenuItem::action(
                    localization::shared("menu.file.open_remote", "Open Remote..."),
                    zed_actions::OpenRemote {
                        create_new_window: false,
                        from_existing_connection: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared(
                        "menu.file.add_folder_to_project",
                        "Add Folder to Project…",
                    ),
                    workspace::AddFolderToProject,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.file.save", "Save"),
                    workspace::Save { save_intent: None },
                ),
                MenuItem::action(
                    localization::shared("menu.file.save_as", "Save As…"),
                    workspace::SaveAs,
                ),
                MenuItem::action(
                    localization::shared("menu.file.save_all", "Save All"),
                    workspace::SaveAll { save_intent: None },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.file.close_editor", "Close Editor"),
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action(
                    localization::shared("menu.file.close_window", "Close Window"),
                    workspace::CloseWindow,
                ),
            ],
        },
        Menu {
            name: localization::shared("menu.edit", "Edit"),
            items: vec![
                MenuItem::os_action(
                    localization::shared("menu.edit.undo", "Undo"),
                    editor::actions::Undo,
                    OsAction::Undo,
                ),
                MenuItem::os_action(
                    localization::shared("menu.edit.redo", "Redo"),
                    editor::actions::Redo,
                    OsAction::Redo,
                ),
                MenuItem::separator(),
                MenuItem::os_action(
                    localization::shared("menu.edit.cut", "Cut"),
                    editor::actions::Cut,
                    OsAction::Cut,
                ),
                MenuItem::os_action(
                    localization::shared("menu.edit.copy", "Copy"),
                    editor::actions::Copy,
                    OsAction::Copy,
                ),
                MenuItem::action(
                    localization::shared("menu.edit.copy_trim", "Copy and Trim"),
                    editor::actions::CopyAndTrim,
                ),
                MenuItem::os_action(
                    localization::shared("menu.edit.paste", "Paste"),
                    editor::actions::Paste,
                    OsAction::Paste,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.edit.find", "Find"),
                    search::buffer_search::Deploy::find(),
                ),
                MenuItem::action(
                    localization::shared("menu.edit.find_in_project", "Find In Project"),
                    workspace::DeploySearch::find(),
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.edit.toggle_line_comment", "Toggle Line Comment"),
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: localization::shared("menu.selection", "Selection"),
            items: vec![
                MenuItem::os_action(
                    localization::shared("menu.selection.select_all", "Select All"),
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action(
                    localization::shared("menu.selection.expand", "Expand Selection"),
                    editor::actions::SelectLargerSyntaxNode,
                ),
                MenuItem::action(
                    localization::shared("menu.selection.shrink", "Shrink Selection"),
                    editor::actions::SelectSmallerSyntaxNode,
                ),
                MenuItem::action(
                    localization::shared(
                        "menu.selection.select_next_sibling",
                        "Select Next Sibling",
                    ),
                    editor::actions::SelectNextSyntaxNode,
                ),
                MenuItem::action(
                    localization::shared(
                        "menu.selection.select_previous_sibling",
                        "Select Previous Sibling",
                    ),
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.selection.add_cursor_above", "Add Cursor Above"),
                    editor::actions::AddSelectionAbove,
                ),
                MenuItem::action(
                    localization::shared("menu.selection.add_cursor_below", "Add Cursor Below"),
                    editor::actions::AddSelectionBelow,
                ),
                MenuItem::action(
                    localization::shared(
                        "menu.selection.select_next_occurrence",
                        "Select Next Occurrence",
                    ),
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.selection.move_line_up", "Move Line Up"),
                    editor::actions::MoveLineUp,
                ),
                MenuItem::action(
                    localization::shared("menu.selection.move_line_down", "Move Line Down"),
                    editor::actions::MoveLineDown,
                ),
                MenuItem::action(
                    localization::shared(
                        "menu.selection.duplicate_selection",
                        "Duplicate Selection",
                    ),
                    editor::actions::DuplicateLineDown,
                ),
            ],
        },
        Menu {
            name: localization::shared("menu.view", "View"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.view.zoom_in", "Zoom In"),
                    zed_actions::IncreaseBufferFontSize { persist: false },
                ),
                MenuItem::action(
                    localization::shared("menu.view.zoom_out", "Zoom Out"),
                    zed_actions::DecreaseBufferFontSize { persist: false },
                ),
                MenuItem::action(
                    localization::shared("menu.view.reset_zoom", "Reset Zoom"),
                    zed_actions::ResetBufferFontSize { persist: false },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.view.toggle_left_dock", "Toggle Left Dock"),
                    workspace::ToggleLeftDock,
                ),
                MenuItem::action(
                    localization::shared("menu.view.toggle_right_dock", "Toggle Right Dock"),
                    workspace::ToggleRightDock,
                ),
                MenuItem::action(
                    localization::shared("menu.view.toggle_bottom_dock", "Toggle Bottom Dock"),
                    workspace::ToggleBottomDock,
                ),
                MenuItem::action(
                    localization::shared("menu.view.close_all_docks", "Close All Docks"),
                    workspace::CloseAllDocks,
                ),
                MenuItem::submenu(Menu {
                    name: localization::shared("menu.view.editor_layout", "Editor Layout"),
                    items: vec![
                        MenuItem::action(
                            localization::shared("menu.view.split_up", "Split Up"),
                            workspace::SplitUp,
                        ),
                        MenuItem::action(
                            localization::shared("menu.view.split_down", "Split Down"),
                            workspace::SplitDown,
                        ),
                        MenuItem::action(
                            localization::shared("menu.view.split_left", "Split Left"),
                            workspace::SplitLeft,
                        ),
                        MenuItem::action(
                            localization::shared("menu.view.split_right", "Split Right"),
                            workspace::SplitRight,
                        ),
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.view.project_panel", "Project Panel"),
                    project_panel::ToggleFocus,
                ),
                MenuItem::action(
                    localization::shared("menu.view.outline_panel", "Outline Panel"),
                    outline_panel::ToggleFocus,
                ),
                MenuItem::action(
                    localization::shared("menu.view.collab_panel", "Collab Panel"),
                    collab_panel::ToggleFocus,
                ),
                MenuItem::action(
                    localization::shared("menu.view.terminal_panel", "Terminal Panel"),
                    terminal_panel::ToggleFocus,
                ),
                MenuItem::action(
                    localization::shared("menu.view.debugger_panel", "Debugger Panel"),
                    ToggleDebugPanel,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.view.diagnostics", "Diagnostics"),
                    diagnostics::Deploy,
                ),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: localization::shared("menu.go", "Go"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.go.back", "Back"),
                    workspace::GoBack,
                ),
                MenuItem::action(
                    localization::shared("menu.go.forward", "Forward"),
                    workspace::GoForward,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.go.command_palette", "Command Palette..."),
                    zed_actions::command_palette::Toggle,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.go.go_to_file", "Go to File..."),
                    workspace::ToggleFileFinder::default(),
                ),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action(
                    localization::shared(
                        "menu.go.go_to_symbol_in_editor",
                        "Go to Symbol in Editor...",
                    ),
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action(
                    localization::shared("menu.go.go_to_line_column", "Go to Line/Column..."),
                    editor::actions::ToggleGoToLine,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.go.go_to_definition", "Go to Definition"),
                    editor::actions::GoToDefinition,
                ),
                MenuItem::action(
                    localization::shared("menu.go.go_to_declaration", "Go to Declaration"),
                    editor::actions::GoToDeclaration,
                ),
                MenuItem::action(
                    localization::shared("menu.go.go_to_type_definition", "Go to Type Definition"),
                    editor::actions::GoToTypeDefinition,
                ),
                MenuItem::action(
                    localization::shared("menu.go.find_all_references", "Find All References"),
                    editor::actions::FindAllReferences,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.go.next_problem", "Next Problem"),
                    editor::actions::GoToDiagnostic::default(),
                ),
                MenuItem::action(
                    localization::shared("menu.go.previous_problem", "Previous Problem"),
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: localization::shared("menu.run", "Run"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.run.spawn_task", "Spawn Task"),
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action(
                    localization::shared("menu.run.start_debugger", "Start Debugger"),
                    debugger_ui::Start,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.run.edit_tasks", "Edit tasks.json..."),
                    crate::zed::OpenProjectTasks,
                ),
                MenuItem::action(
                    localization::shared("menu.run.edit_debug", "Edit debug.json..."),
                    zed_actions::OpenProjectDebugTasks,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.run.continue", "Continue"),
                    debugger_ui::Continue,
                ),
                MenuItem::action(
                    localization::shared("menu.run.step_over", "Step Over"),
                    debugger_ui::StepOver,
                ),
                MenuItem::action(
                    localization::shared("menu.run.step_into", "Step Into"),
                    debugger_ui::StepInto,
                ),
                MenuItem::action(
                    localization::shared("menu.run.step_out", "Step Out"),
                    debugger_ui::StepOut,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.run.toggle_breakpoint", "Toggle Breakpoint"),
                    editor::actions::ToggleBreakpoint,
                ),
                MenuItem::action(
                    localization::shared("menu.run.edit_breakpoint", "Edit Breakpoint"),
                    editor::actions::EditLogBreakpoint,
                ),
                MenuItem::action(
                    localization::shared("menu.run.clear_all_breakpoints", "Clear all Breakpoints"),
                    debugger_ui::ClearAllBreakpoints,
                ),
            ],
        },
        Menu {
            name: localization::shared("menu.window", "Window"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.window.minimize", "Minimize"),
                    super::Minimize,
                ),
                MenuItem::action(
                    localization::shared("menu.window.zoom", "Zoom"),
                    super::Zoom,
                ),
                MenuItem::separator(),
            ],
        },
        Menu {
            name: localization::shared("menu.help", "Help"),
            items: vec![
                MenuItem::action(
                    localization::shared("menu.help.view_release_notes", "View Release Notes"),
                    auto_update_ui::ViewReleaseNotesLocally,
                ),
                MenuItem::action(
                    localization::shared("menu.help.view_telemetry", "View Telemetry"),
                    zed_actions::OpenTelemetryLog,
                ),
                MenuItem::action(
                    localization::shared(
                        "menu.help.view_dependency_licenses",
                        "View Dependency Licenses",
                    ),
                    zed_actions::OpenLicenses,
                ),
                MenuItem::action(
                    localization::shared("menu.help.show_welcome", "Show Welcome"),
                    onboarding::ShowWelcome,
                ),
                MenuItem::action(
                    localization::shared("menu.help.give_feedback", "Give Feedback..."),
                    zed_actions::feedback::GiveFeedback,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    localization::shared("menu.help.documentation", "Documentation"),
                    super::OpenBrowser {
                        url: "https://zed.dev/docs".into(),
                    },
                ),
                MenuItem::action(
                    localization::shared("menu.help.zed_twitter", "Zed Twitter"),
                    super::OpenBrowser {
                        url: "https://twitter.com/zeddotdev".into(),
                    },
                ),
                MenuItem::action(
                    localization::shared("menu.help.join_the_team", "Join the Team"),
                    super::OpenBrowser {
                        url: "https://zed.dev/jobs".into(),
                    },
                ),
            ],
        },
    ];

    if cfg!(not(target_os = "macos")) {
        menus.insert(
            0,
            Menu {
                name: localization::shared("menu.settings", "Settings"),
                items: settings_menu_items(),
            },
        );
    }

    menus
}
