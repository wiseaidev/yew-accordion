use crate::common::{Align, Size};
use yew::prelude::*;

/// Properties for the Accordion component.
#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProps {
    /// A state handle that manages the expansion state of the accordion.
    ///
    /// This property determines whether the accordion is initially expanded or collapsed.
    /// It is a required prop and should be used to toggle the accordion's expanded state.
    pub expand: UseStateHandle<bool>,

    /// The content to be displayed when the accordion is expanded.
    ///
    /// Defines the HTML content shown when the accordion is expanded. Defaults to an empty string.
    #[prop_or_default]
    pub expanded: Html,

    /// The content to be displayed when the accordion is collapsed.
    ///
    /// Defines the HTML content shown when the accordion is collapsed. Defaults to an empty string.
    #[prop_or_default]
    pub collapsed: Html,

    /// The child elements within the accordion.
    ///
    /// Any child HTML content that will be displayed within the accordion container. Defaults to an empty string.
    #[prop_or_default]
    pub children: Html,

    /// Size of the accordion.
    ///
    /// Defines the size of the accordion component, such as "small", "medium", or "large". Defaults to `Size::XXLarge`.
    #[prop_or_default]
    pub size: Size,

    /// ARIA controls attribute for accessibility.
    ///
    /// Provides an accessibility feature for screen readers, linking the accordion with other elements. Defaults to an empty string.
    #[prop_or_default]
    pub aria_controls: &'static str,

    /// Custom inline styles for the container.
    ///
    /// Allows for custom styling of the accordion container. Defaults to an empty string.
    #[prop_or_default]
    pub style: &'static str,

    /// Custom inline styles for the expanded element.
    ///
    /// Allows for custom styling of the expanded element. Defaults to an empty string.
    #[prop_or_default]
    pub expanded_style: &'static str,

    /// Custom inline styles for the collapsed element.
    ///
    /// Allows for custom styling of the collapsed element. Defaults to an empty string.
    #[prop_or_default]
    pub collapsed_style: &'static str,

    /// Custom inline styles for the content container.
    ///
    /// Allows for custom styling of the content section. Defaults to an empty string.
    #[prop_or_default]
    pub content_style: &'static str,

    /// Custom class for the container.
    ///
    /// Applies a custom CSS class to the accordion container. Defaults to an empty string.
    #[prop_or_default]
    pub class: &'static str,

    /// Custom class for the expanded element.
    ///
    /// Applies a custom CSS class to the expanded content. Defaults to an empty string.
    #[prop_or_default]
    pub expanded_class: &'static str,

    /// Custom class for the collapsed element.
    ///
    /// Applies a custom CSS class to the collapsed content. Defaults to an empty string.
    #[prop_or_default]
    pub collapsed_class: &'static str,

    /// Custom class for the content container.
    ///
    /// Applies a custom CSS class to the content section. Defaults to an empty string.
    #[prop_or_default]
    pub content_class: &'static str,

    /// Whether ARIA attributes should be added to the HTML structure.
    ///
    /// If `true`, ARIA attributes will be included for better accessibility. Defaults to `true`.
    #[prop_or(true)]
    pub aria_enabled: bool,

    /// Duration of the animation in milliseconds.
    ///
    /// Defines the animation speed for expanding or collapsing the accordion. Defaults to `600` milliseconds.
    #[prop_or(600)]
    pub duration: u64,

    /// Callback executed before the accordion item is opened.
    ///
    /// This callback is triggered before the accordion expands. Defaults to no-op.
    #[prop_or_default]
    pub will_open: Callback<()>,

    /// Callback executed when the accordion item is opened.
    ///
    /// This callback is triggered after the accordion has expanded. Defaults to no-op.
    #[prop_or_default]
    pub did_open: Callback<()>,

    /// Callback executed before the accordion item is closed.
    ///
    /// This callback is triggered before the accordion collapses. Defaults to no-op.
    #[prop_or_default]
    pub will_close: Callback<()>,

    /// Callback executed when the accordion item is closed.
    ///
    /// This callback is triggered after the accordion has collapsed. Defaults to no-op.
    #[prop_or_default]
    pub did_close: Callback<()>,
}

