use std::fmt::Display;

use yew::prelude::*;
use yew::Properties;

#[derive(Clone, Properties, PartialEq)]
pub struct DisplayTableProps<T> 
where 
    T: PartialEq,
    T: Clone,
    T: Display,
{
    pub data: Vec<T>,
}

#[function_component]
fn DisplayTable<T>(props: &DisplayTableProps<T>) -> Html 
where
    T: PartialEq,
    T: Clone,
    T: Display,
{
    html! {
        <div class="overflow-x-auto">
            <table class="min-w-full divide-y divide-gray-200 text-sm">
                /* 
                <thead class="bg-gray-100">     
                    <tr>
                        <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"> {"Name"} </th>
                    </tr>
                </thead>
                */
                <tbody class="divide-y divide-gray-200">
                    { 
                        props.data.clone().into_iter().map(|t| {
                            html!{
                                <tr>
                                <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900"> {t} </td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>   
    }
}




#[function_component]
fn App() -> Html {
    html! {<DisplayTable<String> data={vec![String::from("May Neelon"), String::from("Miller Dumas")]}/>}
}

fn main() {
    yew::Renderer::<App>::new().render();
}