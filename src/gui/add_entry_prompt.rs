use std::{cell::RefCell, rc::Rc};

use super::{
    actions::*, add_entry_response_dialog::AddEntryResponseDialog, entry_list_item::EntryListItem,
    utils::generate_random_password,
};
use crate::AppState;

use super::main_window::EntryTypeView;
use adw::prelude::*;
use relm4::{component::Connector, prelude::*};
use relm4_icons::icon_names;

pub struct AddPassword {
    name: gtk::EntryBuffer,
    username: gtk::EntryBuffer,
    password: gtk::EntryBuffer,
    url: gtk::EntryBuffer,
    expiration_date: gtk::EntryBuffer,
}

pub struct AddNote {
    name: gtk::EntryBuffer,
    content: gtk::TextBuffer,
}

pub struct AddCard {
    name: gtk::EntryBuffer,
    cardholder_name: gtk::EntryBuffer,
    card_number: gtk::EntryBuffer,
    security_code: gtk::EntryBuffer,
    expiration_date: gtk::EntryBuffer,
}

pub struct AddTOTPEntry {
    name: gtk::EntryBuffer,
    algorithm: gtk::EntryBuffer,
    secret: gtk::EntryBuffer,
    digits: gtk::EntryBuffer,
    skew: gtk::EntryBuffer,
    period: gtk::EntryBuffer,
}

pub struct AddEntryPrompt {
    is_active: bool,

    entry_type_view: EntryTypeView,

    add_password: AddPassword,
    add_note: AddNote,
    add_card: AddCard,
    add_totp: AddTOTPEntry,

    pub response_dialog: Connector<AddEntryResponseDialog>,

    pub app_state: Rc<RefCell<AppState>>,
}

#[derive(Debug)]
pub enum AddEntryPromptMsg {
    SetMode(EntryTypeView),

    AddPress,

    Show,

    GenerateRandomPassword,
}

#[derive(Debug)]
pub enum AddEntryPromptOutput {
    NewEntryListItem(EntryListItem),
}

#[relm4::component(pub)]
impl SimpleComponent for AddEntryPrompt {
    type Init = Rc<RefCell<AppState>>;
    type Input = AddEntryPromptMsg;
    type Output = AddEntryPromptOutput;

    view! {
        adw::ApplicationWindow {
            set_title: Some("Add Entry"),
            set_modal: true,
            set_css_classes: &["background", "csd"],
            set_hide_on_close: true,

            #[watch]
            set_visible: model.is_active,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,

                    #[wrap(Some)]
                    set_title_widget = &gtk::Box {
                        gtk::Box {
                            add_css_class: "linked",
                            append: group = &gtk::ToggleButton {
                                set_label: "Passwords",
                                set_has_frame: true,
                                set_active: true,
                                connect_clicked[sender] => move |_| {
                                    sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::Password));

                                },
                            },
                            gtk::ToggleButton {
                                set_label: "Notes",
                                set_has_frame: true,
                                set_group: Some(&group),
                                connect_clicked[sender] => move |_| {
                                    sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::Note));
                                }
                            },