/// Accordion Component
///
/// A Yew component for displaying an accordion-style UI element that can be expanded or collapsed.
/// This `Accordion` component supports customizing its appearance, animations, and behavior during
/// expansion or collapse. The component can hold content within a collapsible section, which can be
/// expanded or collapsed by the user. It also supports various customization options, such as custom
/// styles, classes, and accessibility features like ARIA attributes.
///
/// # Properties
/// The component uses the `AccordionProps` struct for its properties. Key properties include:
///
/// - **expand**: A state handle that manages the expansion state of the accordion (`UseStateHandle<bool>`).
/// - **expanded**: The content to display when the accordion is expanded (`Html`). Default: `""`.
/// - **collapsed**: The content to display when the accordion is collapsed (`Html`). Default: `""`.
/// - **children**: The child elements inside the accordion (`Html`). Default: `""`.
/// - **size**: Defines the size of the accordion (`Size`). Default: `Size::XXLarge`.
/// - **aria_controls**: The ARIA controls attribute for accessibility (`&'static str`). Default: `""`.
/// - **style**: Custom inline styles for the accordion container (`String`). Default: `""`.
/// - **expanded_style**: Custom inline styles for the expanded accordion section (`String`). Default: `""`.
/// - **collapsed_style**: Custom inline styles for the collapsed accordion section (`String`). Default: `""`.
/// - **content_style**: Custom inline styles for the content container (`String`). Default: `""`.
/// - **class**: Custom CSS class for the accordion container (`String`). Default: `""`.
/// - **expanded_class**: Custom CSS class for the expanded accordion section (`String`). Default: `""`.
/// - **collapsed_class**: Custom CSS class for the collapsed accordion section (`String`). Default: `""`.
/// - **content_class**: Custom CSS class for the content container (`String`). Default: `""`.
/// - **aria_enabled**: Whether ARIA attributes should be added for accessibility (`bool`). Default: `true`.
/// - **duration**: Duration of the animation when expanding or collapsing (`u64`). Default: `600`.
/// - **will_open**: Callback triggered before the accordion expands (`Callback<()>`). Default: no-op.
/// - **did_open**: Callback triggered after the accordion expands (`Callback<()>`). Default: no-op.
/// - **will_close**: Callback triggered before the accordion collapses (`Callback<()>`). Default: no-op.
/// - **did_close**: Callback triggered after the accordion collapses (`Callback<()>`). Default: no-op.
///
/// # Features
/// - Customizable expanded and collapsed content.
/// - Animation duration for smooth transitions between states.
/// - ARIA accessibility features for improved screen reader support.
/// - Callbacks for tracking expansion and collapse events.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use accordion_rs::yew::Accordion;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let expand = use_state(|| false);
///
///     html! {
///         <Accordion
///             expand={expand}
///             expanded="This is expanded content"
///             collapsed="This is collapsed content"
///         />
///     }
/// }
/// ```
///
/// ## Accordion with Custom Styles
/// ```rust
/// use yew::prelude::*;
/// use accordion_rs::yew::Accordion;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let expand = use_state(|| false);
///
///     html! {
///         <Accordion
///             expand={expand}
///             expanded="This is expanded content"
///             collapsed="This is collapsed content"
///             style="background-color: lightblue; padding: 10px"
///             expanded_style="background-color: lightgreen"
///             collapsed_style="background-color: lightcoral"
///         />
///     }
/// }
/// ```
///
/// ## Accordion with Callbacks
/// ```rust
/// use yew::prelude::*;
/// use accordion_rs::yew::Accordion;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let expand = use_state(|| false);
///
///     let will_open = Callback::from(|_| log::info!("Accordion is about to open"));
///     let did_open = Callback::from(|_| log::info!("Accordion has opened"));
///     let will_close = Callback::from(|_| log::info!("Accordion is about to close"));
///     let did_close = Callback::from(|_| log::info!("Accordion has closed"));
///
///     html! {
///         <Accordion
///             expand={expand}
///             expanded="This is expanded content"
///             collapsed="This is collapsed content"
///             will_open={will_open}
///             did_open={did_open}
///             will_close={will_close}
///             did_close={did_close}
///         />
///     }
/// }
/// ```
///
/// # Behavior
/// - The component uses a state to track whether the accordion is expanded or collapsed.
/// - Clicking the accordion header toggles between expanded and collapsed states, with smooth animation transitions.
/// - It emits callbacks when the accordion is about to open or close, and after those actions have completed.
/// - ARIA attributes are dynamically added for accessibility when `aria_enabled` is set to `true`.
///
/// # Notes
/// - The `aria_enabled` property can be set to `false` to disable ARIA attributes for cases where they are not needed.
/// - The `will_open`, `did_open`, `will_close`, and `did_close` callbacks can be used to hook into the accordion's state changes for custom behavior.
/// - The `size` property allows customization of the accordion's size (e.g., `Size::Small`, `Size::Medium`, `Size::Large`).
#[function_component]
pub fn Accordion(props: &AccordionProps) -> Html {
    let is_expanded = &props.expand;
    let is_expanded_value = **is_expanded;

    let toggle_expansion = {
        let is_expanded = is_expanded.clone();
        let props = props.clone();

        move |e: MouseEvent| {
            e.prevent_default();

            if is_expanded_value {
                props.will_close.emit(());
                is_expanded.set(false);
                props.did_close.emit(());
            } else {
                props.will_open.emit(());
                is_expanded.set(true);
                props.did_open.emit(());
            }
        }
    };

    html! {
        <div
            style={format!(
                "{} {}",
                props.size.to_style(),
                props.style
            )}
            class={props.class}
        >
            <div
                aria-expanded={if props.aria_enabled { Some(is_expanded_value.to_string()) } else { None }}
                aria-controls={if props.aria_enabled { Some(props.aria_controls) } else { None }}
                onclick={toggle_expansion.clone()}
                class={if is_expanded_value {
                        props.expanded_class
                    } else {
                        props.collapsed_class
                    }}
                style={format!(
                    "cursor: pointer; transition: all {}ms; {}",
                    props.duration,
                    if is_expanded_value {
                        props.expanded_style
                    } else {
                        props.collapsed_style
                    }
                )}
            >
                { if is_expanded_value { props.expanded.clone() } else { props.collapsed.clone() } }
            </div>
            { if is_expanded_value {
                html! {
                    <div
                        id={props.aria_controls}
                        class={props.content_class}
                        style={format!(
                            "overflow: hidden; transition: all {}ms; {}",
                            props.duration,
                            props.content_style
                        )}
                    >
                        { props.children.clone() }
                    </div>
                }
            } else {
                html! {}
            } }
        </div>
    }
}

