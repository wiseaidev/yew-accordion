use crate::common::{Align, Size};
use dioxus::prelude::*;

/// Properties for the Accordion component.
#[derive(Props, PartialEq, Clone)]
pub struct AccordionProps {
    /// A signal that manages the expansion state of the accordion.
    ///
    /// This property determines whether the accordion is expanded (`true`) or collapsed (`false`).
    /// It is required and is used to toggle the accordion's state.
    pub expand: Signal<bool>,

    /// The content to display when the accordion is expanded.
    ///
    /// This content will be visible only when the accordion is in an expanded state.
    pub expanded: Element,

    /// The content to display when the accordion is collapsed.
    ///
    /// This content will be visible only when the accordion is in a collapsed state.
    pub collapsed: Element,

    /// The child elements to display within the accordion container.
    ///
    /// Any additional content provided as children will be rendered inside the accordion body.
    pub children: Element,

    /// Size of the accordion.
    ///
    /// Defines the size of the accordion component. Typically used for layout adjustments.
    /// Defaults to an `XXLarge` if not specified.
    #[props(default)]
    pub size: Size,

    /// ARIA controls attribute for accessibility.
    ///
    /// Links the accordion container to another element, improving accessibility for screen readers.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub aria_controls: &'static str,

    /// Custom inline styles for the accordion container.
    ///
    /// Specifies additional CSS styling for the accordion's container. Defaults to an empty string.
    #[props(default = "")]
    pub style: &'static str,

    /// Custom inline styles for the expanded state.
    ///
    /// Defines additional styling for the accordion when it is expanded. Defaults to an empty string.
    #[props(default = "")]
    pub expanded_style: &'static str,

    /// Custom inline styles for the collapsed state.
    ///
    /// Defines additional styling for the accordion when it is collapsed. Defaults to an empty string.
    #[props(default = "")]
    pub collapsed_style: &'static str,

    /// Custom inline styles for the content section.
    ///
    /// Specifies additional styling for the content within the accordion. Defaults to an empty string.
    #[props(default = "")]
    pub content_style: &'static str,

    /// Custom CSS class for the accordion container.
    ///
    /// Applies a custom class to the container element for styling purposes. Defaults to an empty string.
    #[props(default = "")]
    pub class: &'static str,

    /// Custom CSS class for the expanded state.
    ///
    /// Specifies a class that is applied to the accordion when it is in an expanded state. Defaults to an empty string.
    #[props(default = "")]
    pub expanded_class: &'static str,

    /// Custom CSS class for the collapsed state.
    ///
    /// Specifies a class that is applied to the accordion when it is in a collapsed state. Defaults to an empty string.
    #[props(default = "")]
    pub collapsed_class: &'static str,

    /// Custom CSS class for the content container.
    ///
    /// Applies a class to the accordion's content container for styling purposes. Defaults to an empty string.
    #[props(default = "")]
    pub content_class: &'static str,

    /// Indicates whether ARIA attributes should be included.
    ///
    /// If `true`, ARIA attributes such as `aria-expanded` and `aria-controls` will be added for accessibility.
    /// Defaults to `true`.
    #[props(default = true)]
    pub aria_enabled: bool,

    /// Duration of the expand/collapse animation in milliseconds.
    ///
    /// Specifies how long the transition animation should take. Defaults to `600` milliseconds.
    #[props(default = 600)]
    pub duration: u64,

    /// Callback executed before the accordion expands.
    ///
    /// This callback is triggered just before the accordion transitions to an expanded state.
    /// Defaults to a no-op.
    #[props(default)]
    pub will_open: Callback<()>,

    /// Callback executed after the accordion has expanded.
    ///
    /// This callback is triggered once the accordion has completed its transition to an expanded state.
    /// Defaults to a no-op.
    #[props(default)]
    pub did_open: Callback<()>,

    /// Callback executed before the accordion collapses.
    ///
    /// This callback is triggered just before the accordion transitions to a collapsed state.
    /// Defaults to a no-op.
    #[props(default)]
    pub will_close: Callback<()>,

