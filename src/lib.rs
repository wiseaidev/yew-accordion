#![doc(
    html_logo_url = "https://github.com/next-rs/yew-accordion/assets/62179149/48fa7fe4-90a1-4314-801b-49389cebf33e",
    html_favicon_url = "https://github.com/next-rs/yew-accordion/assets/62179149/05482d9e-3bb3-49c2-9d3e-26aa9c12d936"
)]

//! # Yew Accordion - Documentation
//!
//! Welcome to the official Yew Accordion documentation. This library
//! provides a customizable accordion component for your Yew applications.
//!
//! ## Usage
//!
//! To use the Yew Accordion library, add the following dependency to your `Cargo.toml` file:
//!
//! ```sh
//! cargo add yew-accordion
//! ```
//!
//! To integrate the library into your Yew application, you can use the `Accordion`, `AccordionItem`,
//! and `AccordionButton` components. Here's a simple example of how to use them:
//!
//! ```rust
//! use yew::prelude::*;
//! use yew_accordion::{Accordion, AccordionItem, AccordionButton};
//!
//! // Your Yew component structure here...
//!
//! #[function_component]
//! pub fn MyAccordionComponent() -> Html {
//!     // Your component logic here...
//!
//!     html! {
//!         <Accordion
//!             expanded_element={html! {<AccordionButton class={"bg-blue-500 text-white p-2 rounded"}>{ "Hide -" }</AccordionButton>}}
//!             collapsed_element={html! {<AccordionButton class={"bg-green-500 text-white p-2 rounded"}>{ "Show +" }</AccordionButton>}}
//!             size="sm"
//!             aria_controls="example-accordion"
//!             container_class="my-custom-class bg-gray-800 p-4 rounded border border-gray-700"
//!             expanded_element_class="my-expanded-class bg-gradient-to-r from-blue-700 to-blue-500 text-white p-2 rounded"
//!             collapsed_element_class="my-collapsed-class bg-gradient-to-r from-green-700 to-green-500 text-white p-2 rounded"
//!             content_container_class="my-content-class bg-gray-900 p-4 rounded border-t border-gray-700"
//!         >
//!             <ul>
//!                 <AccordionItem
//!                     item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
//!                 >{ "Item 1" }</AccordionItem>
//!                 <AccordionItem
//!                     item_class="my-list-item-class border-b p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
//!                 >{ "Item 2" }</AccordionItem>
//!                 <AccordionItem
//!                     item_class="my-list-item-class p-2 hover:bg-gray-700 transition duration-300 ease-in-out"
//!                 >{ "Item 3" }</AccordionItem>
//!             </ul>
//!         </Accordion>
//!     }
//! }
//! ```
//!
//! For more detailed information, check the [examples] provided in the library.
//!
//! [examples]: https://github.com/next-rs/yew-accordion/tree/main/examples
//!
//! ## Configuration
//!
//! Yew Accordion allows you to customize various aspects of the accordion component through the
//! `AccordionProps`, `AccordionItemProps`, and `AccordionButtonProps` structures. You can adjust
//! properties such as size, ARIA controls, and custom classes. Refer to the respective documentation
//! for detailed configuration options.
//!
//! ## Contribution
//!
//! If you encounter any issues or have suggestions for improvements, feel free to contribute
//! to the [GitHub repository](https://github.com/next-rs/yew-accordion). We appreciate your feedback
//! and involvement in making Yew Accordion better!
//!
//! ## Acknowledgments
//!
//! Special thanks to the Yew community and contributors for such an amazing framework.
//!

use yew::prelude::*;

/// Properties for the Accordion component.
#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    #[prop_or_default]
    /// The content to be displayed when the accordion is expanded.
    pub expanded_element: Html,
    
    #[prop_or_default]
    /// The content to be displayed when the accordion is collapsed.
    pub collapsed_element: Html,
    
    #[prop_or_default]
    /// The child elements within the accordion.
    pub children: Html,
    
    #[prop_or_default]
    /// Size of the accordion. Possible values: "sm", "md", "lg".
    pub size: &'static str,
    
    #[prop_or_default]
    /// ARIA controls attribute for accessibility.
    pub aria_controls: &'static str,
    
    #[prop_or_default]
    /// Class for the container element.
    pub container_class: &'static str,
    
    #[prop_or_default]
    /// Class for the expanded element.
    pub expanded_element_class: &'static str,
    
    #[prop_or_default]
    /// Class for the collapsed element.
    pub collapsed_element_class: &'static str,
    
    #[prop_or_default]
    /// Class for the content container.
    pub content_container_class: &'static str,
}

/// Accordion component.
#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    // State to track whether the accordion is expanded or collapsed.
    let is_expanded = use_state(|| false);
    let props = props.clone();
    let is_expanded_value = *is_expanded;

    // Callback function to handle accordion click events.
    let onclick = move |e: MouseEvent| {
        e.prevent_default();
        is_expanded.set(!is_expanded_value);
    };

    html! {
        <div class={get_accordion_container_style(&props.size, &props.container_class)}>
            <div
                aria-expanded={is_expanded_value.to_string()}
                aria-controls={props.aria_controls}
                onclick={onclick}
                class={get_toggle_element_style(&is_expanded_value, &props.expanded_element_class, &props.collapsed_element_class)}
            >
                { if is_expanded_value {props.expanded_element.clone()} else {props.collapsed_element.clone()} }
            </div>
            { if is_expanded_value {
                html! { <div id={props.aria_controls} class={props.content_container_class}>{props.children.clone()}</div> }
            } else {
                html! {}
            } }
        </div>
    }
}

/// Get the CSS class for the accordion container based on size and custom class.
fn get_accordion_container_style(size: &str, custom_class: &str) -> String {
    let base_class = match size {
        "sm" => "w-28",
        "md" => "w-40",
        "lg" => "w-80",
        _ => "",
    };
    format!("{} {}", base_class, custom_class)
}

/// Get the CSS class for the toggle element based on its expanded state.
fn get_toggle_element_style(
    is_expanded: &bool,
    expanded_class: &'static str,
    collapsed_class: &'static str,
) -> &'static str {
    if *is_expanded {
        expanded_class
    } else {
        collapsed_class
    }
}

/// Properties for the AccordionItem component.
#[derive(Clone, PartialEq, Properties)]
pub struct AccordionItemProps {
    #[prop_or_default]
    /// The content of the AccordionItem.
    pub children: Html,
    
    #[prop_or_default]
    /// Additional class for the AccordionItem.
    pub item_class: &'static str,
}

/// AccordionItem component.
#[function_component]
pub fn AccordionItem(props: &AccordionItemProps) -> Html {
    html! { <li class={props.item_class}>{ props.children.clone() }</li> }
}

/// Properties for the AccordionButton component.
#[derive(Clone, PartialEq, Properties)]
pub struct AccordionButtonProps {
    #[prop_or_default]
    /// The content of the AccordionButton.
    pub children: Html,
    
    #[prop_or_default]
    /// Additional class for the AccordionButton.
    pub class: &'static str,
}

/// AccordionButton component.
#[function_component]
pub fn AccordionButton(props: &AccordionButtonProps) -> Html {
    html! { <button class={props.class}>{ props.children.clone() }</button> }
}
