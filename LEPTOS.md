# 🌱 Leptos Accordion RS Usage

Adding Accordion RS to your Leptos project is simple:

1. Make sure your project is set up with Leptos. Refer to their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add `accordion-rs` to your dependencies:

   ```sh
   cargo add accordion-rs --features=lep
   ```

1. Import the `Accordion` component into your Leptos component and start using it in your app.

## 🛠️ Usage

### Basic Example

The following is an example of using the Accordion in a Leptos project:

```rust
use leptos::prelude::*;
use accordion_rs::leptos::{Accordion, List, Item};
use accordion_rs::Align;

#[component]
pub fn App() -> impl IntoView {
    let expanded = signal(false);

    view! {
        <Accordion
            expand={expanded}
            expanded={Box::new(|| view! { <h3>{ "Accordion Expanded" }</h3> }.into_any())}
            collapsed={Box::new(|| view! { <h3>{ "Accordion Collapsed" }</h3> }.into_any())}
        >
            <List>
                <Item align={Align::Left}>{ "Item 1 - Left" }</Item>
                <Item align={Align::Right}>{ "Item 2 - Right" }</Item>
            </List>
        </Accordion>
    }
}
```

### Accordion with Callbacks

You can also attach lifecycle callbacks to handle events during the accordion's state changes.

```rust
use leptos::prelude::*;
use leptos::logging::log;
use accordion_rs::leptos::{Accordion, List, Item};

#[component]
pub fn App() -> impl IntoView {
    let expanded = signal(false);

    let will_open = move || log!("Accordion will open.");
    let did_open = move || log!("Accordion has opened.");
    let will_close = move || log!("Accordion will close.");
    let did_close = move || log!("Accordion has closed.");

    view! {
        <Accordion
            expand={expanded}
            expanded={Box::new(|| view! { <p>"Expanded Content"</p> }.into_any())}
            collapsed={Box::new(|| view! { <p>"Collapsed Content"</p> }.into_any())}
            will_open={Callback::from(will_open)}
            did_open={Callback::from(did_open)}
            will_close={Callback::from(will_close)}
            did_close={Callback::from(did_close)}
        >
            <List>
                <Item>{ "Logging Accordion Lifecycle Events" }</Item>
            </List>
        </Accordion>
    }
}
```

## 🔧 Props

### Main Props

| Property    | Type                                    | Description                                                           | Default         |
| ----------- | --------------------------------------- | --------------------------------------------------------------------- | --------------- |
| `expand`    | `(ReadSignal<bool>, WriteSignal<bool>)` | Tracks and updates the accordion's open/close state.                  | `false`         |
| `expanded`  | `Box<dyn Fn() -> AnyView>`              | Content to render when the accordion is expanded.                     | None            |
| `collapsed` | `Box<dyn Fn() -> AnyView>`              | Content to render when the accordion is collapsed.                    | None            |
| `children`  | `Children`                              | Additional elements to display within the accordion.                  | None            |
| `size`      | `Size`                                  | Defines the size of the accordion (`Small`, `Medium`, `Large`, etc.). | `Size::XXLarge` |
| `duration`  | `u64`                                   | Animation duration for expand/collapse transitions (in milliseconds). | `600`           |

### Styling Props

| Property          | Type           | Description                                      | Default |
| ----------------- | -------------- | ------------------------------------------------ | ------- |
| `class`           | `&'static str` | CSS class for the accordion container.           | `""`    |
| `expanded_class`  | `&'static str` | CSS class for the expanded accordion state.      | `""`    |
| `collapsed_class` | `&'static str` | CSS class for the collapsed accordion state.     | `""`    |
| `content_class`   | `&'static str` | CSS class for the accordion's content container. | `""`    |
| `style`           | `&'static str` | Inline styles for the accordion container.       | `""`    |
| `expanded_style`  | `&'static str` | Inline styles for the expanded accordion state.  | `""`    |
| `collapsed_style` | `&'static str` | Inline styles for the collapsed accordion state. | `""`    |

### Callback Props

| Property     | Type           | Description                                | Default |
| ------------ | -------------- | ------------------------------------------ | ------- |
| `will_open`  | `Callback<()>` | Invoked when the accordion begins opening. | No-op   |
| `did_open`   | `Callback<()>` | Invoked after the accordion has opened.    | No-op   |
| `will_close` | `Callback<()>` | Invoked when the accordion begins closing. | No-op   |
| `did_close`  | `Callback<()>` | Invoked after the accordion has closed.    | No-op   |

### Accessibility Props

| Property        | Type           | Description                                             | Default |
| --------------- | -------------- | ------------------------------------------------------- | ------- |
| `aria_controls` | `&'static str` | ARIA controls attribute for accessibility.              | `""`    |
| `aria_enabled`  | `bool`         | Whether ARIA attributes are enabled for screen readers. | `true`  |

## 💡 Tips

- Use the `expand` prop signal to control the open/close state of the accordion programmatically.
- The `size` prop can be customized to adjust the width of the accordion.
- Callback props like `will_open` and `did_close` provide hooks to perform actions during the accordion's lifecycle.
- Custom classes and styles allow for extensive customization of the accordion's appearance and behavior.
- ARIA attributes can be enabled/disabled using the `aria_enabled` prop for better screen reader compatibility.