    /// Callback executed after the accordion has collapsed.
    ///
    /// This callback is triggered once the accordion has completed its transition to a collapsed state.
    /// Defaults to a no-op.
    #[props(default)]
    pub did_close: Callback<()>,
}

/// Accordion Component
///
/// A Dioxus component for creating collapsible accordion sections. This component allows you
/// to toggle between expanded and collapsed views with smooth animations, and supports customization
/// through various properties like size, style, and callbacks.
///
/// # Properties
/// The component uses the `AccordionProps` struct for configuration. Key properties include:
///
/// - **expand**: A `Signal<bool>` that controls the expansion state of the accordion. It is required to toggle between expanded and collapsed states.
/// - **expanded**: The content that is displayed when the accordion is expanded (`Element`). Default: `""`.
/// - **collapsed**: The content that is displayed when the accordion is collapsed (`Element`). Default: `""`.
/// - **children**: The child elements to display inside the accordion when expanded (`Element`). Default: `""`.
/// - **size**: Defines the size of the accordion (`Size`). Default: `Size::XXLarge`.
/// - **aria_controls**: The ARIA controls attribute for accessibility (`&'static str`). Default: `""`.
/// - **style**: Inline styles for the accordion container (`&'static str`). Default: `""`.
/// - **expanded_style**: Inline styles for the expanded content (`&'static str`). Default: `""`.
/// - **collapsed_style**: Inline styles for the collapsed content (`&'static str`). Default: `""`.
/// - **content_style**: Inline styles for the content section (`&'static str`). Default: `""`.
/// - **class**: Custom CSS class for the accordion container (`&'static str`). Default: `""`.
/// - **expanded_class**: Custom CSS class for the expanded content (`&'static str`). Default: `""`.
/// - **collapsed_class**: Custom CSS class for the collapsed content (`&'static str`). Default: `""`.
/// - **content_class**: Custom CSS class for the content section (`&'static str`). Default: `""`.
/// - **aria_enabled**: If `true`, ARIA attributes will be added to the HTML structure for better accessibility (`bool`). Default: `true`.
/// - **duration**: Duration of the expand/collapse animation in milliseconds (`u64`). Default: `600`.
/// - **will_open**: Callback invoked before the accordion expands (`Callback<()>`). Default: no-op.
/// - **did_open**: Callback invoked after the accordion has expanded (`Callback<()>`). Default: no-op.
/// - **will_close**: Callback invoked before the accordion collapses (`Callback<()>`). Default: no-op.
/// - **did_close**: Callback invoked after the accordion has collapsed (`Callback<()>`). Default: no-op.
///
/// # Features
/// - Smooth transitions between expanded and collapsed states with configurable duration.
/// - Supports ARIA attributes for accessibility.
/// - Customizable styles and classes for each section of the accordion.
/// - Optional callbacks for pre- and post-expansion and collapse events.
///
/// # Examples
///
/// ## Basic Accordion
/// ```rust
/// use dioxus::prelude::*;
/// use accordion_rs::dioxus::Accordion;
/// use accordion_rs::Size;
///
/// fn App() -> Element {
///     let expand_signal = use_signal(|| false);
///
///     rsx! {
///         Accordion {
///             expand: expand_signal.clone(),
///             expanded: rsx! {p { "This is the expanded content" }},
///             collapsed: rsx! {p { "This is the collapsed content" }},
///             size: Size::Medium,
///         }
///     }
/// }
/// ```
///
/// ## Accordion with Custom Styles
/// ```rust
/// use dioxus::prelude::*;
/// use accordion_rs::dioxus::Accordion;
///
/// fn App() -> Element {
///     let expand_signal = use_signal(|| false);
///
///     rsx! {
///         Accordion {
///             expand: expand_signal.clone(),
///             expanded: rsx! { p{ "Expanded content goes here" }},
///             collapsed: rsx! { p{ "Collapsed content here" }},
///             expanded_style: "background-color: lightblue;",
///             collapsed_style: "background-color: lightgray;",
///         }
///     }
/// }
/// ```
///
/// ## Accordion with Callbacks
/// ```rust
/// use dioxus::prelude::*;
/// use accordion_rs::dioxus::Accordion;
///
/// fn App() -> Element {
///     let expand_signal = use_signal(|| false);
///
///     rsx! {
///         Accordion {
///             expand: expand_signal.clone(),
///             expanded: rsx! { p{ "Expanded content" }},
///             collapsed: rsx! { p{ "Collapsed content" }},
///             will_open: |_| println!("Accordion will open!"),
///             did_open: |_| println!("Accordion opened!"),
///             will_close: |_| println!("Accordion will close!"),
///             did_close: |_| println!("Accordion closed!"),
///         }
///     }
/// }
/// ```
#[component]
pub fn Accordion(mut props: AccordionProps) -> Element {
    let toggle_expansion = {
        move |_| {
            if (props.expand)() {
                props.will_close.call(());
                props.expand.set(false);
                props.did_close.call(());
            } else {
                props.will_open.call(());
                props.expand.set(true);
                props.did_open.call(());
            }
        }
    };

    rsx! {
        div {
            class: "{props.class}",
            style: "{props.size.to_style()} {props.style}",
            div {
                class: {if (props.expand)() {
                    props.expanded_class
                } else {
                    props.collapsed_class
                }},
                style: {format!(
                    "cursor: pointer; transition: all {}ms; {}",
                    props.duration,
                    if (props.expand)() {
                        props.expanded_style
                    } else {
                        props.collapsed_style
                    }
                )},
                aria_expanded: if props.aria_enabled { Some((props.expand)().to_string()) } else { None },
                aria_controls: if props.aria_enabled { Some(props.aria_controls) } else { None },
                onclick: toggle_expansion,
                if (props.expand)() {
                    {props.expanded}
                } else {
                    {props.collapsed}
                }
            },
            if (props.expand)() {
                div {
                    id: "{props.aria_controls}",
                    class: "{props.content_class}",
                    style: "overflow: hidden; transition: all {props.duration}ms; {props.content_style}",
                    {props.children}
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    /// The child elements of the item.
    ///
    /// These elements will be rendered inside the item container.
    pub children: Element,

    /// The inline style for the item container.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub style: &'static str,

    /// The CSS class for the item container.
    ///
    /// Defaults to an empty string.
    #[props(default = "")]
    pub class: &'static str,

    /// The alignment of the item content.
    ///
    /// Specifies how the content of the item should be aligned (e.g., `Align::Left`, `Align::Center`, etc.).
    /// Defaults to `Align::Left`.
    #[props(default)]
    pub align: Align,

    /// The title text for the item.
    ///
    /// This text typically appears as a header or label for the item.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub title: &'static str,

    /// The icon associated with the item.
    ///
    /// This can be a URL to an image or an icon class name.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub icon: &'static str,
}

#[component]
pub fn Item(props: ItemProps) -> Element {
    rsx! {
        li {
            class: "{props.class}",
            style: "{props.align.to_style()} {props.style}",
            if !props.icon.is_empty() {
                span { class: "mr-2", "{props.icon}" }
            }
            if !props.title.is_empty() {
                strong { "{props.title}" }
            }
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    /// The child elements or text content of the button.
    ///
    /// This defines the content that will appear inside the button, such as text or other elements.
    pub children: Element,

    /// The inline style for the button.
    ///
    /// This allows for custom styling directly applied to the button element.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub style: &'static str,

    /// The CSS class for the button.
    ///
    /// This applies a custom CSS class to the button for styling.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub class: &'static str,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "{props.class}",
            style: "{props.style}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ListProps {
    /// The child elements of the list.
    ///
    /// These elements represent the individual list items or nested components inside the list container.
    pub children: Element,

    /// The inline style for the list container.
    ///
    /// You can provide custom CSS styles directly to the list container using this property.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub style: &'static str,

    /// The CSS class for the list container.
    ///
    /// This class is used to apply custom styling to the list container via external CSS.
    /// Defaults to an empty string.
    #[props(default = "")]
    pub class: &'static str,
}

#[component]
pub fn List(props: ListProps) -> Element {
    rsx! {
        ul {
            class: "{props.class}",
            style: "{props.style}",
            {props.children}
        }
    }
}
