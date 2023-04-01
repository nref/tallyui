use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

pub struct TallyTableRow {
    renaming: bool,
    name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TallyTableRowProps {
    pub name: AttrValue,
    pub count: i32,
    pub editing: bool,
    pub is_total_row: Option<bool>,
    pub on_incr_clicked: Option<Callback<AttrValue>>,
    pub on_decr_clicked: Option<Callback<AttrValue>>,
    pub on_remove_clicked: Option<Callback<AttrValue>>,
    pub on_name_changed: Option<Callback<String>>,
}

pub enum TallyTableRowMessage {
    Rename,
    RenameFinished(String),
}

fn get_value_from_keyboard_event(e: KeyboardEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    get_value_from_event(event)
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    get_value_from_event(event)
}

fn get_value_from_event(e: Event) -> String {
    let event_target = e.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

impl Component for TallyTableRow {
    type Message = TallyTableRowMessage;
    type Properties = TallyTableRowProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            renaming: false,
            name: ctx.props().name.to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Rename => {
                self.renaming = true;
                true
            }
            Self::Message::RenameFinished(new_name) => {
                self.name = new_name;
                self.renaming = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let on_name_changed = props
            .clone()
            .on_name_changed
            .unwrap_or(Callback::from(|_| {}));
        let count = props.count;
        let is_total_row = props.is_total_row.unwrap_or(false);
        let name_attr = AttrValue::from(self.name.clone());
        let name_attr2 = name_attr.clone();
        let name_attr3 = name_attr.clone();
        let name_str = name_attr.to_string();

        let editing_class = if props.editing {
            "editing"
        } else {
            "not-editing"
        };

        let default_callback = Callback::from(|_| {});

        let on_incr_clicked = props
            .on_incr_clicked
            .clone()
            .unwrap_or(default_callback.clone())
            .reform(move |_| name_attr.clone());

        let on_decr_clicked = props
            .on_decr_clicked
            .clone()
            .unwrap_or(default_callback.clone())
            .reform(move |_| name_attr2.clone());

        let on_remove_clicked = props
            .on_remove_clicked
            .clone()
            .unwrap_or(default_callback.clone())
            .reform(move |_| name_attr3.clone());

        let on_name_clicked = ctx.link().callback(|_| Self::Message::Rename);

        let oninput = Callback::from(move |e: InputEvent| {
            let value = get_value_from_input_event(e);
            on_name_changed.emit(value);
        });

        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let value = get_value_from_keyboard_event(e);
                Some(Self::Message::RenameFinished(value))
            } else {
                None
            }
        });

        html! {
            <tr key={name_str.clone()}>
                <td class={editing_class}>
                    if !is_total_row {
                        <button class={format!("button-edit-row {}", editing_class)} onclick={on_remove_clicked}>{ "🗑" }</button>
                    }
                </td>
                if !is_total_row && self.renaming {
                    <td><input type="text" value={name_str.clone()} {oninput} {onkeypress}/></td>
                } else
                {
                    <td class={if is_total_row { "total-row" } else { "" }} onclick={on_name_clicked}>{name_str.clone()}</td>
                }
                <td class={if is_total_row { "total-row" } else { "" }}>{count}</td>
                <td>
                    if !is_total_row {
                        <button class="button-increment" onclick={on_incr_clicked}>{ "↑" }</button>
                    }
                </td>
                <td>
                    if !is_total_row {
                        <button class="button-decrement" onclick={on_decr_clicked}>{ "↓" }</button>
                    }
                </td>
            </tr>
        }
    }
}
