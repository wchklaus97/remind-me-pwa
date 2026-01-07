use dioxus::prelude::*;
use remind_me_ui::{
    Button, ButtonVariant,
    Card, CardContent, CardHeader, CardTitle,
    FormField, Input,
};
use remind_me_shared::models::Tag;
#[cfg(target_arch = "wasm32")]
use crate::i18n::use_t;
use remind_me_shared::utils::now_timestamp_millis;

// Preset colors for tags
const PRESET_COLORS: &[&str] = &[
    "#FA8A59", // Crab Coral
    "#F4D273", // Bell Gold
    "#6A7CED", // Brand Blue
    "#9E75E9", // Brand Purple
    "#FF6B6B", // Danger Red
    "#51CF66", // Green
    "#4DABF7", // Sky Blue
    "#FFA94D", // Orange
    "#FFD43B", // Yellow
    "#C5F6FA", // Cyan
    "#FFE066", // Light Yellow
    "#D0BFFF", // Light Purple
];

#[component]
pub fn TagForm(
    tag: Option<Tag>,
    on_save: EventHandler<Tag>,
    on_cancel: EventHandler<()>,
) -> Element {
    let tag_id = tag.as_ref().map(|t| t.id.clone());
    let mut name = use_signal(|| tag.as_ref().map(|t| t.name.clone()).unwrap_or_default());
    let mut color = use_signal(|| tag.as_ref().map(|t| t.color.clone()).unwrap_or_else(|| PRESET_COLORS[0].to_string()));

    rsx! {
        Card {
            class: "tag-form",
            header: rsx! {
                CardHeader {
                    CardTitle {
                        if tag.is_some() {
                            {use_t("tags.edit_tag")}
                        } else {
                            {use_t("tags.new_tag")}
                        }
                    }
                }
            },
            CardContent {
                FormField {
                    id: "tag_name".to_string(),
                    name: "name".to_string(),
                    label: use_t("tags.name.label"),
                    required: true,
                    Input {
                        id: "tag_name".to_string(),
                        name: "name".to_string(),
                        r#type: "text",
                        placeholder: use_t("tags.name.placeholder"),
                        value: "{name()}",
                        oninput: move |value| name.set(value),
                    }
                }

                FormField {
                    id: "tag_color".to_string(),
                    name: "color".to_string(),
                    label: use_t("tags.color.label"),
                    div {
                        class: "tag-color-picker",
                        div {
                            class: "color-options",
                            for (idx, color_option) in PRESET_COLORS.iter().enumerate() {
                                button {
                                    class: if color() == *color_option { "color-option selected" } else { "color-option" },
                                    style: format!("background-color: {};", color_option),
                                    onclick: move |_| color.set(color_option.to_string()),
                                    aria_label: format!("Select color {}", idx + 1),
                                    span {
                                        class: "color-check",
                                        "✓"
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
                        onclick: move |_| on_cancel.call(()),
                        {use_t("tags.cancel")}
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        disabled: name().is_empty(),
                        onclick: {
                            let tag_id_clone = tag_id.clone();
                            move |_| {
                                if !name().is_empty() {
                                    let new_tag = Tag {
                                        id: tag_id_clone.clone().unwrap_or_else(|| format!("tag_{}", now_timestamp_millis())),
                                        name: name(),
                                        color: color(),
                                    };
                                    on_save.call(new_tag);
                                }
                            }
                        },
                        if tag.is_some() {
                            {use_t("tags.save")}
                        } else {
                            {use_t("tags.add")}
                        }
                    }
                }
            }
        }
    }
}

