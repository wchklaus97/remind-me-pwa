use dioxus::prelude::*;
use crate::i18n::use_t;

#[component]
pub fn TestimonialsSection() -> Element {
    const INITIALS: [&str; 30] = [
        "AT", "MG", "DP", "JS", "AK", "LR", "CW", "SB", "DM", "JK",
        "PT", "RW", "KL", "MN", "OP", "QR", "ST", "UV", "WX", "YZ",
        "AB", "CD", "EF", "GH", "IJ", "LM", "NO", "PQ", "RS", "TU"
    ];
    
    fn get_avatar_class(index: usize) -> &'static str {
        match index % 3 {
            0 => "at",
            1 => "mg",
            _ => "dp",
        }
    }
    
    rsx! {
        section {
            class: "testimonials-section",
            h2 { class: "section-title", {use_t("landing.sections.testimonials.title")} }
            p { class: "section-subtitle", {use_t("landing.sections.testimonials.subtitle")} }
            div { class: "testimonials-carousel-container",
                // Row 1: Right to Left
                div { class: "testimonials-carousel-row row-1",
                    div { class: "testimonials-carousel-track",
                        // First set of 30 testimonials
                        for i in 0..30 {
                            div {
                                key: "r1-{i}",
                                class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t(&format!("landing.sections.testimonials.items.{}.quote", i))} }
                                div { class: "person",
                                    div { 
                                        class: format!("person-avatar {}", get_avatar_class(i)),
                                        {INITIALS[i]}
                                    }
                                    div { class: "person-info",
                                        div { class: "person-name", {use_t(&format!("landing.sections.testimonials.items.{}.name", i))} }
                                        div { class: "person-role", {use_t(&format!("landing.sections.testimonials.items.{}.role", i))} }
                                    }
                                }
                            }
                        }
                        // Duplicate set for seamless loop
                        for i in 0..30 {
                            div {
                                key: "r1-dup-{i}",
                                class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t(&format!("landing.sections.testimonials.items.{}.quote", i))} }
                                div { class: "person",
                                    div { 
                                        class: format!("person-avatar {}", get_avatar_class(i)),
                                        {INITIALS[i]}
                                    }
                                    div { class: "person-info",
                                        div { class: "person-name", {use_t(&format!("landing.sections.testimonials.items.{}.name", i))} }
                                        div { class: "person-role", {use_t(&format!("landing.sections.testimonials.items.{}.role", i))} }
                                    }
                                }
                            }
                        }
                    }
                }
                // Row 2: Left to Right
                div { class: "testimonials-carousel-row row-2",
                    div { class: "testimonials-carousel-track",
                        // First set of 30 testimonials (reversed order for visual variety)
                        for i in (0..30).rev() {
                            div {
                                key: "r2-{i}",
                                class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t(&format!("landing.sections.testimonials.items.{}.quote", i))} }
                                div { class: "person",
                                    div { 
                                        class: format!("person-avatar {}", get_avatar_class(i)),
                                        {INITIALS[i]}
                                    }
                                    div { class: "person-info",
                                        div { class: "person-name", {use_t(&format!("landing.sections.testimonials.items.{}.name", i))} }
                                        div { class: "person-role", {use_t(&format!("landing.sections.testimonials.items.{}.role", i))} }
                                    }
                                }
                            }
                        }
                        // Duplicate set for seamless loop
                        for i in (0..30).rev() {
                            div {
                                key: "r2-dup-{i}",
                                class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t(&format!("landing.sections.testimonials.items.{}.quote", i))} }
                                div { class: "person",
                                    div { 
                                        class: format!("person-avatar {}", get_avatar_class(i)),
                                        {INITIALS[i]}
                                    }
                                    div { class: "person-info",
                                        div { class: "person-name", {use_t(&format!("landing.sections.testimonials.items.{}.name", i))} }
                                        div { class: "person-role", {use_t(&format!("landing.sections.testimonials.items.{}.role", i))} }
                                    }
                                }
                            }
                        }
                    }
                }
                // Row 3: Right to Left
                div { class: "testimonials-carousel-row row-3",
                    div { class: "testimonials-carousel-track",
                        // First set of 30 testimonials
                        for i in 0..30 {
                            div {
                                key: "r3-{i}",
                                class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t(&format!("landing.sections.testimonials.items.{}.quote", i))} }
                                div { class: "person",
                                    div { 
                                        class: format!("person-avatar {}", get_avatar_class(i)),
                                        {INITIALS[i]}
                                    }
                                    div { class: "person-info",
                                        div { class: "person-name", {use_t(&format!("landing.sections.testimonials.items.{}.name", i))} }
                                        div { class: "person-role", {use_t(&format!("landing.sections.testimonials.items.{}.role", i))} }
                                    }
                                }
                            }
                        }
                        // Duplicate set for seamless loop
                        for i in 0..30 {
                            div {
                                key: "r3-dup-{i}",
                                class: "testimonial-card",
                                div { class: "stars", "★★★★★" }
                                p { class: "quote", {use_t(&format!("landing.sections.testimonials.items.{}.quote", i))} }
                                div { class: "person",
                                    div { 
                                        class: format!("person-avatar {}", get_avatar_class(i)),
                                        {INITIALS[i]}
                                    }
                                    div { class: "person-info",
                                        div { class: "person-name", {use_t(&format!("landing.sections.testimonials.items.{}.name", i))} }
                                        div { class: "person-role", {use_t(&format!("landing.sections.testimonials.items.{}.role", i))} }
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

