mod parser;
mod settings;

use gpui_kit::component::{
    ActiveTheme as _, IndexPath, Sizable as _, Theme, ThemeMode,
    button::{Button, ButtonVariants as _},
    checkbox::Checkbox,
    h_flex,
    input::{Input, InputEvent, InputState, Textarea, TextareaState},
    searchable_list::SearchableVec,
    select::{Select, SelectEvent, SelectState},
    table::{Column, DataTable, TableDelegate, TableState},
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

use std::path::{Path, PathBuf};

use parser::{Row, compare, parse_env};
use settings::Settings;

const KEY_COLUMN: &str = "key";
const MIN_SOURCES: usize = 2;

/// One pasted `.env` file: a label (e.g. "AllTemp") and its raw contents.
struct Source {
    id: usize,
    name: Entity<InputState>,
    contents: Entity<TextareaState>,
    _subscriptions: Vec<Subscription>,
}

/// Table delegate: the key column followed by one column per source.
struct EnvTable {
    columns: Vec<Column>,
    rows: Vec<Row>,
    highlight: bool,
}

impl EnvTable {
    fn new() -> Self {
        Self {
            columns: vec![Column::new(KEY_COLUMN, "").width(220.)],
            rows: Vec::new(),
            highlight: false,
        }
    }
}

impl TableDelegate for EnvTable {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _: &App) -> Column {
        self.columns[col_ix].clone()
    }

    fn render_th(
        &mut self,
        col_ix: usize,
        _: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(cx.theme().foreground)
            .child(self.columns[col_ix].name.clone())
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        cx: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = &self.rows[row_ix];

        if col_ix == 0 {
            return div()
                .font_weight(FontWeight::SEMIBOLD)
                .child(row.key.clone());
        }

        let source = col_ix - 1;
        match &row.values[source] {
            Some(value) if self.highlight && row.differs() => {
                let marker = cx.theme().warning;
                h_flex()
                    .whitespace_nowrap()
                    .children(
                        row.diff_runs(source)
                            .into_iter()
                            .map(move |(text, differs)| {
                                let run = div().child(text);
                                if differs {
                                    run.bg(marker.opacity(0.45)).font_weight(FontWeight::BOLD)
                                } else {
                                    run
                                }
                            }),
                    )
            }
            Some(value) if row.differs() => {
                div().text_color(cx.theme().warning).child(value.clone())
            }
            Some(value) => div().child(value.clone()),
            None => div().text_color(cx.theme().muted_foreground).child("—"),
        }
    }
}

/// Name a source after its file; if another file in the batch shares the file
/// name, prefix the parent directory to tell them apart.
fn display_name<'a>(path: &Path, batch: impl Iterator<Item = &'a PathBuf>) -> String {
    let file = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let clashes = batch
        .filter(|other| other.file_name() == path.file_name())
        .count()
        > 1;
    match path.parent().and_then(|p| p.file_name()) {
        Some(dir) if clashes => format!("{}/{file}", dir.to_string_lossy()),
        _ => file,
    }
}

/// Installed font families, skipping hidden system pseudo-fonts (".SF…").
fn installed_fonts(cx: &App) -> Vec<String> {
    cx.text_system()
        .all_font_names()
        .into_iter()
        .filter(|name| !name.starts_with('.'))
        .collect()
}

struct EnvCompare {
    sources: Vec<Source>,
    next_id: usize,
    table: Entity<TableState<EnvTable>>,
    only_differences: bool,
    highlight: bool,
    settings: Settings,
    show_settings: bool,
    font_select: Entity<SelectState<SearchableVec<String>>>,
    _font_subscription: Subscription,
}

impl EnvCompare {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let table = cx.new(|cx| TableState::new(EnvTable::new(), window, cx));
        let settings = Settings::load();
        Self::apply_theme(&settings, window, cx);

        let fonts = installed_fonts(cx);
        let selected = fonts
            .iter()
            .position(|name| name == &settings.font_family)
            .map(IndexPath::new);
        let font_select = cx.new(|cx| {
            SelectState::new(SearchableVec::new(fonts), selected, window, cx).searchable(true)
        });
        let font_subscription = cx.subscribe(
            &font_select,
            |this, _, event: &SelectEvent<SearchableVec<String>>, cx| {
                let SelectEvent::Confirm(font) = event;
                this.settings.font_family = font.clone().unwrap_or_default();
                this.settings.save();
                cx.notify();
            },
        );

