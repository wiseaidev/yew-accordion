use accordion_rs::leptos::{Accordion, Item, List};
use accordion_rs::{Align, Size};
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Home />
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let expand_0 = signal(false);
    let expand_1 = signal(false);
    let expand_2 = signal(false);
    let expand_3 = signal(false);
    let expand_4 = signal(false);
    let expand_5 = signal(false);
    let expand_6 = signal(true);
    let expand_7 = signal(false);
    let expand_8 = signal(false);

    let on_will_open = Callback::from(move || log!("Accordion will open!"));
    let on_did_open = Callback::from(move || log!("Accordion did open!"));
    let on_will_close = Callback::from(move || log!("Accordion will close!"));
    let on_did_close = Callback::from(move || log!("Accordion did close!"));

    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Accordion RS Leptos Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">

                // Default Accordion
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Default Headless Accordion" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expanded}
    size={Size::Medium}
    expanded={Box::new(|| view! { <h3>"Accordion Expanded"</h3> })}
    collapsed={Box::new(|| view! { <h3>"Accordion Collapsed"</h3> })}
>
    <List>
        <Item align={Align::Left}>
            "Item 1 Left Aligned"
        </Item>
        <Item align={Align::Center}>
            "Item 2 Centered"
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_0}
                        size=Size::Medium
                        expanded=Box::new(|| view! { <h3>"Accordion Expanded"</h3> }.into_any())
                        collapsed=Box::new(|| view! { <h3>"Accordion Collapsed"</h3> }.into_any())
                    >
                        <List>
                            <Item align=Align::Left>"Item 1 Left Aligned"</Item>
                            <Item align=Align::Center>"Item 2 Centered"</Item>
                        </List>
                    </Accordion>
                </div>

                // Accordion with Custom Styling
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Styled Accordion" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expanded}
    size={Size::Custom("20rem")}
    expanded={Box::new(|| view! { <h3 class="text-blue-500">"Styled Accordion Expanded"</h3> })}
    collapsed={Box::new(|| view! { <h3 class="text-red-500">"Styled Accordion Collapsed"</h3> })}
    class="bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md"
    expanded_class="bg-gray-800 text-white"
    collapsed_class="bg-gray-900 text-gray-500"
    duration={400}
>
    <List class="list-disc pl-5">
        <Item align={Align::Right}>
            "Styled Item A"
        </Item>
        <Item align={Align::Justify}>
            "Styled Item B with custom align."
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_1}
                        size=Size::Custom("20rem")
                        expanded=Box::new(|| view! { <h3 class="text-blue-500">"Styled Accordion Expanded"</h3> }.into_any())
                        collapsed=Box::new(|| view! { <h3 class="text-red-500">"Styled Accordion Collapsed"</h3> }.into_any())
                        class="bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md"
                        expanded_class="bg-gray-800 text-white"
                        collapsed_class="bg-gray-900 text-gray-500"
                        duration=400
                    >
                        <List class="list-disc pl-5">
                            <Item align=Align::Right>"Styled Item A"</Item>
                            <Item align=Align::Justify>"Styled Item B with custom align."</Item>
                        </List>
                    </Accordion>
                </div>

                // Nested Accordions
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Nested Accordions" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r##"<Accordion
    expand={expanded_1}
    size={Size::Large}
    expanded={Box::new(|| view! { <div class="text-white font-semibold">"What are nested accordions?"</div> })}
    collapsed={Box::new(|| view! { <div class="text-white font-semibold">"What are nested accordions?"</div> })}
    class="bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md"
>
    <List>
        <Item>
            "Nested accordions allow you to place one accordion inside another for organizing related content."
        </Item>
        <Accordion
            expand={expanded_2}
            size={Size::Medium}
            expanded={Box::new(|| view! { <div class="text-white font-semibold">"Nested Item 1"</div> })}
            collapsed={Box::new(|| view! { <div class="text-white font-semibold">"Nested Item 1"</div> })}
            class="bg-gray-800 border border-gray-600 p-3 rounded-md"
        >
            <List>
                <Item>
                    "This is a nested item within the parent accordion."
                </Item>
            </List>
        </Accordion>
    </List>
