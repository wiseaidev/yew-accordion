
use accordion_rs::dioxus::{Accordion, Item, List};
use accordion_rs::{Align, Size};
use dioxus::prelude::*;
use dioxus_logger::tracing;

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        Home {}
    }
}

#[component]
pub fn Home() -> Element {
    let expand_0 = use_signal(|| false);
    let expand_1 = use_signal(|| false);
    let expand_2 = use_signal(|| false);
    let expand_3 = use_signal(|| false);
    let expand_4 = use_signal(|| false);
    let expand_5 = use_signal(|| false);
    let expand_6 = use_signal(|| true);
    let expand_7 = use_signal(|| false);
    let expand_8 = use_signal(|| false);
    let expand_9 = use_signal(|| false);

    let on_will_open = |_| tracing::info!("Accordion will open!");
    let on_did_open = |_| tracing::info!("Accordion did open!");
    let on_will_close = |_| tracing::info!("Accordion will close!");
    let on_did_close = |_| tracing::info!("Accordion did close!");

    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 {
                class: "text-3xl font-bold mb-8 text-white",
                "Accordion RS Dioxus Examples"
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                // Default Accordion
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Default Headless Accordion" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"Accordion  {{
    expand: expand_0,
    size: Size::Medium,
    expanded: rsx!{{ h3 {{ "Accordion Expanded" }} }},
    collapsed: rsx!{{ h3 {{ "Accordion Collapsed" }} }},
    List  {{
        Item  {{ align: Align::Left, "Item 1 Left Aligned" }} }},
        Item  {{ align: Align::Center, "Item 2 Centered" }} }},
    }},
}}"#
                    }
                    Accordion {
                        expand: expand_0,
                        size: Size::Medium,
                        expanded: rsx!{ h3 { "Accordion Expanded" }},
                        collapsed: rsx!{ h3 { "Accordion Collapsed" }},
                        List {
                            Item { align: Align::Left, "Item 1 Left Aligned" }
                            Item { align: Align::Center, "Item 2 Centered" }
                        }
                    }
                }

                // Accordion with Custom Styling
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Styled Accordion" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"Accordion  {{
    expand: expand_1,
    size: Size::Custom("20rem".into()),
    expanded: rsx!{{ h3 {{ class: "text-blue-500", "Styled Accordion Expanded" }} }},
    collapsed: rsx!{{ h3 {{ class: "text-red-500", "Styled Accordion Collapsed" }} }},
    class: "bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md",
    expanded_class: "bg-gray-800 text-white",
    collapsed_class: "bg-gray-900 text-gray-500",
    duration: 400,
    List  {{
        Item  {{ align: Align::Right, "Styled Item A" }},
        Item  {{ align: Align::Justify, "Styled Item B with custom align." }},
    }},
}}"#
                    }
                    Accordion {
                        expand: expand_1,
                        size: Size::Custom("20rem"),
                        expanded: rsx!{ h3 { class: "text-blue-500", "Styled Accordion Expanded" } },
                        collapsed: rsx!{ h3 { class: "text-red-500", "Styled Accordion Collapsed" } },
                        class: "bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md",
                        expanded_class: "bg-gray-800 text-white",
                        collapsed_class: "bg-gray-900 text-gray-500",
                        duration: 400,
                        List {
                            class: "list-disc pl-5",
                            Item { align: Align::Right, "Styled Item A" }
                            Item { align: Align::Justify, "Styled Item B with custom align." }
                        }
                    }
                }

                // Nested Accordions
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Nested Accordions" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"Accordion  {{
    expand: expand_2,
    size: Size::Large,
    expanded: rsx!{{ div {{ class: "text-white font-semibold", "What are nested accordions?" }} }},
    collapsed: rsx!{{ div {{ class: "text-white font-semibold", "What are nested accordions?" }} }},
    class: "bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md",
    List  {{
        Item  {{ "Nested accordions allow you to place one accordion inside another for organizing related content." }},
        Accordion  {{
            expand: expand_3,
            size: Size::Medium,
            expanded: rsx!{{ div {{ class: "text-white font-semibold", "Nested Item 1" }} }},
            collapsed: rsx!{{ div {{ class: "text-white font-semibold", "Nested Item 1" }} }},
            class: "bg-gray-800 border border-gray-600 p-3 rounded-md",
            List  {{
                Item  {{ class: "text-gray-400", "This is a nested item within the parent accordion." }},
            }},
        }},
    }},
}}"#
                    }
                    Accordion {
                        expand: expand_2,
                        size: Size::Large,
                        expanded: rsx!{div { class: "text-white font-semibold", "What are nested accordions?" }},
                        collapsed: rsx!{div { class: "text-white font-semibold", "What are nested accordions?" }},
                        class: "bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md",
                        List {
                            Item { "Nested accordions allow you to place one accordion inside another for organizing related content." }
                            Accordion {
                                expand: expand_3,
                                size: Size::Medium,
                                expanded: rsx!{div { class: "text-white font-semibold", "Nested Item 1" }},
                                collapsed: rsx!{div { class: "text-white font-semibold", "Nested Item 1" }},
                                class: "bg-gray-800 border border-gray-600 p-3 rounded-md",
                                List {
                                    Item { class: "text-gray-400", "This is a nested item within the parent accordion." }
                                }
                            }
                        }
                    }
                }
                // Accordion with Emojis
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Accordion with Emojis" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"Accordion {{
    expand: expand_4,
    size: Size::XLarge,
    expanded: rsx!{{ h3 {{ class: "text-white bg-gray-900 font-semibold", "Accordion with Emojis Expanded" }} }},
    collapsed: rsx!{{ h3 {{ class: "text-gray-400 bg-gray-900 font-semibold", "Accordion with Emojis Collapsed" }} }},
    class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
    expanded_class: "text-white bg-gray-800",
    collapsed_class: "text-gray-400 bg-gray-900",
    duration: 300,
    List {{
        Item {{ title: "Item with Icon", icon: "🔍", "Search content" }},
        Item {{ title: "Another Icon Item", icon: "📦", "Package content" }},
    }},
}}"#
                    }
                    Accordion {
                        expand: expand_4,
                        size: Size::XLarge,
                        expanded: rsx!{ h3 { class: "text-white bg-gray-900 font-semibold", "Accordion with Emojis Expanded" } },
                        collapsed: rsx!{ h3 { class: "text-gray-400 bg-gray-900 font-semibold", "Accordion with Emojis Collapsed" } },
                        class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
                        expanded_class: "text-white bg-gray-800",
                        collapsed_class: "text-gray-400 bg-gray-900",
                        duration: 300,
                        List {
                            Item { title: "Item with Icon", icon: "🔍", "Search content" }
                            Item { title: "Another Icon Item", icon: "📦", "Package content" }
                        }
                    }
                }

                // Accordion with Form Inputs
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Accordion with Form Inputs" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"Accordion {{
    expand: expand_5,
    size: Size::XLarge,
    expanded: rsx!{{ h3 {{ class: "text-white bg-gray-900 font-semibold", "Form Inputs Accordion Expanded" }} }},
    collapsed: rsx!{{ h3 {{ class: "text-gray-400 bg-gray-900 font-semibold", "Form Inputs Accordion Collapsed" }} }},
    class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
    expanded_class: "text-white bg-gray-800",
    collapsed_class: "text-gray-400 bg-gray-900",
    duration: 400,
    List {{
        Item {{
            title: "Form Item",
            form {{
                label {{ class: "block text-sm text-gray-300 mb-2", "Name: " }}
                input {{ class: "w-full p-2 border border-gray-600 rounded mb-4", type: "text", id: "name", name: "name" }}
                label {{ class: "block text-sm text-gray-300 mb-2", "Email: " }}
                input {{ class: "w-full p-2 border border-gray-600 rounded", type: "email", id: "email", name: "email" }}
            }}
        }},
    }},
}}"#
                    }
                    Accordion {
                        expand: expand_5,
                        size: Size::XLarge,
                        expanded: rsx!{ h3 { class: "text-white bg-gray-900 font-semibold", "Form Inputs Accordion Expanded" } },
                        collapsed: rsx!{ h3 { class: "text-gray-400 bg-gray-900 font-semibold", "Form Inputs Accordion Collapsed" } },
                        class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
                        expanded_class: "text-white bg-gray-800",
                        collapsed_class: "text-gray-400 bg-gray-900",
                        duration: 400,
                        List {
                            Item {
                                title: "Form Item",
                                form {
                                    label { class: "block text-sm text-gray-300 mb-2", "Name: " }
                                    input { class: "w-full p-2 border border-gray-600 rounded mb-4", type: "text", id: "name", name: "name" }
                                    label { class: "block text-sm text-gray-300 mb-2", "Email: " }
                                    input { class: "w-full p-2 border border-gray-600 rounded", type: "email", id: "email", name: "email" }
                                }
                            }
                        }
                    }
                }
                // FAQ Section
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 {
                        class: "text-xl font-bold mb-2",
                        "FAQs"
                    }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Accordion  {{
    expand: expand_6,
    size: Size::XLarge,
    expanded: rsx!{{ div {{ class: "text-white bg-gray-900 font-semibold", "What is Dioxus?" }} }},
    collapsed: rsx!{{ div {{ class: "text-gray-400 bg-gray-900 font-semibold", "What is Dioxus?" }} }},
    class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
    expanded_class: "text-white bg-gray-800",
    collapsed_class: "text-gray-400 bg-gray-900",
    List  {{
        Item  {{ "Dioxus is a modern Rust framework for building performant and declarative UI components." }},
    }},
}}
Accordion  {{
    expand: expand_7,
    size: Size::XLarge,
    expanded: rsx!{{ div {{ class: "text-white bg-gray-900 font-semibold", "Is Dioxus production ready?" }} }},
    collapsed: rsx!{{ div {{ class: "text-gray-400 bg-gray-900 font-semibold", "Is Dioxus production ready?" }} }},
    class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
    expanded_class: "text-white bg-gray-800",
    collapsed_class: "text-gray-400 bg-gray-900",
    List  {{
        Item  {{ "Yes, Dioxus is production-ready and offers excellent performance with Rust and WebAssembly." }},
    }},
}}"##
                    }
                    Accordion {
                        expand: expand_6,
                        size: Size::XLarge,
                        expanded: rsx!{div { class: "text-white bg-gray-900 font-semibold", "What is Dioxus?" }},
                        collapsed: rsx!{div { class: "text-gray-400 bg-gray-900 font-semibold", "What is Dioxus?" }},
                        class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
                        expanded_class: "text-white bg-gray-800",
                        collapsed_class: "text-gray-400 bg-gray-900",
                        List {
                            Item {
                                "Dioxus is a modern Rust framework for building performant and declarative UI components."
                            }
                        }
                    }
                    Accordion {
                        expand: expand_7,
                        size: Size::XLarge,
                        expanded: rsx!{div { class: "text-white bg-gray-900 font-semibold", "Is Dioxus production ready?" }},
                        collapsed: rsx!{div { class: "text-gray-400 bg-gray-900 font-semibold", "Is Dioxus production ready?" }},
                        class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
                        expanded_class: "text-white bg-gray-800",
                        collapsed_class: "text-gray-400 bg-gray-900",
                        List {
                            Item {
                                "Yes, Dioxus is production-ready and offers excellent performance with Rust and WebAssembly."
                            }
                        }
                    }
                }
                // Accordion with Callback Props
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 {
                        class: "text-xl font-bold mb-2",
                        "Accordion with Callbacks"
                    }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Accordion  {{
    expand: expand_8,
    size: Size::XLarge,
    expanded: rsx!{{ h3 {{ "Accordion Expanded" }} }},
    collapsed: rsx!{{ h3 {{ "Accordion Collapsed" }} }},
    class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
    expanded_class: "text-white bg-gray-800",
    collapsed_class: "text-gray-400 bg-gray-800",
    did_open: on_did_open,
    will_open: on_will_open,
    did_close: on_did_close,
    will_close: on_will_close,
    List  {{
        Item  {{ align: Align::Left, "Item 1 - Left" }},
        Item  {{ align: Align::Right, "Item 2 - Right" }},
    }},
}}"##
                    }
                    Accordion {
                        expand: expand_8,
                        size: Size::XLarge,
                        expanded: rsx!{h3 { "Accordion Expanded" }},
                        collapsed: rsx!{h3 { "Accordion Collapsed" }},
                        class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
                        expanded_class: "text-white bg-gray-800",
                        collapsed_class: "text-gray-400 bg-gray-800",
                        did_open: on_did_open,
                        will_open: on_will_open,
                        did_close: on_did_close,
                        will_close: on_will_close,
                        List {
                            Item { align: Align::Left, "Item 1 - Left" }
                            Item { align: Align::Right, "Item 2 - Right" }
                        }
                    }
                }
                // Accordion with Logging
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 {
                        class: "text-xl font-bold mb-2",
                        "Accordion with Logging (Press F12)"
                    }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Accordion  {{
    expand: expand_9,
    size: Size::XLarge,
    expanded: rsx!{{ h3 {{ "Log Accordion Expanded" }} }},
    collapsed: rsx!{{ h3 {{ "Log Accordion Collapsed" }} }},
    class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
    expanded_class: "text-white bg-gray-800",
    collapsed_class: "text-gray-400 bg-gray-800",
    did_open: |_| tracing::info!("Log: Accordion did open!"),
    will_close: |_| tracing::info!("Log: Accordion will close!"),
    List  {{
        Item  {{ "This accordion logs open and close actions." }},
    }},
}}"##
                    }
                    Accordion {
                        expand: expand_9,
                        size: Size::XLarge,
                        expanded: rsx!{h3 { "Log Accordion Expanded" }},
                        collapsed: rsx!{h3 { "Log Accordion Collapsed" }},
                        class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
                        expanded_class: "text-white bg-gray-800",
                        collapsed_class: "text-gray-400 bg-gray-800",
                        did_open: |_| tracing::info!("Log: Accordion did open!"),
                        will_close: |_| tracing::info!("Log: Accordion will close!"),
                        List {
                            Item {
                                "This accordion logs open and close actions."
                            }
                        }
                    }
                }
            }
        }
    }
}