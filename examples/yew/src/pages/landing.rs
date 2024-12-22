use accordion_rs::yew::{Accordion, Item, List};
use accordion_rs::{Size, Align};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
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
    size={Size::Medium}
    expanded={html! { <h3>{"Accordion Expanded"}</h3> }}
    collapsed={html! { <h3>{"Accordion Collapsed"}</h3> }}
>
    <List>
        <Item alignment={Align::Left}>
            {"Item 1 Left Aligned"}
        </Item>
        <Item alignment={Align::Center}>
            {"Item 2 Centered"}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        size={Size::Medium}
                        expanded={html! { <h3>{"Accordion Expanded"}</h3> }}
                        collapsed={html! { <h3>{"Accordion Collapsed"}</h3> }}
                    >
                        <List>
                            <Item alignment={Align::Left}>{ "Item 1 Left Aligned" }</Item>
                            <Item alignment={Align::Center}>{ "Item 2 Centered" }</Item>
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
    size={Size::Custom("20rem")}
    expanded={html! { <h3 class="text-blue-500">{"Styled Accordion Expanded"}</h3> }}
    collapsed={html! { <h3 class="text-red-500">{"Styled Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md"
    expanded_class="bg-gray-800 text-white"
    collapsed_class="bg-gray-900 text-gray-500"
    duration={400}
>
    <List class="list-disc pl-5">
        <Item alignment={Align::Right}>
            {"Styled Item A"}
        </Item>
        <Item alignment={Align::Justify}>
            {"Styled Item B with custom alignment."}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
                        size={Size::Custom("20rem")}
                        expanded={html! { <h3 class="text-blue-500">{"Styled Accordion Expanded"}</h3> }}
                        collapsed={html! { <h3 class="text-red-500">{"Styled Accordion Collapsed"}</h3> }}
                        class="bg-gray-900 text-gray-400 border border-gray-700 p-4 rounded-md"
                        expanded_class="bg-gray-800 text-white"
                        collapsed_class="bg-gray-900 text-gray-500"
                        duration=400
                    >
                        <List class="list-disc pl-5">
                            <Item alignment={Align::Right}>{ "Styled Item A" }</Item>
                            <Item alignment={Align::Justify}>
                                { "Styled Item B with custom alignment." }
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
    size={Size::Large}
    expanded={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
    collapsed={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
    class="bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md"
>
    <Item>
        {"Nested accordions allow you to place one accordion inside another for organizing related content."}
    </Item>
    <Accordion
        size={Size::Medium}
        expanded={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
        collapsed={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
        class="bg-gray-800 border border-gray-600 p-3 rounded-md"
    >
        <Item>
            {"This is a nested item within the parent accordion."}
        </Item>
    </Accordion>
</Accordion>
"## }
                    </pre>
                    <Accordion
                        size={Size::Large}
                        expanded={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
                        collapsed={html! { <div class="text-white font-semibold">{"What are nested accordions?"}</div> }}
                        class="bg-gray-900 text-gray-400 border border-gray-600 p-4 rounded-md"
                    >
                        <Item>
                            { "Nested accordions allow you to place one accordion inside another for organizing related content." }
                        </Item>
                        <Accordion
                            size={Size::Medium}
                            expanded={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
                            collapsed={html! { <div class="text-white font-semibold">{"Nested Item 1"}</div> }}
                            class="bg-gray-800 border border-gray-600 p-3 rounded-md"
                        >
                            <Item class="text-gray-400">
                                { "This is a nested item within the parent accordion." }
                            </Item>
                        </Accordion>
                    </Accordion>
                </div>
                // Accordion with Emojis
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Emojis" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    size={Size::XLarge}
    expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Accordion with Emojis Expanded"}</h3> }}
    collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Accordion with Emojis Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
    duration={300}
>
    <Item title="Item with Icon" icon="🔍">
        {"Search content"}
    </Item>
    <Item title="Another Icon Item" icon="📦">
        {"Package content"}
    </Item>
</Accordion>"# }
                    </pre>
                    <Accordion
                        size={Size::XLarge}
                        expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Accordion with Emojis Expanded"}</h3> }}
                        collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Accordion with Emojis Collapsed"}</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                        duration=300
                    >
                        <Item title="Item with Icon" icon="🔍">{ "Search content" }</Item>
                        <Item title="Another Icon Item" icon="📦">{ "Package content" }</Item>
                    </Accordion>
                </div>
                // Accordion with Form Inputs
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Form Inputs" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
    size={Size::XLarge}
    expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Form Inputs Accordion Expanded"}</h3> }}
    collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Form Inputs Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
    duration={400}
>
    <Item title="Form Item">
        <form>
            <label class="block text-sm text-gray-300 mb-2" for="name">{"Name: "}</label>
            <input class="w-full p-2 border border-gray-600 rounded mb-4" type="text" id="name" name="name" />
            <label class="block text-sm text-gray-300 mb-2" for="email">{"Email: "}</label>
            <input class="w-full p-2 border border-gray-600 rounded" type="email" id="email" name="email" />
        </form>
    </Item>
</Accordion>"# }
                    </pre>
                    <Accordion
                        size={Size::XLarge}
                        expanded={html! { <h3 class="text-white bg-gray-900 font-semibold">{"Form Inputs Accordion Expanded"}</h3> }}
                        collapsed={html! { <h3 class="text-gray-400 bg-gray-900 font-semibold">{"Form Inputs Accordion Collapsed"}</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                        duration=400
                    >
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
                    </Accordion>
                </div>
                // FAQ Section
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "FAQs" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r##"<Accordion
    size={Size::XLarge}
    expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
    collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
>
    <Item>
        {"Yew is a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly."}
    </Item>
</Accordion>
<Accordion
    size={Size::XLarge}
    expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
    collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-900"
>
    <Item>
        {"Yes, Yew is production ready and offers excellent performance due to its WASM foundation."}
    </Item>
</Accordion>
"## }
                    </pre>
                    <Accordion
                        size={Size::XLarge}
                        expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
                        collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"What is Yew?"}</div> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                    >
                        <Item>
                            { "Yew is a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly." }
                        </Item>
                    </Accordion>
                    <Accordion
                        size={Size::XLarge}
                        expanded={html! { <div class="text-white bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
                        collapsed={html! { <div class="text-gray-400 bg-gray-900 font-semibold">{"Is Yew production ready?"}</div> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-900"
                    >
                        <Item>
                            { "Yes, Yew is production ready and offers excellent performance due to its WASM foundation." }
                        </Item>
                    </Accordion>
                </div>
                // Accordion with Callback Props
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Accordion with Callbacks" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"<Accordion
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
        <Item alignment={Align::Left}>
            {"Item 1 - Left"}
        </Item>
        <Item alignment={Align::Right}>
            {"Item 2 - Right"}
        </Item>
    </List>
</Accordion>"# }
                    </pre>
                    <Accordion
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
                            <Item alignment={Align::Left}>{ "Item 1 - Left" }</Item>
                            <Item alignment={Align::Right}>{ "Item 2 - Right" }</Item>
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
    size={Size::XLarge}
    expanded={html! { <h3>{"Log Accordion Expanded"}</h3> }}
    collapsed={html! { <h3>{"Log Accordion Collapsed"}</h3> }}
    class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
    expanded_class="text-white bg-gray-800"
    collapsed_class="text-gray-400 bg-gray-800"
    did_open={Callback::from(|_| log::info!("Log: Accordion did open!"))}
    will_close={Callback::from(|_| log::info!("Log: Accordion will close!"))}
>
    <Item>
        {"This accordion logs open and close actions."}
    </Item>
</Accordion>"# }
                    </pre>
                    <Accordion
                        size={Size::XLarge}
                        expanded={html! { <h3>{ "Log Accordion Expanded" }</h3> }}
                        collapsed={html! { <h3>{ "Log Accordion Collapsed" }</h3> }}
                        class="bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg"
                        expanded_class="text-white bg-gray-800"
                        collapsed_class="text-gray-400 bg-gray-800"
                        did_open={Callback::from(|_| log::info!("Log: Accordion did open!"))}
                        will_close={Callback::from(|_| log::info!("Log: Accordion will close!"))}
                    >
                        <Item>{ "This accordion logs open and close actions." }</Item>
                    </Accordion>
                </div>
            </div>
        </div>
    }
}
