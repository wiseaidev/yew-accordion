use yew::prelude::*;
use yew_accordion::{Accordion, AccordionButton, AccordionItem};

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="flex space-x-4 justify-center">
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>{ "Show +" }</AccordionButton>}}
                    size="sm"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-700"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gray-900 p-4 rounded border-t border-gray-700"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded animate-pulse"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-pulse"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-4 rounded border border-blue-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-800 to-blue-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-blue-900 to-blue-700 text-white p-4 rounded border-t border-blue-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-blue-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-blue-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-blue-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded transform scale-105"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded transform scale-105"}>{ "Show +" }</AccordionButton>}}
                    size="lg"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-purple-700 to-purple-500 text-white p-4 rounded border border-purple-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-purple-800 to-purple-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-purple-900 to-purple-700 text-white p-4 rounded border-t border-purple-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-purple-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-purple-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-purple-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-orange-500 text-white p-2 rounded animate-spin"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-spin"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-orange-700 to-orange-500 text-white p-4 rounded border border-orange-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-orange-800 to-orange-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-orange-900 to-orange-700 text-white p-4 rounded border-t border-orange-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-orange-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-orange-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-orange-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-teal-500 text-white p-2 rounded animate-bounce"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-bounce"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-teal-700 to-teal-500 text-white p-4 rounded border border-teal-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-teal-800 to-teal-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-teal-900 to-teal-700 text-white p-4 rounded border-t border-teal-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-teal-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-teal-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-teal-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-indigo-500 text-white p-2 rounded animate-pulse"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-pulse"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-indigo-700 to-indigo-500 text-white p-4 rounded border border-indigo-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-indigo-800 to-indigo-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-indigo-900 to-indigo-700 text-white p-4 rounded border-t border-indigo-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-indigo-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-indigo-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-indigo-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-pink-500 text-white p-2 rounded animate-rotate"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-rotate"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-pink-700 to-pink-500 text-white p-4 rounded border border-pink-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-pink-800 to-pink-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-pink-900 to-pink-700 text-white p-4 rounded border-t border-pink-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-pink-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-pink-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-pink-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-red-500 text-white p-2 rounded animate-ping"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-ping"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-red-700 to-red-500 text-white p-4 rounded border border-red-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-red-800 to-red-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-red-900 to-red-700 text-white p-4 rounded border-t border-red-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-red-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-red-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-red-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-cyan-500 text-white p-2 rounded animate-bounce"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-bounce"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-cyan-700 to-cyan-500 text-white p-4 rounded border border-cyan-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-cyan-800 to-cyan-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-cyan-900 to-cyan-700 text-white p-4 rounded border-t border-cyan-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-cyan-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-cyan-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-cyan-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
            <div class="flex-shrink-0">
                <Accordion
                    expanded_element={html! {<AccordionButton class={"bg-blue-gray-500 text-white p-2 rounded animate-pulse"}>{ "Hide -" }</AccordionButton>}}
                    collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded animate-pulse"}>{ "Show +" }</AccordionButton>}}
                    size="md"
                    aria_controls="example-accordion"
                    container_class="my-custom-class bg-gradient-to-r from-blue-gray-700 to-blue-gray-500 text-white p-4 rounded border border-blue-gray-600"
                    expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-gray-800 to-blue-gray-600 text-white p-2 rounded"
                    collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
                    content_container_class="my-content-class bg-gradient-to-r from-blue-gray-900 to-blue-gray-700 text-white p-4 rounded border-t border-blue-gray-600"
                >
                    <ul>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-blue-gray-600 transition duration-300 ease-in-out"
                        >{ "Item 1" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class border-b p-2 hover:bg-blue-gray-600 transition duration-300 ease-in-out"
                        >{ "Item 2" }</AccordionItem>
                        <AccordionItem
                            item_class="my-list-item-class p-2 hover:bg-blue-gray-600 transition duration-300 ease-in-out"
                        >{ "Item 3" }</AccordionItem>
                    </ul>
                </Accordion>
            </div>
        </div>
    }
}