        let mut this = Self {
            sources: Vec::new(),
            next_id: 0,
            table,
            only_differences: false,
            highlight: false,
            settings,
            show_settings: false,
            font_select,
            _font_subscription: font_subscription,
        };
        for _ in 0..MIN_SOURCES {
            this.add_source(window, cx);
        }
        this
    }

    fn apply_theme(settings: &Settings, window: &mut Window, cx: &mut App) {
        let mode = if settings.dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        Theme::change(mode, Some(window), cx);
    }

    fn add_source(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.add_source_with(None, None, window, cx);
    }

    /// Add an environment, optionally pre-filled with a name and contents.
    fn add_source_with(
        &mut self,
        label: Option<String>,
        text: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id = self.next_id;
        self.next_id += 1;

        let label = label.unwrap_or_else(|| format!("Env {}", id + 1));
        let name = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Name")
                .default_value(label)
        });
        let contents = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder("Paste .env contents here…")
                .rows(10)
        });
        if let Some(text) = text {
            // `set_value` emits no change event; `rebuild` below picks it up.
            contents.update(cx, |state, cx| state.set_value(text, window, cx));
        }

        let subscriptions = vec![
            cx.subscribe(&name, |this, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    this.rebuild(cx);
                }
            }),
            cx.subscribe(&contents, |this, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    this.rebuild(cx);
                }
            }),
        ];

        self.sources.push(Source {
            id,
            name,
            contents,
            _subscriptions: subscriptions,
        });
        self.rebuild(cx);
    }

    /// Show the native file picker and load every selected file as an environment.
    fn open_files(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let picked = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: true,
            prompt: Some("Compare".into()),
        });
        cx.spawn_in(window, async move |this, cx| {
            if let Ok(Ok(Some(paths))) = picked.await {
                this.update_in(cx, |this, window, cx| this.load_paths(&paths, window, cx))
                    .ok();
            }
        })
        .detach();
    }

    /// Load each file as its own environment, replacing any still-empty ones.
    fn load_paths(&mut self, paths: &[PathBuf], window: &mut Window, cx: &mut Context<Self>) {
        let files: Vec<(&PathBuf, String)> = paths
            .iter()
            .filter_map(|path| Some((path, std::fs::read_to_string(path).ok()?)))
            .collect();
        if files.is_empty() {
            return;
        }

        self.sources
            .retain(|source| !source.contents.read(cx).value().trim().is_empty());

        for (path, text) in &files {
            let label = display_name(path, files.iter().map(|(p, _)| *p));
            self.add_source_with(Some(label), Some(text.clone()), window, cx);
        }
        while self.sources.len() < MIN_SOURCES {
            self.add_source(window, cx);
        }
    }

    /// Empty an environment's contents but keep its column and name.
    fn clear_source(&mut self, id: usize, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(source) = self.sources.iter().find(|source| source.id == id) {
            // `set_value` emits no change event, so rebuild explicitly.
            source
                .contents
                .update(cx, |state, cx| state.set_value("", window, cx));
            self.rebuild(cx);
        }
    }

    fn remove_source(&mut self, id: usize, cx: &mut Context<Self>) {
        if self.sources.len() > MIN_SOURCES {
            self.sources.retain(|source| source.id != id);
            self.rebuild(cx);
        }
    }

    /// Re-parse every source and push the result into the table.
    fn rebuild(&mut self, cx: &mut Context<Self>) {
        let parsed: Vec<_> = self
            .sources
            .iter()
            .map(|source| parse_env(&source.contents.read(cx).value()))
            .collect();
        let mut rows = compare(&parsed);
        if self.only_differences {
            rows.retain(Row::differs);
        }
        let highlight = self.highlight;

        let mut columns = vec![Column::new(KEY_COLUMN, "").width(220.)];
        for source in &self.sources {
            let label = source.name.read(cx).value();
            columns.push(Column::new(format!("source-{}", source.id), label).width(260.));
        }

        self.table.update(cx, |table, cx| {
            let delegate = table.delegate_mut();
            delegate.columns = columns;
            delegate.rows = rows;
            delegate.highlight = highlight;
            table.refresh(cx);
        });
        cx.notify();
    }

    fn render_source(&self, source: &Source, cx: &mut Context<Self>) -> impl IntoElement {
        let id = source.id;
        let removable = self.sources.len() > MIN_SOURCES;

        v_flex()
            .gap_2()
            .p_3()
            .border_1()
            .border_color(cx.theme().border)
            .rounded_lg()
            .child(
                h_flex()
                    .gap_2()
                    .child(div().flex_1().child(Input::new(&source.name)))
                    .child(
                        Button::new(("clear", id))
                            .ghost()
                            .small()
                            .label("Clear")
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.clear_source(id, window, cx);
                            })),
                    )
                    .when(removable, |row| {
                        row.child(
                            Button::new(("remove", id))
                                .ghost()
                                .small()
                                .label("Remove")
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.remove_source(id, cx);
                                })),
                        )
                    }),
            )
            .child(Textarea::new(&source.contents))
    }
}

