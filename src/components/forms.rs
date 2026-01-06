use dioxus::prelude::*;
use remind_me_ui::{
    Button, ButtonVariant,
    Card, CardContent, CardHeader, CardTitle,
    FormField, Input, Textarea, Checkbox,
};
use crate::models::{Reminder, Tag};
use crate::i18n::use_t;
use crate::utils::{now_rfc3339, now_timestamp_millis, to_datetime_local_value};

#[component]
pub fn AddReminderForm(
    tags: Vec<Tag>,
    on_add: EventHandler<Reminder>,
) -> Element {
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut due_date = use_signal(String::new);
    let mut selected_tag_ids = use_signal(|| Vec::<String>::new());

    rsx! {
        Card {
            class: "add-form",
            header: rsx! {
                CardHeader {
                    CardTitle { {use_t("form.new_reminder.title")} }
                }
            },
            CardContent {
                FormField {
                    id: "reminder_title".to_string(),
                    name: "title".to_string(),
                    label: use_t("form.title.label"),
                    required: true,
                    Input {
                        id: "reminder_title".to_string(),
                        name: "title".to_string(),
                        r#type: "text",
                        placeholder: use_t("form.title.placeholder"),
                        value: "{title()}",
                        oninput: move |value| title.set(value),
                    }
                }

                FormField {
                    id: "reminder_description".to_string(),
                    name: "description".to_string(),
                    label: use_t("form.description.label"),
                    Textarea {
                        id: "reminder_description".to_string(),
                        name: "description".to_string(),
                        placeholder: use_t("form.description.placeholder"),
                        value: "{description()}",
                        rows: 4,
                        oninput: move |value| description.set(value),
                    }
                }

                FormField {
                    id: "reminder_due_date".to_string(),
                    name: "due_date".to_string(),
                    label: use_t("form.due_date.label"),
                    Input {
                        id: "reminder_due_date".to_string(),
                        name: "due_date".to_string(),
                        r#type: "datetime-local",
                        value: "{due_date()}",
                        oninput: move |value| due_date.set(value),
                    }
                }

                FormField {
                    id: "reminder_tags".to_string(),
                    name: "tags".to_string(),
                    label: use_t("form.tags.label"),
                    div {
                        class: "tag-selection",
                        if tags.is_empty() {
                            p {
                                class: "text-sm text-gray-500",
                                "No tags available. Create tags in Tag Manager."
                            }
                        } else {
                            div {
                                class: "tag-checkboxes",
                                for tag in tags.iter() {
                                    {
                                        let tag_id = tag.id.clone();
                                        let is_checked = selected_tag_ids().contains(&tag_id);
                                        rsx! {
                                            div {
                                                class: "tag-checkbox-label",
                                                Checkbox {
                                                    checked: is_checked,
                                                    onchange: move |_| {
                                                        let mut current = selected_tag_ids();
                                                        if current.contains(&tag_id) {
                                                            current.retain(|id| id != &tag_id);
                                                        } else {
                                                            current.push(tag_id.clone());
                                                        }
                                                        selected_tag_ids.set(current);
                                                    },
                                                }
                                                span {
                                                    class: "tag-checkbox-text",
                                                    style: format!("color: {};", tag.color),
                                                    {tag.name.clone()}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "mt-4 flex justify-end",
                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: title().is_empty(),
                        aria_label: Some(use_t("form.add")),
                        onclick: move |_| {
                            if !title().is_empty() {
                                let reminder = Reminder {
                                    id: format!("reminder_{}", now_timestamp_millis()),
                                    title: title(),
                                    description: description(),
                                    due_date: due_date(),
                                    completed: false,
                                    created_at: now_rfc3339(),
                                    tag_ids: selected_tag_ids(),
                                };
                                on_add.call(reminder);
                                title.set(String::new());
                                description.set(String::new());
                                due_date.set(String::new());
                                selected_tag_ids.set(Vec::new());
                            }
                        },
                        {use_t("form.add")}
                    }
                }
            }
        }
    }
}

#[component]
pub fn EditReminderForm(
    reminder: Reminder,
    tags: Vec<Tag>,
    on_save: EventHandler<Reminder>,
    on_cancel: EventHandler<()>,
) -> Element {
    let mut title = use_signal(|| reminder.title.clone());
    let mut description = use_signal(|| reminder.description.clone());
    let mut due_date = use_signal(|| {
        to_datetime_local_value(&reminder.due_date)
    });
    let mut selected_tag_ids = use_signal(|| reminder.tag_ids.clone());

    rsx! {
        Card {
            class: "add-form",
            header: rsx! {
                CardHeader {
                    CardTitle { {use_t("form.edit_reminder.title")} }
                }
            },
            CardContent {
                FormField {
                    id: "edit_reminder_title".to_string(),
                    name: "title".to_string(),
                    label: use_t("form.title.label"),
                    required: true,
                    Input {
                        id: "edit_reminder_title".to_string(),
                        name: "title".to_string(),
                        r#type: "text",
                        placeholder: use_t("form.title.placeholder"),
                        value: "{title()}",
                        oninput: move |value| title.set(value),
                    }
                }

                FormField {
                    id: "edit_reminder_description".to_string(),
                    name: "description".to_string(),
                    label: use_t("form.description.label"),
                    Textarea {
                        id: "edit_reminder_description".to_string(),
                        name: "description".to_string(),
                        placeholder: use_t("form.description.placeholder"),
                        value: "{description()}",
                        rows: 4,
                        oninput: move |value| description.set(value),
                    }
                }

                FormField {
                    id: "edit_reminder_due_date".to_string(),
                    name: "due_date".to_string(),
                    label: use_t("form.due_date.label"),
                    Input {
                        id: "edit_reminder_due_date".to_string(),
                        name: "due_date".to_string(),
                        r#type: "datetime-local",
                        value: "{due_date()}",
                        oninput: move |value| due_date.set(value),
                    }
                }

                FormField {
                    id: "edit_reminder_tags".to_string(),
                    name: "tags".to_string(),
                    label: use_t("form.tags.label"),
                    div {
                        class: "tag-selection",
                        if tags.is_empty() {
                            p {
                                class: "text-sm text-gray-500",
                                "No tags available. Create tags in Tag Manager."
                            }
                        } else {
                            div {
                                class: "tag-checkboxes",
                                for tag in tags.iter() {
                                    {
                                        let tag_id = tag.id.clone();
                                        let is_checked = selected_tag_ids().contains(&tag_id);
                                        rsx! {
                                            div {
                                                class: "tag-checkbox-label",
                                                Checkbox {
                                                    checked: is_checked,
                                                    onchange: move |_| {
                                                        let mut current = selected_tag_ids();
                                                        if current.contains(&tag_id) {
                                                            current.retain(|id| id != &tag_id);
                                                        } else {
                                                            current.push(tag_id.clone());
                                                        }
                                                        selected_tag_ids.set(current);
                                                    },
                                                }
                                                span {
                                                    class: "tag-checkbox-text",
                                                    style: format!("color: {};", tag.color),
                                                    {tag.name.clone()}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    class: "mt-4 flex justify-end gap-2",
                    Button {
                        variant: ButtonVariant::Ghost,
                        aria_label: Some(use_t("form.cancel")),
                        onclick: move |_| on_cancel.call(()),
                        {use_t("form.cancel")}
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: title().is_empty(),
                        aria_label: Some(use_t("form.save")),
                        onclick: move |_| {
                            if !title().is_empty() {
                                let updated = Reminder {
                                    id: reminder.id.clone(),
                                    title: title(),
                                    description: description(),
                                    due_date: due_date(),
                                    completed: reminder.completed,
                                    created_at: reminder.created_at.clone(),
                                    tag_ids: selected_tag_ids(),
                                };
                                on_save.call(updated);
                                title.set(String::new());
                                description.set(String::new());
                                due_date.set(String::new());
                            }
                        },
                        {use_t("form.save")}
                    }
                }
            }
        }
    }
}
