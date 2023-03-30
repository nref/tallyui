use gloo::storage::LocalStorage;
use gloo_console::log;
use gloo_storage::Storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::prelude::*;

pub struct TallyTable {
    pub table: Table,
}

pub struct TallyTableRow;

#[derive(Clone, PartialEq, Properties)]
pub struct TallyTableRowProps {
    pub name: AttrValue,
    pub count: i32,
    pub is_total_row: Option<bool>,
    pub on_incr_clicked: Callback<AttrValue>,
    pub on_decr_clicked: Callback<AttrValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
    rows: HashMap<String, i32>,
}

fn get_table() -> Table {
    let table: Table = LocalStorage::get("table").unwrap_or_else(|_| {
        let mut rows = HashMap::new();
        rows.insert("EAs".to_string(), 0);
        rows.insert("Speech".to_string(), 0);
        rows.insert("Psych".to_string(), 0);
        rows.insert("Copiers".to_string(), 0);
        rows.insert("Misc".to_string(), 0);

        let default_table = Table { rows };

        save_table(&default_table);
        LocalStorage::get("table").unwrap_or(Table {
            rows: HashMap::new(),
        })
    });

    table
}

fn save_table(table: &Table) {
    LocalStorage::set("table", table).ok();
}

fn update_row(row: &String, count: i32) -> Table {
    let mut table = get_table();

    if let Some(entry) = table.rows.get_mut(row) {
        *entry += count;
    }

    save_table(&table);
    table
}

fn handle_clear() {
    log!("Clear All clicked");
    LocalStorage::delete("table");
    get_table();
}

fn handle_decrement(row: &String) {
    update_row(row, -1);
}

fn handle_increment(row: &String) {
    update_row(row, 1);
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

pub enum TallyMessage {
    Increment(AttrValue),
    Decrement(AttrValue),
    Clear,
}

impl Component for TallyTable {
    type Message = TallyMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { table: get_table() }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Increment(name) => {
                handle_increment(&name.to_string());
                if let Some(entry) = self.table.rows.get_mut(&name.to_string()) {
                    *entry += 1;
                }
                true
            }
            Self::Message::Decrement(name) => {
                handle_decrement(&name.to_string());
                if let Some(entry) = self.table.rows.get_mut(&name.to_string()) {
                    if *entry > 0 {
                        *entry -= 1;
                    }
                }
                true
            }
            Self::Message::Clear => {
                // Wipe the categories too:
                // handle_clear();
                // self.table.rows = HashMap::new();

                for (_key, value) in self.table.rows.iter_mut() {
                    *value = 0;
                }
                save_table(&self.table);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sum = self.table.rows.values().fold(0, |acc, &x| acc + x);

        html! {

            <>
                <table>
                    <tr>
                        <th></th>
                        <th>{"Category"}</th>
                        <th>{"Count"}</th>
                        <th></th>
                        <th></th>
                    </tr>
                    {
                        self.table.rows.iter().map(|(name, count)| {
                            let on_incr_clicked = ctx.link().callback(Self::Message::Increment);
                            let on_decr_clicked = ctx.link().callback(Self::Message::Decrement);

                            html! {
                                <TallyTableRow
                                    name={name.clone()}
                                    count={count}
                                    on_incr_clicked={on_incr_clicked}
                                    on_decr_clicked={on_decr_clicked} />
                            }

                        }).collect::<Html>()
                    }

                    // <tr>
                    //     <td><button>{"➕"}</button></td>
                    //     <td><input/></td>
                    // </tr>


                    <TallyTableRow
                        name={"Total"}
                        count={sum}
                        is_total_row={true}
                        on_incr_clicked={|_| {}}
                        on_decr_clicked={|_| {}}
                        />
                </table>
                <button class="button-dangerous"
                    onclick={ctx.link().callback(|_| Self::Message::Clear)}>{"Reset"}</button>
            </>
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <center>
                <h1>{"Tally"}</h1>
            </center>
            <TallyTable />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
