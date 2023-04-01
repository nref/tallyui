use yew::prelude::*;

use crate::components::tally_table_row::TallyTableRow;
use crate::data::table_repo::TableRepo;
use crate::model::table::Table;
use gloo_console::log;

pub enum TallyMessage {
    Increment(AttrValue),
    Decrement(AttrValue),
    Reset,
    Edit,
    RemoveRow(AttrValue),
    AddRow,
}

pub struct TallyTable {
    pub table: Table,
    pub repo: TableRepo,

    editing: bool,
}

impl Component for TallyTable {
    type Message = TallyMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let repo = TableRepo {};
        Self {
            editing: false,
            table: repo.get_table(),
            repo,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Increment(name) => {
                self.repo.increment_row(&name.to_string());
                if let Some(entry) = self.table.rows.get_mut(&name.to_string()) {
                    *entry += 1;
                }
                true
            }
            Self::Message::Decrement(name) => {
                self.repo.decrement_row(&name.to_string());
                if let Some(entry) = self.table.rows.get_mut(&name.to_string()) {
                    if *entry > 0 {
                        *entry -= 1;
                    }
                }
                true
            }
            Self::Message::Reset => {
                for (_key, value) in self.table.rows.iter_mut() {
                    *value = 0;
                }
                self.repo.save_table(&self.table);
                true
            }
            Self::Message::Edit => {
                self.editing = !self.editing;
                true
            }
            Self::Message::RemoveRow(name) => {
                log!(format!("Removing row {}", &name.to_string()));
                self.table.rows.remove(&name.to_string());
                log!("Rows:");
                for (k, v) in self.table.rows.iter() {
                    log!(format!("  {}", k));
                }
                true
            }
            Self::Message::AddRow => {
                let name = "New category".to_string();
                let mut name2 = name.to_string();

                let mut i = 2;
                while self.table.rows.contains_key(&name2) {
                    name2 = format!("{} {}", name, i);
                    i += 1;
                }

                self.table.rows.insert(name2, 0);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let sum = self.table.rows.values().fold(0, |acc, &x| acc + x);
        let link = ctx.link();

        let editing_class = if self.editing {
            "editing"
        } else {
            "not-editing"
        };

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
                            html! {
                                <TallyTableRow
                                    name={name.clone()}
                                    count={count}
                                    editing={self.editing}
                                    on_remove_clicked={link.callback(Self::Message::RemoveRow)}
                                    on_incr_clicked={link.callback(Self::Message::Increment)}
                                    on_decr_clicked={link.callback(Self::Message::Decrement)} />
                            }

                        }).collect::<Html>()
                    }

                    <tr class={editing_class}>
                        <td><button class="button-edit-row" onclick={link.callback(|_| Self::Message::AddRow)}>{"➕"}</button></td>
                    </tr>


                    <TallyTableRow
                        name={"Total"}
                        count={sum}
                        is_total_row={true}
                        editing={false}/>
                </table>

                <button class="button-outside-table" onclick={link.callback(|_| Self::Message::Edit)}>
                    {if self.editing { "Stop Editing" } else { "Edit" }}
                </button>
                <button class="button-reset-table" onclick={link.callback(|_| Self::Message::Reset)}>
                    {"Reset"}
                </button>
            </>
        }
    }
}
