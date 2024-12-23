## 🧬 Accordion RS Dioxus Usage

Adding Accordion RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the Accordion component to your dependencies by including it in your `Cargo.toml` file.

   ```sh
   cargo add accordion-rs --features=dio
   ```

1. Import the `Accordion` component into your Dioxus component and start using it in your app.

## 🛠️ Usage

Incorporating the Accordion component into your application is easy. Follow these steps:

1. Import the Accordion component into your Dioxus project:

   ```rust
   use dioxus::prelude::*;
   use accordion_rs::dioxus::Accordion;
   ```

1. Define the accordion sections and use the Accordion component in your Dioxus app:

   ```rust
   use accordion_rs::dioxus::{Accordion, Item, List};
   use accordion_rs::{Size, Align};
   use dioxus::prelude::*;

   pub fn app() -> Element {
       let expand = use_signal(|| false);

       rsx!(
           Accordion {
               expand: expand,
               size: Size::XLarge,
               expanded: rsx!(div { "Accordion Expanded" }),
               collapsed: rsx!(div { "Accordion Collapsed" }),
               class: "bg-gray-900 text-gray-300 border border-gray-700 p-4 rounded-lg",
               expanded_class: "text-white bg-gray-800",
               collapsed_class: "text-gray-400 bg-gray-800",
               List {
                   Item { align: Align::Left, "Item 1 - Left" }
                   Item { align: Align::Right, "Item 2 - Right" }
               }
           }
       )
   }
   ```

## 🔧 Props

### Main Props

| Property    | Type           | Description                                                               | Default         |
| ----------- | -------------- | ------------------------------------------------------------------------- | --------------- |
| `expand`    | `Signal<bool>` | Signal managing whether the accordion is initially expanded or collapsed. | `false`         |
| `expanded`  | `Element`      | Content to display when the accordion is expanded.                        | `""`            |
| `collapsed` | `Element`      | Content to display when the accordion is collapsed.                       | `""`            |
| `children`  | `Element`      | Child elements displayed within the accordion container.                  | `""`            |
| `size`      | `Size`         | Size of the accordion (`"small"`, `"medium"`, `"large"`, etc.).           | `Size::XXLarge` |
| `duration`  | `u64`          | Animation duration for expand/collapse transitions, in milliseconds.      | `600`           |

### Styling Props

```sh
+-----------------------------------------------------------+
|                  [Accordion Container]                    |  <-- `class` & `style`
|                                                           |
|   +-----------------------------------------------+       |  <-- `collapsed_class` & `collapsed_style`
|   |             [Collapsed Content]               |       |  <-- `collapsed`
|   +-----------------------------------------------+       |
|                                                           |
|   +-----------------------------------------------+       |  <-- `expanded_class` & `expanded_style`
|   |             [Expanded Content]                |       |  <-- `expanded`
|   +-----------------------------------------------+       |
|                                                           |
|   +-----------------------------------------------+       |  <-- `content_class` & `content_style`
|   |              [Accordion Content]              |       |  <-- `children`
|   +-----------------------------------------------+       |
|                                                           |
+-----------------------------------------------------------+
```

| Property          | Type           | Description                                       | Default |
| ----------------- | -------------- | ------------------------------------------------- | ------- |
| `class`           | `&'static str` | CSS class for the accordion container.            | `""`    |
| `expanded_class`  | `&'static str` | CSS class for the expanded content.               | `""`    |
| `collapsed_class` | `&'static str` | CSS class for the collapsed content.              | `""`    |
| `content_class`   | `&'static str` | CSS class for the content section.                | `""`    |
| `style`           | `&'static str` | Custom inline styles for the accordion container. | `""`    |
| `expanded_style`  | `&'static str` | Custom inline styles for the expanded element.    | `""`    |
| `collapsed_style` | `&'static str` | Custom inline styles for the collapsed element.   | `""`    |
| `content_style`   | `&'static str` | Custom inline styles for the accordion content.   | `""`    |

### Callback Props

| Property     | Type           | Description                                     | Default |
| ------------ | -------------- | ----------------------------------------------- | ------- |
| `will_open`  | `Callback<()>` | Callback triggered before the accordion opens.  | No-op   |
| `did_open`   | `Callback<()>` | Callback triggered after the accordion opens.   | No-op   |
| `will_close` | `Callback<()>` | Callback triggered before the accordion closes. | No-op   |
| `did_close`  | `Callback<()>` | Callback triggered after the accordion closes.  | No-op   |

### A11Y Props

| Property        | Type           | Description                                                          | Default |
| --------------- | -------------- | -------------------------------------------------------------------- | ------- |
| `aria_controls` | `&'static str` | ARIA controls attribute for accessibility.                           | `""`    |
| `aria_enabled`  | `bool`         | Whether ARIA attributes are enabled for screen reader accessibility. | `true`  |

## 💡 Notes

- Use the `expand` prop to control the open/close state of the accordion programmatically.
- The `size` prop can be customized to adjust the width of the accordion.
- Callback props like `will_open` and `did_close` provide hooks to perform actions during the accordion's lifecycle.
- Custom classes and styles allow for extensive customization of the accordion's appearance and behavior.
- ARIA attributes can be enabled/disabled using the `aria_enabled` prop for better screen reader compatibility.
