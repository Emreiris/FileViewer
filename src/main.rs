

#![windows_subsystem = "windows"]


use druid::WidgetExt;
use druid::{Widget, WindowDesc,Data,AppLauncher, Lens, FileDialogOptions,
    commands::{OPEN_FILE, QUIT_APP, SHOW_OPEN_PANEL},
    WindowId, LocalizedString, Menu, MenuItem, Env};

use druid::widget::TextBox;

use serde_json::Value;
use std::fmt::{self};
use std::fs;

#[derive(Clone, Lens, Debug)]
struct JsonText
{
    text:String
}

impl Data for JsonText {
    
    fn same(&self, other: &Self) -> bool
    {
        self.text == other.text
    }
}

impl fmt::Display for JsonText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

fn build_ui() -> impl Widget<JsonText> {
    TextBox::multiline()
        .with_placeholder("Your sample text here :)")
        .fix_width(360.0)
        .fix_height(140.0)
        .lens(JsonText::text)
}

fn make_menu(_: Option<WindowId>, _: &JsonText, _: &Env) -> Menu<JsonText> {
    Menu::new(LocalizedString::new("json-viewer.menu")).entry(
        Menu::new(LocalizedString::new("json-viewer.menu.file").with_placeholder("File"))
            .entry(
                MenuItem::new(
                    LocalizedString::new("json-viewer.menu.file.open").with_placeholder("Open"),
                )
                .command(SHOW_OPEN_PANEL.with(Default::default())),
            )
            .separator()
            .entry(
                MenuItem::new(
                    LocalizedString::new("json-viewer.menu.file.quit").with_placeholder("Quit"),
                )
                .command(QUIT_APP),
            ),
    )
}

fn read_json(path:String) -> JsonText
{
    JsonText{text: String::from(fs::read_to_string(path).unwrap())}
}

fn main() {

    let main_window = WindowDesc::new(build_ui())
        .window_size((800.0, 600.0))
        .title("Text viewer").menu(make_menu);
    let json_text = read_json(String::from("example.json"));
    
    AppLauncher::with_window(main_window)
        .launch(json_text)
        .expect("Failed to launch application");
}
