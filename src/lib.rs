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
