use adw::{prelude::*, ViewStack, ViewSwitcher};
use adw::{Application, ApplicationWindow, HeaderBar};
use gtk::{
    Box, Button, Label, MenuButton, Popover, ScrolledWindow, SearchBar, SearchEntry, ToggleButton,
};
use std::fmt::Display;
use std::future::Future;
use std::io;
use std::process::Output;
use tokio::process::Command;
fn list_packages() -> impl Future<Output = io::Result<Output>> {
    Command::new("pacman").arg("-Qs").output()
}

#[derive(Debug)]
struct Package {
    repository: String,
    version: String,
    name: String,
    description: String,
}
impl TryFrom<&(&str, &str)> for Package {
    type Error = io::Error;
    fn try_from(p: &(&str, &str)) -> Result<Self, Self::Error> {
        let (repository, name_version) = match p.0.split_once('/') {
            Some((a, b)) => (a, b),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        };
        let (name, version) = match name_version.split_once(' ') {
            Some((a, b)) => (a, b),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")),
        };
        let description = p.1;
        Ok(Self {
            repository: repository.to_owned(),
            version: version.to_owned(),
            name: name.to_owned(),
            description: description.to_owned(),
        })
    }
}
impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
fn ui() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();
    application.connect_activate(|app| {
        let header_bar = HeaderBar::builder().build();
        let view_switcher = ViewSwitcher::new();
        let scroll_window1 = ScrolledWindow::new();
        let scroll_window2 = ScrolledWindow::new();
        let scroll_window3 = ScrolledWindow::new();
        let view_stack = ViewStack::new();
        view_stack.add_titled_with_icon(
            &scroll_window1,
            Some("Explore"),
            "Explore",
            "compass2-symbolic",
        );
        view_stack.add_titled_with_icon(
            &scroll_window2,
            Some("Installed"),
            "Installed",
            "library-symbolic",
        );
        view_stack.add_titled_with_icon(
            &scroll_window3,
            Some("Updates"),
            "Updates",
            "update-symbolic",
        );
        let search_button = ToggleButton::new();
        search_button.set_icon_name("system-search-symbolic");
        let menu_button = MenuButton::new();
        menu_button.set_icon_name("open-menu-symbolic");
        let button_sr = Button::builder().label("Software Repositories").build();
        let button_sr = Label::new(Some("Software Repositories"));
        let button_sr3 = Label::new(Some("Software Repositories"));
        let button_sr2 = Label::new(Some("Software Repositories"));
        let button_sr1 = Label::new(Some("Software Repositories"));
        let button_ks = Button::builder().label("Keyboard Shortcuts").build();
        let button_p = Button::builder().label("Preferences").build();
        let button_as = Button::builder().label("About Software").build();

        let popover = Popover::new();
        popover.set_child(Some(&button_sr));
        popover.set_child(Some(&button_sr1));
        popover.set_child(Some(&button_sr2));
        popover.set_child(Some(&button_sr3));
        //.child(&button_ks)
        //.child(&button_p)
        //.child(&button_as)
        //.build().chi;

        view_switcher.set_stack(Some(&view_stack));
        menu_button.set_popover(Some(&popover));
        header_bar.pack_start(&search_button);
        header_bar.pack_end(&menu_button);
        let search_bar = SearchBar::new();
        search_bar.set_search_mode(false);

        let search_entry = SearchEntry::new();
        search_bar.set_child(Some(&search_entry));
        let content = Box::new(gtk::Orientation::Vertical, 0);
        content.append(&header_bar);
        header_bar.set_title_widget(Some(&view_switcher));
        let window = ApplicationWindow::builder()
            .application(app)
            // add content to window
            .content(&content)
            .build();
        window.present();
    });
    application.run();
}

#[tokio::main]
async fn main() -> Result<(), std::boxed::Box<dyn std::error::Error>> {
    let list = list_packages().await?;
    let vec = list.stdout;
    let string = String::from_utf8(vec.clone()).unwrap();
    let x: Vec<(&str, &str)> = string
        .lines()
        .collect::<Vec<&str>>()
        .chunks(2)
        .filter_map(|s| {
            if s.len() == 2 {
                Some((s[0], s[1]))
            } else {
                None
            }
        })
        .collect();
    let x: Vec<Package> = x.iter().filter_map(|s| Package::try_from(s).ok()).collect();
    //println!("{:?}", x);
    ui();
    Ok(())
}
