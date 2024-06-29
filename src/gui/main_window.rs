use crate::gui::entry_list_item::{EntryListItem, EntryType};
use crate::gui::utils::make_list_view_wrapper_from_data_vault;
use crate::AppState;
use adw::prelude::*;
use relm4::{prelude::*, typed_view::list::TypedListView};
use relm4_icons::icon_names;
use std::cell::RefCell;
use std::rc::Rc;

use super::add_entry_prompt::{AddEntryPrompt, AddEntryPromptMsg, AddEntryPromptOutput};

#[derive(Debug, PartialEq, Eq)]
pub enum EntryTypeView {
    Password,
    Note,
    Card,
    TOTP,
}

pub struct MainWindow {
    is_logged_in: bool,

    entry_view: EntryTypeView,
    list_view_wrapper: TypedListView<EntryListItem, gtk::SingleSelection>,

    add_entry_prompt: Controller<AddEntryPrompt>,
}

#[derive(Debug)]
pub enum MainWindowMsg {
    SetMode(EntryTypeView),

    NewEntryListItem(EntryListItem),

    ShowAddEntryPrompt,
}

#[relm4::component(pub)]
impl SimpleComponent for MainWindow {
    type Init = Rc<RefCell<AppState>>;
    type Input = MainWindowMsg;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_margin_all: 20,
            set_modal: true,
            set_title: Some("Password Manager"),
            set_resizable: true,
            set_default_size: (1000, 700),
            set_css_classes: &["background", "csd", "circular", "accent"],

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,

                    #[wrap(Some)]
                    set_title_widget = &gtk::Box {
                        set_spacing: 20,

                        gtk::Box {
                            add_css_class: "linked",
                            append: group = &gtk::ToggleButton {
                                set_label: "Passwords",
                                set_has_frame: true,
                                set_active: true,
                                connect_clicked[sender] => move |_| {
                                    sender.input(MainWindowMsg::SetMode(EntryTypeView::Password));

                                },
                            },
                            gtk::ToggleButton {
                                set_label: "Notes",
                                set_has_frame: true,
                                set_group: Some(&group),
                                connect_clicked[sender] => move |_| {
                                    sender.input(MainWindowMsg::SetMode(EntryTypeView::Note));
                                }
                            },

                            gtk::ToggleButton {
                                set_label: "Cards",
                                set_has_frame: true,
                                set_group: Some(&group),
                                connect_clicked[sender] => move |_| {
                                    sender.input(MainWindowMsg::SetMode(EntryTypeView::Card));
                                }
                            },

                            gtk::ToggleButton {
                                set_label: "OTP",
                                set_has_frame: true,
                                set_group: Some(&group),
                                connect_clicked[sender] => move |_| {
                                    sender.input(MainWindowMsg::SetMode(EntryTypeView::TOTP));
                                }
                            },
                        },

                        gtk::Button {
                            set_has_frame: true,
                            set_icon_name: icon_names::PLUS_LARGE,

                            connect_clicked[sender] => move |_| {
                                sender.input(MainWindowMsg::ShowAddEntryPrompt);
                            }
                        }
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_height_request: 300,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 20,
                        set_margin_all: 20,
                        set_width_request: 200,

                        gtk::ScrolledWindow {
                            set_vexpand: true,
                            set_hexpand: true,
                            set_has_frame: true,
                            inline_css: "border: 3px solid gray; border-radius: 6px;",

                            #[local_ref]
                            list_view -> gtk::ListView {
                                set_single_click_activate: true,
                                connect_activate => move |_, nr| {
                                    println!("Activated: {}", nr);
                                }
                            }
                        }
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 20,
                        set_margin_all: 20,
                        set_width_request: 500,

                        set_vexpand: true,
                        set_hexpand: true,

                        // Password View
                        adw::PreferencesGroup {
                            set_title: "Password",
                            #[watch]
                            set_visible: matches!(model.entry_view, EntryTypeView::Password),

                            add = &adw::EntryRow {
                                set_title : "Name",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Username",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Password",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "URL",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Expiration Date",
                                set_editable : false,
                            },
                        },

                        // Note View
                        adw::PreferencesGroup {
                            set_title: "Note",
                            #[watch]
                            set_visible: matches!(model.entry_view, EntryTypeView::Note),

                            add = &adw::EntryRow {
                                set_title : "Name",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Content",
                                set_editable : false,
                            },
                        },

                        // Card View
                        adw::PreferencesGroup {
                            set_title: "Card",
                            #[watch]
                            set_visible: matches!(model.entry_view, EntryTypeView::Card),

                            add = &adw::EntryRow {
                                set_title : "Name",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Cardholder Name",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Card Number",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Security Code",
                                set_editable : false,
                            },

                            add = &adw::EntryRow {
                                set_title : "Expiration Date",
                                set_editable : false,
                            },
                        },
                    }
                }
            }
        }
    }

    fn init(
        state: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut list_view_wrapper = make_list_view_wrapper_from_data_vault(state.clone());

        list_view_wrapper.add_filter(|item| item.entry_type == EntryType::Password);
        list_view_wrapper.add_filter(|item| item.entry_type == EntryType::Note);
        list_view_wrapper.add_filter(|item| item.entry_type == EntryType::Card);
        list_view_wrapper.add_filter(|item| item.entry_type == EntryType::TOTP);

        list_view_wrapper.set_filter_status(0, true);
        list_view_wrapper.set_filter_status(1, false);
        list_view_wrapper.set_filter_status(2, false);
        list_view_wrapper.set_filter_status(3, false);

        let add_entry_prompt: Controller<AddEntryPrompt> = AddEntryPrompt::builder()
            .launch(state.clone())
            .forward(sender.input_sender(), |msg| match msg {
                AddEntryPromptOutput::NewEntryListItem(new_entry_list_item) => {
                    MainWindowMsg::NewEntryListItem(new_entry_list_item)
                }
            });

        let model = MainWindow {
            is_logged_in: false,

            entry_view: EntryTypeView::Password,
            list_view_wrapper,

            add_entry_prompt,
        };

        let list_view = &model.list_view_wrapper.view;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            MainWindowMsg::SetMode(mode) => {
                self.entry_view = mode;

                self.list_view_wrapper
                    .set_filter_status(0, self.entry_view == EntryTypeView::Password);
                self.list_view_wrapper
                    .set_filter_status(1, self.entry_view == EntryTypeView::Note);
                self.list_view_wrapper
                    .set_filter_status(2, self.entry_view == EntryTypeView::Card);
                self.list_view_wrapper
                    .set_filter_status(3, self.entry_view == EntryTypeView::TOTP);
            }

            MainWindowMsg::NewEntryListItem(new_entry_list_item) => {
                self.list_view_wrapper.append(new_entry_list_item);
            }

            MainWindowMsg::ShowAddEntryPrompt => {
                self.add_entry_prompt.emit(AddEntryPromptMsg::Show);
            }
        }
    }
}

pub fn run_main_window(app_state: Rc<RefCell<AppState>>) {
    let app = RelmApp::new("rust-password-manager-client");
    app.run::<MainWindow>(app_state);
}