</Accordion>"## }
                    </pre>
                    <Accordion
                        expand={expand_2}
                        size=Size::Large
                        expanded=Box::new(|| view! { <div class="text-white font-semibold">"What are nested accordions?"</div> }.into_any())
                        collapsed=Box::new(|| view! { <div class="text-white font-semibold">"What are nested accordions?"</div> }.into_any())
                        class="bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md"
                    >
                        <List>
                            <Item>
                                "Nested accordions allow you to place one accordion inside another for organizing related content."
                            </Item>
                            <Accordion
                                expand={expand_3}
                                size=Size::Medium
                                expanded=Box::new(|| view! { <div class="text-white font-semibold">"Nested Item 1"</div> }.into_any())
                                collapsed=Box::new(|| view! { <div class="text-white font-semibold">"Nested Item 1"</div> }.into_any())
                                class="bg-gray-800 border border-gray-600 p-3 rounded-md"
                            >
                                <List>
                                    <Item>"This is a nested item within the parent accordion."</Item>
                                </List>
                            </Accordion>
                        </List>
                    </Accordion>
                </div>
                // Accordion with Emojis
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Emojis" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_signal}
    size={Size::XLarge}
    expanded={Box::new(|| view! { <h3 class="text-white bg-gray-900 font-semibold">"Accordion with Emojis Expanded"</h3> })}
    collapsed={Box::new(|| view! { <h3 class="text-gray-400 bg-gray-900 font-semibold">"Accordion with Emojis Collapsed"</h3> })}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
    duration=300
>
    <List>
        <Item title="Item with Icon" icon="🔍">"Search content"</Item>
        <Item title="Another Icon Item" icon="📦">"Package content"</Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_4}
                        size=Size::XLarge
                        expanded={Box::new(|| view! { <h3 class="text-white bg-gray-900 font-semibold">"Accordion with Emojis Expanded"</h3> }.into_any())}
                        collapsed={Box::new(|| view! { <h3 class="text-gray-400 bg-gray-900 font-semibold">"Accordion with Emojis Collapsed"</h3> }.into_any())}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                        duration=300
                    >
                        <List>
                            <Item title="Item with Icon" icon="🔍">"Search content"</Item>
                            <Item title="Another Icon Item" icon="📦">"Package content"</Item>
                        </List>
                    </Accordion>
                </div>
                // Accordion with Form Inputs
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Form Inputs" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_signal}
    size={Size::XLarge}
    expanded={Box::new(|| view! { <h3 class="text-white bg-gray-900 font-semibold">"Form Inputs Accordion Expanded"</h3> }.into_any())}
    collapsed={Box::new(|| view! { <h3 class="text-gray-400 bg-gray-900 font-semibold">"Form Inputs Accordion Collapsed"</h3> }.into_any())}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
    duration=400