impl Render for EnvCompare {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sources: Vec<_> = self
            .sources
            .iter()
            .map(|source| self.render_source(source, cx).into_any_element())
            .collect();

        h_flex()
            .size_full()
            .on_drop(cx.listener(|this, dropped: &ExternalPaths, window, cx| {
                this.load_paths(dropped.paths(), window, cx);
            }))
            .font_family(if self.settings.font_family.is_empty() {
                cx.theme().mono_font_family.clone()
            } else {
                self.settings.font_family.clone().into()
            })
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                v_flex()
                    .w(px(380.))
                    .h_full()
                    .flex_none()
                    .gap_3()
                    .p_4()
                    .border_r_1()
                    .border_color(cx.theme().border)
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::BOLD)
                                            .child("ENV Compare"),
                                    )
                                    .child(
                                        Button::new("settings")
                                            .ghost()
                                            .small()
                                            .label(if self.show_settings {
                                                "Hide settings"
                                            } else {
                                                "Settings"
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.show_settings = !this.show_settings;
                                                cx.notify();
                                            })),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("open-files")
                                            .outline()
                                            .small()
                                            .label("Open files…")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.open_files(window, cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("add-source")
                                            .primary()
                                            .small()
                                            .label("Add environment")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.add_source(window, cx);
                                            })),
                                    ),
                            ),
                    )
                    .when(self.show_settings, |panel| {
                        panel.child(
                            v_flex()
                                .gap_2()
                                .p_3()
                                .border_1()
                                .border_color(cx.theme().border)
                                .rounded_lg()
                                .child(
                                    Checkbox::new("dark-mode")
                                        .label("Dark mode")
                                        .checked(self.settings.dark_mode)
                                        .on_click(cx.listener(
                                            |this, checked: &bool, window, cx| {
                                                this.settings.dark_mode = *checked;
                                                this.settings.save();
                                                Self::apply_theme(&this.settings, window, cx);
                                                cx.notify();
                                            },
                                        )),
                                )
                                .child(div().text_sm().child("Font family"))
                                .child(
                                    Select::new(&self.font_select)
                                        .placeholder("Theme default")
                                        .search_placeholder("Search fonts")
                                        .cleanable(true),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Clear the selection to use the default font."),
                                ),
                        )
                    })
                    .child(
                        v_flex()
                            .gap_2()
                            .child(
                                Checkbox::new("only-differences")
                                    .label("Show only differences")
                                    .checked(self.only_differences)
                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                        this.only_differences = *checked;
                                        this.rebuild(cx);
                                    })),
                            )
                            .child(
                                Checkbox::new("highlight-differences")
                                    .label("Highlight differences")
                                    .checked(self.highlight)
                                    .on_click(cx.listener(|this, checked: &bool, _, cx| {
                                        this.highlight = *checked;
                                        this.rebuild(cx);
                                    })),
                            ),
                    )
                    .child(
                        v_flex()
                            .id("sources")
                            .flex_1()
                            .gap_3()
                            .overflow_y_scroll()
                            .children(sources),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .p_4()
                    .child(DataTable::new(&self.table).stripe(true).bordered(true)),
            )
    }
}

fn main() {
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(|cx| {
            gpui_kit::init(cx);

            gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
                cx.new(|cx| EnvCompare::new(window, cx))
            })
            .expect("failed to open window");

            cx.activate(true);
        });
}
