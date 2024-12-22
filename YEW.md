# Y Accordion RS Yew Usage

Adding Accordion RS to your project is simple:

1. Make sure your project is set up with **Yew**. Refer to their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Accordion component to your dependencies by including it in your `Cargo.toml` file.

   ```sh
   cargo add accordion-rs --features=yew
   ```

1. Import the `Accordion` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Yew Accordion into your application is easy. Follow these steps:

1. Import the Accordion component into your Yew project:

   ```rust
   use yew::prelude::*;
   use accordion_rs::yew::Accordion;
   ```

1. Define the accordion sections and use the Accordion component in your Yew component:

   ```rust
   use accordion_rs::yew::{Accordion, Item, List};
   use accordion_rs::{Size, Align};
   use yew::prelude::*;

   #[function_component(App)]
   pub fn app() -> Html {
       let on_will_open = Callback::from(|_| log::info!("Accordion will open!"));
       let on_did_open = Callback::from(|_| log::info!("Accordion did open!"));
       let on_will_close = Callback::from(|_| log::info!("Accordion will close!"));
       let on_did_close = Callback::from(|_| log::info!("Accordion did close!"));
       html! {
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
       }
   }
   ```

## 🔧 Props

### Main Props

| Property    | Type   | Description                                                         | Default         |
| ----------- | ------ | ------------------------------------------------------------------- | --------------- |
| `expanded`  | `Html` | Content to be displayed when the accordion is expanded.             | `""`            |
| `collapsed` | `Html` | Content to be displayed when the accordion is collapsed.            | `""`            |
| `size`      | `Size` | Size of the accordion component (e.g., `Small`, `Medium`, `Large`). | `Size::XXLarge` |
| `duration`  | `u64`  | Animation duration in milliseconds.                                 | `600`           |

### Callback Props

| Property     | Type           | Description                                     | Default |
| ------------ | -------------- | ----------------------------------------------- | ------- |
| `will_open`  | `Callback<()>` | Callback triggered before the accordion opens.  | No-op   |
| `did_open`   | `Callback<()>` | Callback triggered after the accordion opens.   | No-op   |
| `will_close` | `Callback<()>` | Callback triggered before the accordion closes. | No-op   |
| `did_close`  | `Callback<()>` | Callback triggered after the accordion closes.  | No-op   |

### Styling

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

| Property          | Type           | Description                                    | Default |
| ----------------- | -------------- | ---------------------------------------------- | ------- |
| `class`           | `&'static str` | CSS class for the accordion container.         | `""`    |
| `collapsed_class` | `&'static str` | CSS class for the collapsed section.           | `""`    |
| `expanded_class`  | `&'static str` | CSS class for the expanded section.            | `""`    |
| `content_class`   | `&'static str` | CSS class for the content section.             | `""`    |
| `style`           | `&'static str` | Inline CSS styles for the accordion container. | `""`    |
| `collapsed_style` | `&'static str` | Inline CSS styles for the collapsed section.   | `""`    |
| `expanded_style`  | `&'static str` | Inline CSS styles for the expanded section.    | `""`    |
| `content_style`   | `&'static str` | Inline CSS styles for the content section.     | `""`    |

## 💡 Notes

- The `size` prop controls the overall size of the accordion and can be set to `Small`, `Medium`, or `Large` to suit your design.
- The `expanded` and `collapsed` properties define the HTML content for each state, and the accordion can be styled via the `style`, `class`, and other CSS props.
- The animation for opening/closing the accordion is controlled via the `duration` prop, which is in milliseconds.
