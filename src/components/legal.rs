use dioxus::prelude::*;

use crate::router::Route;
use crate::components::page_template::PublicPageTemplate;
use crate::i18n::use_t;

#[component]
pub fn PrivacyPolicyPage(
    on_enter_app: EventHandler<()>,
    on_jump: EventHandler<&'static str>,
    on_navigate: EventHandler<Route>,
) -> Element {
    rsx! {
        PublicPageTemplate {
            title: use_t("legal.privacy.title"),
            active_section: "".to_string(),
            on_enter_app,
            on_jump,
            on_navigate,
            div {
                class: "legal-prose",
                p { class: "legal-updated", {use_t("legal.privacy.last_updated")} }

                h3 { {use_t("legal.privacy.summary.title")} }
                ul {
                    li { strong { {use_t("legal.privacy.summary.no_data_collection.label")} } " " {use_t("legal.privacy.summary.no_data_collection.text")} }
                    li { strong { {use_t("legal.privacy.summary.local_storage.label")} } " " {use_t("legal.privacy.summary.local_storage.text")} }
                    li { strong { {use_t("legal.privacy.summary.offline_first.label")} } " " {use_t("legal.privacy.summary.offline_first.text")} }
                }

                h3 { {use_t("legal.privacy.definitions.title")} }
                ul {
                    li { strong { {use_t("legal.privacy.definitions.app.label")} } " " {use_t("legal.privacy.definitions.app.text")} }
                    li { strong { {use_t("legal.privacy.definitions.device.label")} } " " {use_t("legal.privacy.definitions.device.text")} }
                    li { strong { {use_t("legal.privacy.definitions.local_storage.label")} } " " {use_t("legal.privacy.definitions.local_storage.text")} }
                }

                h3 { {use_t("legal.privacy.introduction.title")} }
                p { {use_t("legal.privacy.introduction.text")} }

                h3 { {use_t("legal.privacy.data_collection.title")} }
                p {
                    strong { {use_t("legal.privacy.data_collection.label")} }
                    " " {use_t("legal.privacy.data_collection.text")}
                }

                h3 { {use_t("legal.privacy.local_storage.title")} }
                p { {use_t("legal.privacy.local_storage.intro")} }
                ul {
                    li { {use_t("legal.privacy.local_storage.items.reminders")} }
                    li { {use_t("legal.privacy.local_storage.items.preferences")} }
                    li { {use_t("legal.privacy.local_storage.items.filter")} }
                }
                p { {use_t("legal.privacy.local_storage.note")} }

                h3 { {use_t("legal.privacy.information_you_provide.title")} }
                p { {use_t("legal.privacy.information_you_provide.text")} }

                h3 { {use_t("legal.privacy.network_requests.title")} }
                p { {use_t("legal.privacy.network_requests.text")} }

                h3 { {use_t("legal.privacy.no_account_required.title")} }
                p { {use_t("legal.privacy.no_account_required.text")} }

                h3 { {use_t("legal.privacy.third_party_services.title")} }
                p { {use_t("legal.privacy.third_party_services.text")} }

                h3 { {use_t("legal.privacy.data_retention.title")} }
                p { {use_t("legal.privacy.data_retention.text")} }

                h3 { {use_t("legal.privacy.security.title")} }
                p { {use_t("legal.privacy.security.text")} }

                h3 { {use_t("legal.privacy.children_privacy.title")} }
                p { {use_t("legal.privacy.children_privacy.text")} }

                h3 { {use_t("legal.privacy.international_users.title")} }
                p { {use_t("legal.privacy.international_users.text")} }

                h3 { {use_t("legal.privacy.open_source.title")} }
                p { {use_t("legal.privacy.open_source.text")} }
                ul { class: "legal-links",
                    li {
                        a {
                            class: "legal-link",
                            href: "https://github.com/wchklaus97/remind-me-pwa",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: use_t("legal.privacy.open_source.github_aria"),
                            {use_t("legal.privacy.open_source.github_link")}
                        }
                    }
                }

                h3 { {use_t("legal.privacy.changes.title")} }
                p { {use_t("legal.privacy.changes.text")} }

                h3 { {use_t("legal.privacy.contact.title")} }
                p { {use_t("legal.privacy.contact.text")} }
                ul { class: "legal-links",
                    li {
                        a {
                            class: "legal-link",
                            href: "https://github.com/wchklaus97/remind-me-pwa/issues",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: use_t("legal.privacy.contact.github_aria"),
                            {use_t("legal.privacy.contact.github_link")}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn TermsOfUsePage(
    on_enter_app: EventHandler<()>,
    on_jump: EventHandler<&'static str>,
    on_navigate: EventHandler<Route>,
) -> Element {
    rsx! {
        PublicPageTemplate {
            title: use_t("legal.terms.title"),
            active_section: "".to_string(),
            on_enter_app,
            on_jump,
            on_navigate,
            div {
                class: "legal-prose",
                p { class: "legal-updated", {use_t("legal.terms.last_updated")} }

                h3 { {use_t("legal.terms.summary.title")} }
                ul {
                    li { {use_t("legal.terms.summary.free_opensource")} }
                    li { {use_t("legal.terms.summary.data_control")} }
                    li { {use_t("legal.terms.summary.no_warranty")} }
                }

                h3 { {use_t("legal.terms.acceptance.title")} }
                p { {use_t("legal.terms.acceptance.text")} }

                h3 { {use_t("legal.terms.eligibility.title")} }
                p { {use_t("legal.terms.eligibility.text")} }

                h3 { {use_t("legal.terms.description.title")} }
                p { {use_t("legal.terms.description.text")} }

                h3 { {use_t("legal.terms.license.title")} }
                p { {use_t("legal.terms.license.intro")} }
                ul {
                    li { {use_t("legal.terms.license.use_any_purpose")} }
                    li { {use_t("legal.terms.license.copy_modify")} }
                    li { {use_t("legal.terms.license.distribute")} }
                    li { {use_t("legal.terms.license.commercial")} }
                }
                p { {use_t("legal.terms.license.related_links")} }
                ul { class: "legal-links",
                    li {
                        a {
                            class: "legal-link",
                            href: "https://opensource.org/license/mit/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: use_t("legal.terms.license.mit_aria"),
                            {use_t("legal.terms.license.mit_link")}
                        }
                    }
                    li {
                        a {
                            class: "legal-link",
                            href: "https://github.com/wchklaus97/remind-me-pwa",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: use_t("legal.terms.license.github_aria"),
                            {use_t("legal.terms.license.github_link")}
                        }
                    }
                }

                h3 { {use_t("legal.terms.user_responsibilities.title")} }
                p { {use_t("legal.terms.user_responsibilities.intro")} }
                ul {
                    li { {use_t("legal.terms.user_responsibilities.backup")} }
                    li { {use_t("legal.terms.user_responsibilities.laws")} }
                    li { {use_t("legal.terms.user_responsibilities.security")} }
                }

                h3 { {use_t("legal.terms.acceptable_use.title")} }
                p { {use_t("legal.terms.acceptable_use.text")} }

                h3 { {use_t("legal.terms.data_storage.title")} }
                p { {use_t("legal.terms.data_storage.text")} }

                h3 { {use_t("legal.terms.intellectual_property.title")} }
                p { {use_t("legal.terms.intellectual_property.text")} }

                h3 { {use_t("legal.terms.disclaimer.title")} }
                p { {use_t("legal.terms.disclaimer.text")} }

                h3 { {use_t("legal.terms.limitation.title")} }
                p { {use_t("legal.terms.limitation.text")} }

                h3 { {use_t("legal.terms.termination.title")} }
                p { {use_t("legal.terms.termination.text")} }

                h3 { {use_t("legal.terms.governing_law.title")} }
                p { {use_t("legal.terms.governing_law.text")} }

                h3 { {use_t("legal.terms.changes.title")} }
                p { {use_t("legal.terms.changes.text")} }

                h3 { {use_t("legal.terms.contact.title")} }
                p { {use_t("legal.terms.contact.text")} }
                ul { class: "legal-links",
                    li {
                        a {
                            class: "legal-link",
                            href: "https://github.com/wchklaus97/remind-me-pwa/issues",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            aria_label: use_t("legal.terms.contact.github_aria"),
                            {use_t("legal.terms.contact.github_link")}
                        }
                    }
                }
            }
        }
    }
}


