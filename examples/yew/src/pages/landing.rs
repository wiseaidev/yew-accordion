use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::{Align, Size};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let expand_0 = use_state(|| false);
    let expand_1 = use_state(|| false);
    let expand_2 = use_state(|| false);
    let expand_3 = use_state(|| false);
    let expand_4 = use_state(|| false);
    let expand_5 = use_state(|| false);
    let expand_6 = use_state(|| true);
    let expand_7 = use_state(|| false);
    let expand_8 = use_state(|| false);
    let expand_9 = use_state(|| false);

    let on_will_open = Callback::from(|_| log::info!("Accordion will open!"));
    let on_did_open = Callback::from(|_| log::info!("Accordion did open!"));
    let on_will_close = Callback::from(|_| log::info!("Accordion will close!"));
    let on_did_close = Callback::from(|_| log::info!("Accordion did close!"));

    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Accordion RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                // Default Accordion
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Default Headless Accordion" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_state}
    size={Size::Medium}
    expanded={html! { <h3>{"Accordion Expanded"}</h3> }}
    collapsed={html! { <h3>{"Accordion Collapsed"}</h3> }}
>
    <List>
        <Item align={Align::Left}>
            {"Item 1 Left Aligned"}
        </Item>
        <Item align={Align::Center}>
            {"Item 2 Centered"}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_0}
                        size={Size::Medium}
                        expanded={html! { <h3>{"Accordion Expanded"}</h3> }}
                        collapsed={html! { <h3>{"Accordion Collapsed"}</h3> }}
                    >
                        <List>
                            <Item align={Align::Left}>{ "Item 1 Left Aligned" }</Item>
                            <Item align={Align::Center}>{ "Item 2 Centered" }</Item>
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
    expand={expand_state}
    size={Size::Custom("20rem")}
    expanded={html! { <h3 class="text-blue-500">{"Styled Accordion Expanded"}</h3> }}
    collapsed={html! { <h3 class="text-red-500">{"Styled Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md"
    expanded_class="bg-gray-800 text-white"
    collapsed_class="bg-gray-900 text-gray-500"
    duration={400}
>
    <List class="list-disc pl-5">
        <Item align={Align::Right}>
            {"Styled Item A"}
        </Item>
        <Item align={Align::Justify}>
            {"Styled Item B with custom align."}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_1}
                        size={Size::Custom("20rem")}
                        expanded={html! { <h3 class="text-blue-500">{"Styled Accordion Expanded"}</h3> }}
                        collapsed={html! { <h3 class="text-red-500">{"Styled Accordion Collapsed"}</h3> }}
                        class="bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md"
                        expanded_class="bg-gray-800 text-white"
                        collapsed_class="bg-gray-900 text-gray-500"
                        duration=400
                    >
                        <List class="list-disc pl-5">
                            <Item align={Align::Right}>{ "Styled Item A" }</Item>
                            <Item align={Align::Justify}>
                                { "Styled Item B with custom align." }
                            </Item>
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
    expand={expand_state_1}
    size={Size::Large}
    expanded={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
    collapsed={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
    class="bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md"
>
    <List>
        <Item>
            {"Nested accordions allow you to place one accordion inside another for organizing related content."}
        </Item>
        <Accordion
            expand={expand_state_2}
            size={Size::Medium}
            expanded={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
            collapsed={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
            class="bg-gray-800 border border-gray-600 p-3 rounded-md"
        >
            <List>
                <Item>
                    {"This is a nested item within the parent accordion."}
                </Item>
            </List>
        </Accordion>
    </List>
</Accordion>
"## }
                    </pre>
                    <Accordion
                        expand={expand_2}
                        size={Size::Large}
                        expanded={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
                        collapsed={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
                        class="bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md"
                    >
                        <List>
                            <Item>
                                { "Nested accordions allow you to place one accordion inside another for organizing related content." }
                            </Item>
                            <Accordion
                                expand={expand_3}
                                size={Size::Medium}
                                expanded={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
                                collapsed={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
                                class="bg-gray-800 border border-gray-600 p-3 rounded-md"
                            >
                                <List>
                                    <Item class="text-gray-400">
                                        { "This is a nested item within the parent accordion." }
                                    </Item>
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
    expand={expand_state}
    size={Size::XLarge}
    expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Accordion with Emojis Expanded"}</h3> }}
    collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Accordion with Emojis Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
    duration={300}
>
    <List>
        <Item title="Item with Icon" icon="🔍">
            {"Search content"}
        </Item>
        <Item title="Another Icon Item" icon="📦">
            {"Package content"}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_4}
                        size={Size::XLarge}
                        expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Accordion with Emojis Expanded"}</h3> }}
                        collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Accordion with Emojis Collapsed"}</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                        duration=300
                    >
                        <List>
                            <Item title="Item with Icon" icon="🔍">{ "Search content" }</Item>
                            <Item title="Another Icon Item" icon="📦">{ "Package content" }</Item>
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
    expand={expand_state}
    size={Size::XLarge}
    expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Form Inputs Accordion Expanded"}</h3> }}
    collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Form Inputs Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
    duration={400}
>
    <List>
        <Item title="Form Item">
            <form>
                <label class="block text-sm text-gray-300 mb-2" for="name">{"Name: "}</label>
                <input class="w-full p-2 border border-gray-600 rounded mb-4" type="text" id="name" name="name" />
                <label class="block text-sm text-gray-300 mb-2" for="email">{"Email: "}</label>
                <input class="w-full p-2 border border-gray-600 rounded" type="email" id="email" name="email" />
            </form>
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_5}
                        size={Size::XLarge}
                        expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Form Inputs Accordion Expanded"}</h3> }}
                        collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Form Inputs Accordion Collapsed"}</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                        duration=400
                    >
                        <List>
                            <Item title="Form Item">
                                <form>
                                    <label class="block text-sm text-gray-300 mb-2" for="name">
                                        { "Name: " }
                                    </label>
                                    <input
                                        class="w-full p-2 border border-gray-600 rounded mb-4"
                                        type="text"
                                        id="name"
                                        name="name"
                                    />
                                    <label class="block text-sm text-gray-300 mb-2" for="email">
                                        { "Email: " }
                                    </label>
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
                        { r##"<Accordion
    expand={expand_state}
    size={Size::XLarge}
    expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
    collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
>
    <List>
        <Item>
            {"Yew is a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly."}
        </Item>
    </List>
</Accordion>
<Accordion
    expand={expand_state}
    size={Size::XLarge}
    expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
    collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
>
    <List>
        <Item>
            {"Yes, Yew is production ready and offers excellent performance due to its WASM foundation."}
        </Item>
    </List>
</Accordion>
"## }
                    </pre>
                    <Accordion
                        expand={expand_6}
                        size={Size::XLarge}
                        expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
                        collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                    >
                        <List>
                            <Item>
                                { "Yew is a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly." }
                            </Item>
                        </List>
                    </Accordion>
                    <Accordion
                        expand={expand_7}
                        size={Size::XLarge}
                        expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
                        collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                    >
                        <List>
                            <Item>
                                { "Yes, Yew is production ready and offers excellent performance due to its WASM foundation." }
                            </Item>
                        </List>
                    </Accordion>
                </div>
                // Accordion with Callback Props
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Callbacks" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_state}
    size={Size::XLarge}
    expanded={html! { <h3>{"Accordion Expanded"}</h3> }}
    collapsed={html! { <h3>{"Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-800"
    did_open={on_did_open.clone()}
    will_open={on_will_open.clone()}
    did_close={on_did_close.clone()}
    will_close={on_will_close.clone()}
>
    <List>
        <Item align={Align::Left}>
            {"Item 1 - Left"}
        </Item>
        <Item align={Align::Right}>
            {"Item 2 - Right"}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_8}
                        size={Size::XLarge}
                        expanded={html! { <h3>{ "Accordion Expanded" }</h3> }}
                        collapsed={html! { <h3>{ "Accordion Collapsed" }</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-800"
                        did_open={on_did_open.clone()}
                        will_open={on_will_open.clone()}
                        did_close={on_did_close.clone()}
                        will_close={on_will_close.clone()}
                    >
                        <List>
                            <Item align={Align::Left}>{ "Item 1 - Left" }</Item>
                            <Item align={Align::Right}>{ "Item 2 - Right" }</Item>
                        </List>
                    </Accordion>
                </div>
                // Accordion with Logging
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Logging (Press F12)" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    expand={expand_state}
    size={Size::XLarge}
    expanded={html! { <h3>{"Log Accordion Expanded"}</h3> }}
    collapsed={html! { <h3>{"Log Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-800"
    did_open={Callback::from(|_| log::info!("Log: Accordion did open!"))}
    will_close={Callback::from(|_| log::info!("Log: Accordion will close!"))}
>
    <List>
        <Item>
            {"This accordion logs open and close actions."}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        expand={expand_9}
                        size={Size::XLarge}
                        expanded={html! { <h3>{ "Log Accordion Expanded" }</h3> }}
                        collapsed={html! { <h3>{ "Log Accordion Collapsed" }</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-800"
                        did_open={Callback::from(|_| log::info!("Log: Accordion did open!"))}
                        will_close={Callback::from(|_| log::info!("Log: Accordion will close!"))}
                    >
                        <List>
                            <Item>{ "This accordion logs open and close actions." }</Item>
                        </List>
                    </Accordion>
                </div>
            </div>
        </div>
    }
}