/// Properties for the Item component.
#[derive(Clone, PartialEq, Properties)]
pub struct ItemProps {
    /// The content of the Item.
    ///
    /// Defines the HTML content that will be displayed inside the accordion item. Defaults to an empty string.
    #[prop_or_default]
    pub children: Html,

    /// Additional inline styles for the Item.
    ///
    /// Allows for custom styling of the accordion item. Defaults to an empty string.
    #[prop_or_default]
    pub style: &'static str,

    /// Additional class for the Item.
    ///
    /// Applies a custom CSS class to the accordion item. Defaults to an empty string.
    #[prop_or_default]
    pub class: &'static str,

    /// Alignment of the content inside the Item.
    ///
    /// Defines the alignment of content within the accordion item, such as left, right, or center. Defaults to `Align::Left`.
    #[prop_or_default]
    pub align: Align,

    /// The title of the Item.
    ///
    /// Provides the title or heading for the accordion item. Defaults to `None`.
    #[prop_or_default]
    pub title: &'static str,

    /// The icon for the Item.
    ///
    /// Specifies an optional icon to be displayed alongside the title. Defaults to `None`.
    #[prop_or_default]
    pub icon: &'static str,
}

/// Item component.
#[function_component]
pub fn Item(props: &ItemProps) -> Html {
    html! {
        <li
            class={props.class}
            style={format!(
                "{} {}",
                props.align.to_style(),
                props.style
            )}
        >
            { if !props.icon.is_empty() {
                    html! { <span class="mr-2">{ props.icon }</span> }
                } else {
                    html! {}
                } }
            { if !props.title.is_empty() {
                    html! { <strong>{ props.title }</strong> }
                } else {
                    html! {}
                } }
            { props.children.clone() }
        </li>
    }
}

/// Properties for the Button component.
#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    /// The content of the Button.
    ///
    /// Defines the text or HTML content displayed on the button. Defaults to an empty string.
    #[prop_or_default]
    pub children: Html,

    /// Additional inline styles for the Button.
    ///
    /// Allows for custom styling of the button. Defaults to an empty string.
    #[prop_or_default]
    pub style: &'static str,

    /// Additional inline styles for the Button.
    ///
    /// Allows for custom CSS class styling for the button. Defaults to an empty string.
    #[prop_or_default]
    pub class: &'static str,
}

/// Button component.
#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    html! {
        <button class={props.class} style={props.style}>
            { props.children.clone() }
        </button>
    }
}

/// Properties for the List component.
#[derive(Clone, PartialEq, Properties)]
pub struct ListProps {
    /// The List items.
    ///
    /// Defines the child elements (list items) that will be displayed inside the list. Defaults to an empty string.
    #[prop_or_default]
    pub children: Html,

    /// Additional inline styles for the List.
    ///
    /// Allows for custom styling of the list container. Defaults to an empty string.
    #[prop_or_default]
    pub style: &'static str,

    /// Additional inline styles for the List.
    ///
    /// Allows for custom CSS class styling for the list container. Defaults to an empty string.
    #[prop_or_default]
    pub class: &'static str,
}

/// List component.
#[function_component]
pub fn List(props: &ListProps) -> Html {
    html! {
        <ul class={props.class} style={props.style}>{ props.children.clone() }</ul>
    }
}