                            gtk::ToggleButton {
                                set_label: "Cards",
                                set_has_frame: true,
                                set_group: Some(&group),
                                connect_clicked[sender] => move |_| {
                                    sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::Card));
                                }
                            },

                            gtk::ToggleButton {
                                set_label: "OTP",
                                set_has_frame: true,
                                set_group: Some(&group),
                                connect_clicked[sender] => move |_| {
                                    sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::TOTP));
                                }
                            },
                        },

                        // Generate Password Button
                        gtk::Button {
                            set_has_frame: true,
                            set_icon_name: icon_names::UPDATE,
                            set_tooltip_text: Some("Generate random password (to clipboard)"),

                            connect_clicked[sender] => move |_| {
                                sender.input(AddEntryPromptMsg::GenerateRandomPassword);
                            }
                        },
                    },
                },

                // Add Password Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::Password),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.name,
                    },

                    gtk::Label {
                        set_label: "Username",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.username,
                    },

                    gtk::Label {
                        set_label: "Password",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.password,
                        set_input_purpose: gtk::InputPurpose::Password,
                        set_visibility: false,
                    },

                    gtk::Label {
                        set_label: "URL",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.url,
                    },

                    gtk::Label {
                        set_label: "Expiration Date",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.expiration_date,
                    },
                },

                // Add Note Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::Note),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_note.name,
                    },

                    gtk::Label {
                        set_label: "Content",
                    },
                    gtk::TextView {
                        set_buffer: Some(&model.add_note.content),
                        set_height_request: 100,
                        inline_css: "border: 1px; border-radius: 6px; background-color: #3a3a3a;",
                        set_top_margin: 10,
                        set_bottom_margin: 10,
                        set_left_margin: 10,
                        set_right_margin: 10,
                    },
                },

                // Add Card Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::Card),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.name,
                    },

                    gtk::Label {
                        set_label: "Cardholder Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.cardholder_name,
                    },

                    gtk::Label {
                        set_label: "Card Number",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.card_number,
                    },

                    gtk::Label {
                        set_label: "Security Code",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.security_code,
                    },

                    gtk::Label {
                        set_label: "Expiration Date",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.expiration_date,
                    },
                },

                // Add TOTP Entry Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    set_margin_all: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::TOTP),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_totp.name,
                    },

                    gtk::Label {
                        set_label: "Algorithm",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_totp.algorithm,
                    },

                    gtk::Label {
                        set_label: "Secret",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_totp.secret,
                    },

                    gtk::Label {
                        set_label: "Digits",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_totp.digits,
                    },

                    gtk::Label {
                        set_label: "Skew",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_totp.skew,
                    },

                    gtk::Label {
                        set_label: "Period",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_totp.period,
                    },

                },

                gtk::Button {
                    set_margin_all: 40,
                    set_label: "Add",
                    connect_clicked[sender] => move |_| {
                        sender.input(AddEntryPromptMsg::AddPress);
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
        let model = AddEntryPrompt {
            is_active: false,

            entry_type_view: EntryTypeView::Password,

            add_password: AddPassword {
                name: gtk::EntryBuffer::default(),
                username: gtk::EntryBuffer::default(),
                password: gtk::EntryBuffer::default(),
                url: gtk::EntryBuffer::default(),
                expiration_date: gtk::EntryBuffer::default(),
            },
            add_note: AddNote {
                name: gtk::EntryBuffer::default(),
                content: gtk::TextBuffer::default(),
            },
            add_card: AddCard {
                name: gtk::EntryBuffer::default(),
                cardholder_name: gtk::EntryBuffer::default(),
                card_number: gtk::EntryBuffer::default(),
                security_code: gtk::EntryBuffer::default(),
                expiration_date: gtk::EntryBuffer::default(),
            },
            add_totp: AddTOTPEntry {
                name: gtk::EntryBuffer::default(),
                algorithm: gtk::EntryBuffer::default(),
                secret: gtk::EntryBuffer::default(),
                digits: gtk::EntryBuffer::default(),
                skew: gtk::EntryBuffer::default(),
                period: gtk::EntryBuffer::default(),
            },

            response_dialog: AddEntryResponseDialog::builder()
                .transient_for(&root)
                .launch(()),

            app_state: state,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AddEntryPromptMsg::SetMode(mode) => {
                self.entry_type_view = mode;
            }

            AddEntryPromptMsg::AddPress => match self.entry_type_view {
                EntryTypeView::Password => {
                    let name = self.add_password.name.text();
                    let username = self.add_password.username.text();
                    let password = self.add_password.password.text();
                    let url = self.add_password.url.text();
                    let expiration_date = self.add_password.expiration_date.text();

                    if let Ok(new_entry_list_item) = add_password_action(
                        &name,
                        &username,
                        &password,
                        &url,
                        &expiration_date,
                        self,
                    ) {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }

                EntryTypeView::Note => {
                    let name = self.add_note.name.text();
                    let content = self.add_note.content.text(
                        &self.add_note.content.start_iter(),
                        &self.add_note.content.end_iter(),
                        false,
                    );

                    if let Ok(new_entry_list_item) = add_note_action(&name, &content, self) {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }

                EntryTypeView::Card => {
                    let name = self.add_card.name.text();
                    let cardholder_name = self.add_card.cardholder_name.text();
                    let card_number = self.add_card.card_number.text();
                    let security_code = self.add_card.security_code.text();
                    let expiration_date = self.add_card.expiration_date.text();

                    if let Ok(new_entry_list_item) = add_card_action(
                        &name,
                        &cardholder_name,
                        &card_number,
                        &security_code,
                        &expiration_date,
                        self,
                    ) {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }

                EntryTypeView::TOTP => {
                    let name = self.add_totp.name.text();
                    let algorithm = self.add_totp.algorithm.text();
                    let secret = self.add_totp.secret.text();
                    let digits = self.add_totp.digits.text();
                    let skew = self.add_totp.skew.text();
                    let period = self.add_totp.period.text();

                    if let Ok(new_entry_list_item) =
                        add_totp_action(&name, &algorithm, &secret, &digits, &skew, &period, self)
                    {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }
            },

            AddEntryPromptMsg::Show => {
                self.is_active = true;
            }

            AddEntryPromptMsg::GenerateRandomPassword => {
                let gen_pass = generate_random_password();

                // lol
                let button = gtk::Button::builder().build();
                let clipboard = button.clipboard();

                clipboard.set_text(&gen_pass);
            }
        }
    }
}
