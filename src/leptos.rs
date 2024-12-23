use crate::common::{Align, Size};
use leptos::prelude::*;

/// Accordion Component
///
/// A Leptos component for displaying an accordion-style UI element that can be expanded or collapsed.
/// This `Accordion` component supports customizing its appearance, animations, and behavior during
/// expansion or collapse. The component can hold content within a collapsible section, which can be
/// expanded or collapsed by the user. It also supports various customization options, such as custom
/// styles, classes, and accessibility features like ARIA attributes.
///
/// # Properties
///
/// - **expand**: A tuple signal containing a `ReadSignal<bool>` and a `WriteSignal<bool>` for tracking and updating the expansion state of the accordion.
/// - **expanded**: A view content to display when the accordion is expanded (`Box<dyn Fn() -> AnyView>`).
/// - **collapsed**: A view content to display when the accordion is collapsed (`Box<dyn Fn() -> AnyView>`).
/// - **children**: Child elements inside the accordion (`Children`).
/// - **size**: Defines the size of the accordion (`Size`). Default: `Size::XXLarge`.
/// - **aria_controls**: The ARIA controls attribute for accessibility (`&'static str`). Default: `""`.
/// - **style**: Inline styles applied to the accordion container (`&'static str`). Default: `""`.
/// - **expanded_style**: Inline styles applied when the accordion is expanded (`&'static str`). Default: `""`.
/// - **collapsed_style**: Inline styles applied when the accordion is collapsed (`&'static str`). Default: `""`.
/// - **content_style**: Inline styles applied to the accordion's content container (`&'static str`). Default: `""`.
/// - **class**: CSS class for the accordion container (`&'static str`). Default: `""`.
/// - **expanded_class**: CSS class applied when the accordion is expanded (`&'static str`). Default: `""`.
/// - **collapsed_class**: CSS class applied when the accordion is collapsed (`&'static str`). Default: `""`.
/// - **content_class**: CSS class applied to the content container (`&'static str`). Default: `""`.
/// - **aria_enabled**: Whether ARIA attributes should be included for accessibility (`bool`). Default: `true`.
/// - **duration**: Duration of the expand/collapse transition in milliseconds (`u64`). Default: `600`.
/// - **will_open**: Callback triggered before the accordion starts expanding (`Callback<()>`). Default: no-op.
/// - **did_open**: Callback triggered after the accordion finishes expanding (`Callback<()>`). Default: no-op.
/// - **will_close**: Callback triggered before the accordion starts collapsing (`Callback<()>`). Default: no-op.
/// - **did_close**: Callback triggered after the accordion finishes collapsing (`Callback<()>`). Default: no-op.
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
/// use leptos::prelude::*;
/// use accordion_rs::leptos::{Accordion, List, Item};
/// use accordion_rs::{Align};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let expanded = signal(false);
///
///     view! {
///         <Accordion
///             expand={expanded}
///             expanded={Box::new(|| view! { <p>"This is expanded content."</p> }.into_any())}
///             collapsed={Box::new(|| view! { <p>"This is collapsed content."</p> }.into_any())}
///         >
///             <List>
///                 <Item align={Align::Left}>{ "Item 1 - Left" }</Item>
///                 <Item align={Align::Right}>{ "Item 2 - Right" }</Item>
///             </List>
///         </Accordion>
///     }
/// }
/// ```
///
/// ## Accordion with Custom Styles
/// ```rust
/// use leptos::prelude::*;
/// use accordion_rs::leptos::{Accordion, List, Item};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let expanded = signal(false);
///
///     view! {
///         <Accordion
///             expand={expanded}
///             expanded={Box::new(|| view! { <p>"Expanded content."</p> }.into_any())}
///             collapsed={Box::new(|| view! { <p>"Collapsed content."</p> }.into_any())}
///             style="background-color: lightblue; padding: 10px;"
///             expanded_style="background-color: lightgreen;"
///             collapsed_style="background-color: lightcoral;"
///         >
///             <List>
///                 <Item>{ "Item 1 - Left" }</Item>
///                 <Item>{ "Item 2 - Right" }</Item>
///             </List>
///         </Accordion>
///     }
/// }
/// ```
///
/// ## Accordion with Callbacks
/// ```rust
/// use leptos::prelude::*;
/// use leptos::logging::log;
/// use accordion_rs::leptos::{Accordion, List, Item};
///
/// #[component]
/// pub fn App() -> impl IntoView {
///     let expanded = signal(false);
///
///     let will_open = move || log!("Accordion is about to open.");
///     let did_open = move || log!("Accordion has opened.");
///     let will_close = move || log!("Accordion is about to close.");
///     let did_close = move || log!("Accordion has closed.");
///
///     view! {
///         <Accordion
///             expand={expanded}
///             expanded={Box::new(|| view! { <p>"Expanded content."</p> }.into_any())}
///             collapsed={Box::new(|| view! { <p>"Collapsed content."</p> }.into_any())}
///             will_open={Callback::from(will_open)}
///             did_open={Callback::from(did_open)}
///             will_close={Callback::from(will_close)}
///             did_close={Callback::from(did_close)}
///         >
///             <List>
///                 <Item>{ "Item 1 - Left" }</Item>
///                 <Item>{ "Item 2 - Right" }</Item>
///             </List>
///         </Accordion>
///     }
/// }
/// ```
///
/// # Behavior
/// - The accordion toggles between expanded and collapsed states based on the `expand` signal.
/// - Transitions between states are smooth, with customizable duration.
/// - Callbacks allow you to hook into the expand/collapse lifecycle events.
/// - ARIA attributes can be toggled for better accessibility when `aria_enabled` is `true`.
///
/// # Notes
/// - The `size` property determines the overall size of the accordion (e.g., `Size::Small`, `Size::Medium`).
/// - Use inline styles or CSS classes for detailed customization of the accordion's appearance.
/// - Default callbacks (`will_open`, `did_open`, `will_close`, `did_close`) are no-ops but can be customized as needed.
#[component]
pub fn Accordion(
    /// Signal to track if the accordion is expanded.
    ///
    /// This is a tuple containing a `ReadSignal` to observe the expanded state
    /// and a `WriteSignal` to update it. Use this to programmatically control or
    /// react to the accordion's expansion.
    expand: (ReadSignal<bool>, WriteSignal<bool>),

    /// Content to display when the accordion is expanded.
    ///
    /// This is a function returning an `AnyView` that will be rendered inside the accordion
    /// when it is in an expanded state.
    expanded: Box<dyn Fn() -> AnyView + Send + Sync>,

    /// Content to display when the accordion is collapsed.
    ///
    /// This is a function returning an `AnyView` that will be rendered inside the accordion
    /// when it is in a collapsed state.
    collapsed: Box<dyn Fn() -> AnyView + Send + Sync>,

    /// Child elements inside the accordion.
    ///
    /// These are additional elements that are rendered as part of the accordion's body.
    children: ChildrenFn,

    /// Size of the accordion.
    ///
    /// This defines the overall size of the accordion component. Acceptable values
    /// are defined by the `Size` enum, and the default value is `Size::XXLarge`.
    #[prop(default = Size::XXLarge)]
    size: Size,

    /// ARIA controls attribute.
    ///
    /// Sets the value for the `aria-controls` attribute, which is used for accessibility
    /// purposes to associate the accordion header with its content. Defaults to an empty string.
    #[prop(default = "")]
    aria_controls: &'static str,

    /// Inline style for the accordion.
    ///
    /// Applies custom inline styles to the accordion container. Defaults to an empty string.
    #[prop(default = "")]
    style: &'static str,

    /// Style when the accordion is expanded.
    ///
    /// Defines additional inline styles applied to the accordion when it is expanded.
    /// Defaults to an empty string.
    #[prop(default = "")]
    expanded_style: &'static str,

    /// Style when the accordion is collapsed.
    ///
    /// Defines additional inline styles applied to the accordion when it is collapsed.
    /// Defaults to an empty string.
    #[prop(default = "")]
    collapsed_style: &'static str,

    /// Style for the accordion's content container.
    ///
    /// Sets inline styles for the container that wraps the accordion's content.
    /// Defaults to an empty string.
    #[prop(default = "")]
    content_style: &'static str,

    /// CSS class for the accordion.
    ///
    /// Adds a CSS class to the accordion container for styling purposes. Defaults to an empty string.
    #[prop(default = "")]
    class: &'static str,

    /// CSS class when the accordion is expanded.
    ///
    /// Adds a CSS class to the accordion container when it is in an expanded state.
    /// Defaults to an empty string.
    #[prop(default = "")]
    expanded_class: &'static str,

    /// CSS class when the accordion is collapsed.
    ///
    /// Adds a CSS class to the accordion container when it is in a collapsed state.
    /// Defaults to an empty string.
    #[prop(default = "")]
    collapsed_class: &'static str,

    /// CSS class for the content container.
    ///
    /// Adds a CSS class to the container that wraps the accordion's content. Defaults to an empty string.
    #[prop(default = "")]
    content_class: &'static str,

    /// Whether to include ARIA attributes.
    ///
    /// If `true`, ARIA attributes are included to improve accessibility. Defaults to `true`.
    #[prop(default = true)]
    aria_enabled: bool,

    /// Duration of the expand/collapse transition in milliseconds.
    ///
    /// Sets the time it takes for the accordion to transition between expanded and collapsed states.
    /// Defaults to `600` milliseconds.
    #[prop(default = 600)]
    duration: u64,

    /// Callback for when the accordion starts opening.
    ///
    /// This callback is invoked at the start of the accordion's expand transition.
    /// Defaults to no-op.
    #[prop(default = Callback::from(|| {}))]
    will_open: Callback<()>,

    /// Callback for when the accordion finishes opening.
    ///
    /// This callback is invoked after the accordion has fully expanded.
    /// Defaults to no-op.
    #[prop(default = Callback::from(|| {}))]
    did_open: Callback<()>,

    /// Callback for when the accordion starts closing.
    ///
    /// This callback is invoked at the start of the accordion's collapse transition.
    /// Defaults to no-op.
    #[prop(default = Callback::from(|| {}))]
    will_close: Callback<()>,

    /// Callback for when the accordion finishes closing.
    ///
    /// This callback is invoked after the accordion has fully collapsed.
    /// Defaults to no-op.
    #[prop(default = Callback::from(|| {}))]
    did_close: Callback<()>,
) -> impl IntoView {
    let toggle_expansion = move || {
        if expand.0.get() {
            will_close.run(());
            expand.1.set(false);
            did_close.run(());
        } else {
            will_open.run(());
            expand.1.set(true);
            did_open.run(());
        }
    };

    view! {
        <div
            style=format!("{} {}", size.to_style(), style)
            class=class
        >
            <div
                aria-expanded={move || if aria_enabled { Some(expand.0.get().to_string()) } else { None }}
                aria-controls=aria_controls
                on:click=move |_| toggle_expansion()
                class=move || if expand.0.get() { expanded_class } else { collapsed_class }
                style=move || format!(
                    "cursor: pointer; transition: all {}ms; {}",
                    duration,
                    if expand.0.get() { expanded_style } else { collapsed_style }
                )
            >
                {move || {
                    if expand.0.get() {
                        expanded()
                    } else {
                        collapsed()
                    }
                }}
            </div>
            <Show when=move || expand.0.get() clone:children>
                <div
                    id=aria_controls
                    class=content_class
                    style=format!(
                        "overflow: hidden; transition: all {}ms; {}",
                        duration,
                        content_style
                    )
                >
                    {children()}
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn Item(
    /// Child content of the Item
    children: Children,

    /// Additional styles for the Item
    #[prop(default = "")]
    style: &'static str,

    /// CSS class for the Item
    #[prop(default = "")]
    class: &'static str,

    /// Alignment for the content
    #[prop(default = Align::Left)]
    align: Align,

    /// Title of the Item
    #[prop(default = "")]
    title: &'static str,

    /// Optional icon for the Item
    #[prop(default = "")]
    icon: &'static str,
) -> impl IntoView {
    view! {
        <li
            class=class
            style=format!("{} {}", align.to_style(), style)
        >
            {move || {
                if !icon.is_empty() {
                    Some(view! { <span class="mr-2">{icon}</span> })
                } else {
                    None
                }
            }}
            {move || {
                if !title.is_empty() {
                    Some(view! { <strong>{title}</strong> })
                } else {
                    None
                }
            }}
            {children()}
        </li>
    }
}

#[component]
pub fn Button(
    /// Content for the Button
    children: Children,

    /// Styles for the Button
    #[prop(default = "")]
    style: &'static str,

    /// CSS class for the Button
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <button class=class style=style>
            {children()}
        </button>
    }
}

#[component]
pub fn List(
    /// Child items for the List
    children: Children,

    /// Styles for the List
    #[prop(default = "")]
    style: &'static str,

    /// CSS class for the List
    #[prop(default = "")]
    class: &'static str,
) -> impl IntoView {
    view! {
        <ul class=class style=style>
            {children()}
        </ul>
    }
}
