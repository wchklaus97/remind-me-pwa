use dioxus::prelude::*;
use remind_me_ui::{
    Button, ButtonVariant, ButtonSize,
    Card, CardContent,
    Modal, ModalSize,
    EmptyState,
};
use crate::models::Tag;
use crate::storage::{load_tags, save_tags};
use crate::components::TagForm;
use crate::i18n::use_t;

#[component]
pub fn TagManager(
    open: Signal<bool>,
    on_close: EventHandler<()>,
) -> Element {
    let mut tags = use_signal(load_tags);
    let mut editing_tag = use_signal(|| None::<Tag>);
    let mut show_form = use_signal(|| false);
    let mut delete_confirm_id = use_signal(|| None::<String>);

    // Reload tags when modal opens
    use_effect(move || {
        if open() {
            tags.set(load_tags());
            editing_tag.set(None);
            show_form.set(false);
            delete_confirm_id.set(None);
        }
    });

    rsx! {
        Modal {
            open: open,
            size: ModalSize::Large,
            title: use_t("tags.title"),
            close_on_backdrop: true,
            on_close: move |_| on_close.call(()),
            
            if show_form() {
                TagForm {
                    tag: editing_tag(),
                    on_save: move |tag: Tag| {
                        let mut updated_tags = tags();
                        if let Some(existing_index) = updated_tags.iter().position(|t| t.id == tag.id) {
                            updated_tags[existing_index] = tag;
                        } else {
                            updated_tags.push(tag);
                        }
                        tags.set(updated_tags.clone());
                        save_tags(&updated_tags);
                        editing_tag.set(None);
                        show_form.set(false);
                    },
                    on_cancel: move |_| {
                        editing_tag.set(None);
                        show_form.set(false);
                    },
                }
            } else {
                div {
                    class: "tag-manager-content",
                    div {
                        class: "tag-manager-header",
                        Button {
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                editing_tag.set(None);
                                show_form.set(true);
                            },
                            {use_t("tags.new_tag")}
                        }
                    }

                    if tags().is_empty() {
                        EmptyState {
                            icon: "🏷️",
                            title: use_t("tags.empty"),
                            description: "",
                            action_text: use_t("tags.new_tag"),
                            on_action: move |_| {
                                editing_tag.set(None);
                                show_form.set(true);
                            },
                        }
                    } else {
                        div {
                            class: "tag-list",
                            for tag in tags().iter() {
                                TagItem {
                                    tag: tag.clone(),
                                    delete_confirm_id: delete_confirm_id,
                                    on_edit: move |t: Tag| {
                                        editing_tag.set(Some(t));
                                        show_form.set(true);
                                    },
                                    on_delete: move |tag_id: String| {
                                        let mut updated_tags = tags();
                                        updated_tags.retain(|t| t.id != tag_id);
                                        tags.set(updated_tags.clone());
                                        save_tags(&updated_tags);
                                        delete_confirm_id.set(None);
                                    },
                                    on_confirm_delete: move |tag_id: String| {
                                        delete_confirm_id.set(Some(tag_id));
                                    },
                                    on_cancel_delete: move |_| {
                                        delete_confirm_id.set(None);
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TagItem(
    tag: Tag,
    delete_confirm_id: Signal<Option<String>>,
    on_edit: EventHandler<Tag>,
    on_delete: EventHandler<String>,
    on_confirm_delete: EventHandler<String>,
    on_cancel_delete: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            key: "{tag.id}",
            class: "tag-item-wrapper",
            Card {
                class: "tag-item",
                CardContent {
                    div {
                        class: "tag-item-content",
                        {
                            let tag_id = tag.id.clone();
                            let tag_clone = tag.clone();
                            rsx! {
                                span {
                                    class: "tag-preview",
                                    style: format!("background-color: {};", tag.color),
                                    {tag.name.clone()}
                                }
                                div {
                                    class: "tag-item-actions",
                                    if delete_confirm_id() == Some(tag_id.clone()) {
                                        div {
                                            class: "delete-confirm-inline",
                                            p {
                                                class: "mb-2 text-sm",
                                                {use_t("tags.delete_confirm.message")}
                                            }
                                            div {
                                                class: "flex gap-2",
                                                Button {
                                                    variant: ButtonVariant::Ghost,
                                                    size: ButtonSize::Small,
                                                    onclick: move |_| on_cancel_delete.call(()),
                                                    {use_t("tags.cancel")}
                                                }
                                                Button {
                                                    variant: ButtonVariant::Danger,
                                                    size: ButtonSize::Small,
                                                    onclick: move |_| on_delete.call(tag_id.clone()),
                                                    {use_t("tags.delete")}
                                                }
                                            }
                                        }
                                    } else {
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Small,
                                            onclick: move |_| on_edit.call(tag_clone.clone()),
                                            {use_t("tags.edit")}
                                        }
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            size: ButtonSize::Small,
                                            onclick: move |_| on_confirm_delete.call(tag_id.clone()),
                                            {use_t("tags.delete")}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
