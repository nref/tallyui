use yew::prelude::*;

pub struct TallyTableRow;

#[derive(Clone, PartialEq, Properties)]
pub struct TallyTableRowProps {
    pub name: AttrValue,
    pub count: i32,
    pub is_total_row: Option<bool>,
    pub on_incr_clicked: Callback<AttrValue>,
    pub on_decr_clicked: Callback<AttrValue>,
}

impl Component for TallyTableRow {
    type Message = ();
    type Properties = TallyTableRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let count = ctx.props().count;
        let is_total_row = ctx.props().is_total_row.unwrap_or(false);
        let name_attr = ctx.props().name.clone();
        let name_attr2 = name_attr.clone();
        let name_str = name_attr.to_string();

        let on_incr_clicked = ctx
            .props()
            .on_incr_clicked
            .reform(move |_| name_attr.clone());

        let on_decr_clicked = ctx
            .props()
            .on_decr_clicked
            .reform(move |_| name_attr2.clone());

        html! {
            <tr key={name_str.clone()}>
                <td></td>
                // <td><button>{ "🗑️" }</button></td>
                <td class={if is_total_row { "total-row" } else { "" }}>{name_str.clone()}</td>
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