>
    <List>
        <Item title="Form Item">
            <form>
                <label class="block text-sm text-gray-300 mb-2" for="name">"Name: "</label>
                <input class="w-full p-2 border border-gray-600 rounded mb-4" type="text" id="name" name="name" />
                <label class="block text-sm text-gray-300 mb-2" for="email">"Email: "</label>
                <input class="w-full p-2 border border-gray-600 rounded" type="email" id="email" name="email" />
            </form>
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_5}
                        size=Size::XLarge
                        expanded=Box::new(|| view! { <h3 class="text-white bg-gray-900 font-semibold">"Form Inputs Accordion Expanded"</h3> }.into_any())
                        collapsed=Box::new(|| view! { <h3 class="text-gray-400 bg-gray-900 font-semibold">"Form Inputs Accordion Collapsed"</h3> }.into_any())
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                        duration=400
                    >
                        <List>
                            <Item title="Form Item">
                                <form>
                                    <label class="block text-sm text-gray-300 mb-2" for="name">"Name: "</label>
                                    <input
                                        class="w-full p-2 border border-gray-600 rounded mb-4"
                                        type="text"
                                        id="name"
                                        name="name"
                                    />
                                    <label class="block text-sm text-gray-300 mb-2" for="email">"Email: "</label>
                                    <input
                                        class="w-full p-2 border border-gray-600 rounded"
                                        type="email"
                                        id="email"
                                        name="email"
                                    />
                                </form>
                            </Item>
                        </List>
                    </Accordion>
                </div>
                // FAQ Section
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "FAQs" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_signal}
    size={Size::XLarge}
    expanded={Box::new(|| view! { <div class="text-white bg-gray-900 font-semibold">"What is Leptos?"</div> }.into_any())}
    collapsed={Box::new(|| view! { <div class="text-gray-400 bg-gray-900 font-semibold">"What is Leptos?"</div> }.into_any())}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
>
    <List>
        <Item>
            "Leptos is a web framework for building reactive web applications in Rust."
        </Item>
    </List>
</Accordion>
<Accordion
    expand={expand_signal}
    size={Size::XLarge}
    expanded={Box::new(|| view! { <div class="text-white bg-gray-900 font-semibold">"Is Leptos production-ready?"</div> }.into_any())}
    collapsed={Box::new(|| view! { <div class="text-gray-400 bg-gray-900 font-semibold">"Is Leptos production-ready?"</div> }.into_any())}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
>
    <List>
        <Item>
            "Yes, Leptos is production-ready, offering excellent performance with its reactive system and Rust safety."
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_6}
                        size=Size::XLarge
                        expanded=Box::new(|| view! { <div class="text-white bg-gray-900 font-semibold">"What is Leptos?"</div> }.into_any())
                        collapsed=Box::new(|| view! { <div class="text-gray-400 bg-gray-900 font-semibold">"What is Leptos?"</div> }.into_any())
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                    >
                        <List>
                            <Item>
                                "Leptos is a web framework for building reactive web applications in Rust."
                            </Item>
                        </List>
                    </Accordion>
                    <Accordion
                        expand={expand_7}
                        size=Size::XLarge
                        expanded=Box::new(|| view! { <div class="text-white bg-gray-900 font-semibold">"Is Leptos production-ready?"</div> }.into_any())
                        collapsed=Box::new(|| view! { <div class="text-gray-400 bg-gray-900 font-semibold">"Is Leptos production-ready?"</div> }.into_any())
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                    >
                        <List>
                            <Item>
                                "Yes, Leptos is production-ready, offering excellent performance with its reactive system and Rust safety."
                            </Item>
                        </List>
                    </Accordion>
                </div>
                // Accordion with Callback
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">"Accordion with Callbacks"</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_signal}
    size=Size::XLarge
    expanded=Box::new(|| view! { <h3>"Accordion Expanded"</h3> }.into_any())
    collapsed=Box::new(|| view! { <h3>"Accordion Collapsed"</h3> }.into_any())
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-800"
    did_open={move || on_did_open()}
    will_open={move || on_will_open()}
    did_close={move || on_did_close()}
    will_close={move || on_will_close()}
>
    <List>
        <Item Align::Left>"Item 1 - Left"</Item>
        <Item Align::Right>"Item 2 - Right"</Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand=expand_8
                        size=Size::XLarge
                        expanded=Box::new(|| view! { <h3>"Accordion Expanded"</h3> }.into_any())
                        collapsed=Box::new(|| view! { <h3>"Accordion Collapsed"</h3> }.into_any())
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-800"
                        did_open=on_did_open
                        will_open=on_will_open
                        did_close=on_did_close
                        will_close=on_will_close
                    >
                        <List>
                            <Item align=Align::Left>"Item 1 - Left"</Item>
                            <Item align=Align::Right>"Item 2 - Right"</Item>
                        </List>
                    </Accordion>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App/> });
}
