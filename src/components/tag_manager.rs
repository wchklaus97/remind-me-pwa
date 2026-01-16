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

#[cfg(target_arch = "wasm32")]
fn resolve_tag_drop_target(x: f64, y: f64) -> Option<String> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let element = document.element_from_point(x, y)?;
    if let Ok(Some(wrapper)) = element.closest(".tag-item-wrapper") {
        return wrapper.get_attribute("data-tag-id");
    }
    None
}

#[component]
pub fn TagManagerPanel() -> Element {
    let mut tags = use_signal(load_tags);
    let mut dragged_tag_id = use_signal(|| None::<String>);
    let mut drop_target_id = use_signal(|| None::<String>);
    let mut editing_tag = use_signal(|| None::<Tag>);
    let mut show_form = use_signal(|| false);
    let mut delete_confirm_id = use_signal(|| None::<String>);

    rsx! {
        section {
            class: "tag-manager-panel",
            aria_label: use_t("tags.title"),
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
                        class: "tag-manager-intro",
                        p {
                            class: "tag-manager-subtitle",
                            {use_t("tags.description")}
                        }
                        p {
                            class: "tag-manager-hint",
                            {use_t("tags.reorder_hint")}
                        }
                    }
                    div {
                        class: "tag-manager-header",
                        Button {
                            variant: ButtonVariant::Primary,
                            class: "btn btn-primary".to_string(),
                            aria_label: Some(use_t("tags.new_tag")),
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
                            for (index, tag) in tags().iter().enumerate() {
                                TagItem {
                                    tag: tag.clone(),
                                    index: index,
                                    total: tags().len(),
                                    delete_confirm_id: delete_confirm_id,
                                    dragged_tag_id: dragged_tag_id,
                                    drop_target_id: drop_target_id,
                                    on_edit: move |t: Tag| {
                                        editing_tag.set(Some(t));
                                        show_form.set(true);
                                    },
                                    on_reorder: move |(drag_id, target_id): (String, String)| {
                                        let mut updated_tags = tags();
                                        let from_index = updated_tags.iter().position(|t| t.id == drag_id);
                                        let to_index = updated_tags.iter().position(|t| t.id == target_id);

                                        if let (Some(from), Some(to)) = (from_index, to_index) {
                                            if from != to {
                                                let tag = updated_tags.remove(from);
                                                let insert_index = if from < to { to.saturating_sub(1) } else { to };
                                                updated_tags.insert(insert_index, tag);
                                                tags.set(updated_tags.clone());
                                                save_tags(&updated_tags);
                                            }
                                        }
                                    },
                                    on_move: move |(from_index, to_index): (usize, usize)| {
                                        let mut updated_tags = tags();
                                        if from_index < updated_tags.len() && to_index < updated_tags.len() {
                                            let tag = updated_tags.remove(from_index);
                                            updated_tags.insert(to_index, tag);
                                            tags.set(updated_tags.clone());
                                            save_tags(&updated_tags);
                                        }
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
pub fn TagManager(
    open: Signal<bool>,
    on_close: EventHandler<()>,
) -> Element {
    let mut tags = use_signal(load_tags);
    let mut dragged_tag_id = use_signal(|| None::<String>);
    let mut drop_target_id = use_signal(|| None::<String>);
    let mut editing_tag = use_signal(|| None::<Tag>);
    let mut show_form = use_signal(|| false);
    let mut delete_confirm_id = use_signal(|| None::<String>);

    // Reload tags when modal opens
    use_effect(move || {
        if open() {
            tags.set(load_tags());
                dragged_tag_id.set(None);
                drop_target_id.set(None);
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
                        class: "tag-manager-intro",
                        p {
                            class: "tag-manager-subtitle",
                            {use_t("tags.description")}
                        }
                        p {
                            class: "tag-manager-hint",
                            {use_t("tags.reorder_hint")}
                        }
                    }
                    div {
                        class: "tag-manager-header",
                        Button {
                            variant: ButtonVariant::Primary,
                            class: "btn btn-primary".to_string(),
                            aria_label: Some(use_t("tags.new_tag")),
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
                            for (index, tag) in tags().iter().enumerate() {
                                TagItem {
                                    tag: tag.clone(),
                                    index: index,
                                    total: tags().len(),
                                    delete_confirm_id: delete_confirm_id,
                                    dragged_tag_id: dragged_tag_id,
                                    drop_target_id: drop_target_id,
                                    on_edit: move |t: Tag| {
                                        editing_tag.set(Some(t));
                                        show_form.set(true);
                                    },
                                    on_reorder: move |(drag_id, target_id): (String, String)| {
                                        let mut updated_tags = tags();
                                        let from_index = updated_tags.iter().position(|t| t.id == drag_id);
                                        let to_index = updated_tags.iter().position(|t| t.id == target_id);

                                        if let (Some(from), Some(to)) = (from_index, to_index) {
                                            if from != to {
                                                let tag = updated_tags.remove(from);
                                                let insert_index = if from < to { to.saturating_sub(1) } else { to };
                                                updated_tags.insert(insert_index, tag);
                                                tags.set(updated_tags.clone());
                                                save_tags(&updated_tags);
                                            }
                                        }
                                    },
                                    on_move: move |(from_index, to_index): (usize, usize)| {
                                        let mut updated_tags = tags();
                                        if from_index < updated_tags.len() && to_index < updated_tags.len() {
                                            let tag = updated_tags.remove(from_index);
                                            updated_tags.insert(to_index, tag);
                                            tags.set(updated_tags.clone());
                                            save_tags(&updated_tags);
                                        }
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
    index: usize,
    total: usize,
    delete_confirm_id: Signal<Option<String>>,
    dragged_tag_id: Signal<Option<String>>,
    drop_target_id: Signal<Option<String>>,
    on_edit: EventHandler<Tag>,
    on_reorder: EventHandler<(String, String)>,
    on_move: EventHandler<(usize, usize)>,
    on_delete: EventHandler<String>,
    on_confirm_delete: EventHandler<String>,
    on_cancel_delete: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            key: "{tag.id}",
            class: {
                let mut class_name = "tag-item-wrapper".to_string();
                if dragged_tag_id().as_ref() == Some(&tag.id) {
                    class_name.push_str(" is-dragging");
                }
                if drop_target_id().as_ref() == Some(&tag.id) {
                    class_name.push_str(" is-drop-target");
                }
                class_name
            },
            draggable: "true",
            tabindex: "0",
            aria_label: "{use_t("tags.reorder_hint")}",
            data_tag_id: "{tag.id}",
            ondragstart: move |_| {
                dragged_tag_id.set(Some(tag.id.clone()));
                drop_target_id.set(Some(tag.id.clone()));
            },
            onpointerdown: move |event| {
                if event.pointer_type() != "touch" {
                    return;
                }
                event.prevent_default();
                dragged_tag_id.set(Some(tag.id.clone()));
                drop_target_id.set(Some(tag.id.clone()));
            },
            onpointermove: move |event| {
                if event.pointer_type() != "touch" {
                    return;
                }
                event.prevent_default();
                #[cfg(target_arch = "wasm32")]
                {
                    let coords = event.client_coordinates();
                    if let Some(target_id) = resolve_tag_drop_target(coords.x as f64, coords.y as f64) {
                        drop_target_id.set(Some(target_id));
                    }
                }
            },
            onpointerup: move |event| {
                if event.pointer_type() != "touch" {
                    return;
                }
                event.prevent_default();
                let drag_id = dragged_tag_id();
                let target_id = drop_target_id();
                if let (Some(drag), Some(target)) = (drag_id, target_id) {
                    if drag != target {
                        on_reorder.call((drag.clone(), target.clone()));
                    }
                }
                dragged_tag_id.set(None);
                drop_target_id.set(None);
            },
            onkeydown: move |event| {
                let key = event.key();
                if key == "ArrowUp" && index > 0 {
                    event.prevent_default();
                    on_move.call((index, index - 1));
                } else if key == "ArrowDown" && index + 1 < total {
                    event.prevent_default();
                    on_move.call((index, index + 1));
                }
            },
            ondragover: move |event| {
                event.prevent_default();
                drop_target_id.set(Some(tag.id.clone()));
            },
            ondrop: move |event| {
                event.prevent_default();
                if let Some(drag_id) = dragged_tag_id() {
                    on_reorder.call((drag_id.clone(), tag.id.clone()));
                }
                dragged_tag_id.set(None);
                drop_target_id.set(None);
            },
            ondragend: move |_| {
                dragged_tag_id.set(None);
                drop_target_id.set(None);
            },
            Card {
                class: "tag-item tag-item-card",
                CardContent {
                    div {
                        class: "tag-item-content",
                        {
                            let tag_id = tag.id.clone();
                            let tag_clone = tag.clone();
                            rsx! {
                                span {
                                    class: "tag-drag-handle",
                                    aria_label: use_t("tags.drag_handle"),
                                    "⋮⋮"
                                }
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
                                                    class: "btn btn-ghost".to_string(),
                                                    aria_label: Some(use_t("tags.cancel")),
                                                    onclick: move |_| on_cancel_delete.call(()),
                                                    {use_t("tags.cancel")}
                                                }
                                                Button {
                                                    variant: ButtonVariant::Danger,
                                                    size: ButtonSize::Small,
                                                    class: "btn btn-danger".to_string(),
                                                    aria_label: Some(use_t("tags.delete")),
                                                    onclick: move |_| on_delete.call(tag_id.clone()),
                                                    {use_t("tags.delete")}
                                                }
                                            }
                                        }
                                    } else {
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Small,
                                            class: "btn btn-ghost".to_string(),
                                            aria_label: Some(use_t("tags.edit")),
                                            onclick: move |_| on_edit.call(tag_clone.clone()),
                                            {use_t("tags.edit")}
                                        }
                                        Button {
                                            variant: ButtonVariant::Danger,
                                            size: ButtonSize::Small,
                                            class: "btn btn-danger".to_string(),
                                            aria_label: Some(use_t("tags.delete")),
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
