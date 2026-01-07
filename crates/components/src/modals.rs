use dioxus::prelude::*;
use remind_me_ui::{Button, ButtonVariant, Modal, ModalSize};
#[cfg(target_arch = "wasm32")]
use crate::i18n::use_t;

#[component]
pub fn DeleteConfirmModal(
    open: Signal<Option<String>>,
    reminder_id: String,
    on_confirm: EventHandler<String>,
    on_cancel: EventHandler<()>,
) -> Element {
    let is_open = open().is_some();
    let mut open_signal = use_signal(|| is_open);

    // Update signal when open changes using use_effect
    use_effect(move || {
        let current_is_open = open().is_some();
        if current_is_open != open_signal() {
            open_signal.set(current_is_open);
        }
    });

    rsx! {
        Modal {
            open: open_signal,
            size: ModalSize::Small,
            title: use_t("delete.title"),
            close_on_backdrop: true,
            on_close: move |_| on_cancel.call(()),
            p {
                class: "mb-4",
                {use_t("delete.message")}
            }
            div {
                class: "flex justify-end gap-2",
                Button {
                    variant: ButtonVariant::Ghost,
                    onclick: move |_| on_cancel.call(()),
                    {use_t("delete.cancel")}
                }
                Button {
                    variant: ButtonVariant::Danger,
                    onclick: move |_| on_confirm.call(reminder_id.clone()),
                    {use_t("delete.confirm")}
                }
            }
        }
    }
}
