use yew::prelude::*;

use crate::components::tally_table_row::TallyTableRow;
use crate::data::table_repo::TableRepo;
use crate::model::table::Table;

pub enum TallyMessage {
    Increment(AttrValue),
    Decrement(AttrValue),
    Clear,
}

pub struct TallyTable {
    pub table: Table,
    pub repo: TableRepo,
}

impl Component for TallyTable {
    type Message = TallyMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let repo = TableRepo {};
        Self {
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
            Self::Message::Clear => {
                // Wipe the categories too:
                // handle_clear();
                // self.table.rows = HashMap::new();

                for (_key, value) in self.table.rows.iter_mut() {
                    *value = 0;
                }
                self.repo.save_table(&self.table);
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
