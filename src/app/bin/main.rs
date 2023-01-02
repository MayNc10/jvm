#[cfg(target_family = "wasm")]
mod app {

use std::collections::HashMap;
use std::iter::zip;

use lazy_static::lazy_static;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use gloo_console::log;
use rust_jvm::class::classfile::ClassFile;
use rust_jvm::constant_pool::Entry;
use rust_jvm::errorcodes::Error;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use wasm_bindgen::JsValue;
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

use std::fmt::Display;
//use std::fs::File;

use yew::prelude::*;
use yew::Properties;

#[derive(Clone, Properties, PartialEq)]
pub struct DisplayTableProps<T> 
where 
    T: PartialEq,
    T: Clone,
    T: Display,
{
    pub header: String,
    pub labels: Vec<String>,
    pub data: Vec<Vec<T>>,
    pub table_size_str: &'static str,
    #[prop_or("".to_string())]
    pub position_class: String
}

#[function_component]
fn DisplayTable<T>(props: &DisplayTableProps<T>) -> Html 
where
    T: PartialEq,
    T: Clone,
    T: Display,
{
    html! {
        <div class={
            classes!("absolute", "w-1/2", "rounded-2xl", "gradient", "bg--to-r", 
            "from-pink-500", "via-red-500", "to-yellow-500", "p-1", "shadow-xl", props.position_class.clone())}>
            <a class="block rounded-xl bg-white p-6 sm:p-8 " href="">
                //<div class="mt-16 sm:pr-8 ">
                    <h3 class="text-xl font-bold text-gray-900"> {props.header.clone()} </h3>
                    <div class={classes!("overflow-x-auto","overflow-y-auto", "h-[25vh]")}>
                        <table class="min-w-full divide-y divide-gray-200 text-sm">                        
                            <thead class="bg-gray-100">     
                                <tr>
                                    {
                                        props.labels.clone().into_iter().map(|l| {
                                            html! {
                                                <th class="whitespace-nowrap px-4 py-2 text-left font-medium text-gray-900"> {l} </th>
                                            }
                                        }).collect::<Html>()
                                    }
                                </tr>
                            </thead>
                            
                            <tbody class="divide-y divide-gray-200">
                                {props.data.clone().into_iter().map(|t| {
                                    html!{
                                        <tr>
                                            {t.clone().into_iter().map(|data| {
                                                html! {
                                                    <td class="whitespace-nowrap px-4 py-2 font-medium text-gray-900"> {data} </td>
                                                }
                                            }).collect::<Html>()}
                                        </tr>
                                    }
                                }).collect::<Html>()}
                            </tbody>
                        </table>
                    </div>   
                //</div>
            </a>
        </div>
    }
}


#[derive(Clone, Properties, PartialEq)]
pub struct LabeledBadgeProps {
    pub header: String,
    pub labels: Vec<String>,
    pub data: Vec<String>,
}

#[function_component]
fn LabeledBadge(props: &LabeledBadgeProps) -> Html {
    let weaved = zip(props.labels.clone(), props.data.clone());

    html! {
        <div class="w-1/2 rounded-2xl bg-gradient-to-r from-pink-500 via-red-500 to-yellow-500 p-1 shadow-xl">
            <a class="block rounded-xl bg-white p-6 sm:p-8" href="">
                <div class="mt-16 sm:pr-8">
                <h3 class="text-xl font-bold text-gray-900"> {props.header.clone()} </h3>
                {
                    weaved.map(|(label, data)| {
                        html! {
                            <p class="mt-2 text-sm text-gray-500"> {format!("{}: {}", label, data)} </p>
                        }
                    }).collect::<Html>()
                }
                </div>
            </a>
        </div>

    }
}


struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
}

pub struct App {
    readers: HashMap<String, FileReader>,
    class: Option<ClassFile>,
    error: Option<Error>,
}

lazy_static! {
    static ref GENERAL_LABELS: Vec<String> = vec![
        "Name",
        "Major version", "Minor version",
        "Number of constant pool entries",
        "Access flags", "Super class",
        "Number of interfaces",
        "Number of fields",
        "Number of methods"

    ].into_iter().map(|s| String::from(s)).collect();
    static ref GENERAL_HEADER: String = String::from("General Info");
    static ref CP_LABELS: Vec<String> = vec![
        "Index", "Type", "Data"
    ].into_iter().map(|s| String::from(s)).collect();
    static ref FIELDS_POSITION: String = "top-0".to_string();
}

static CP_TABLE_SIZE: usize = 25;

lazy_static! {
    static ref CP_TABLE_SIZE_STRING: String = format!("h-[{}vh]", CP_TABLE_SIZE);
    static ref CP_TABLE_SIZE_STR: &'static str = CP_TABLE_SIZE_STRING.as_str();
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            class: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, _file_type, data) => {
                let res =  unsafe { ClassFile::new(data.as_slice()) };
                if res.is_err() {
                    self.error = Some(res.unwrap_err());
                } else {
                    let (class, _) = res.unwrap();
                    self.class = Some(class);
                }
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.error.is_some() {
            html! {<p> {format!("Encountered error: {:?}", self.error.as_ref().unwrap())} </p>}
        }
        else if self.class.is_none() {
            /* 
            html! {
                <div id="wrapper">
                    <p id="title">{ "Upload Your .class File" }</p>
                    <label for="file-upload">
                        <div
                            id="drop-container"
                            ondrop={ctx.link().callback(|event: DragEvent| {
                                event.prevent_default();
                                let files = event.data_transfer().unwrap().files();
                                Self::upload_files(files)
                            })}
                            ondragover={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                            ondragenter={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                        >
                            <i class="fa fa-cloud-upload"></i>
                            <p>{"Drop your class file here or click to select"}</p>
                        </div>
                    </label>
                    <input
                        id="file-upload"
                        type="file"
                        accept="*"
                        multiple={true}
                        onchange={ctx.link().callback(move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            Self::upload_files(input.files())
                        })}
                    />
                </div>
            }
            */

            html! {
                <div class="flex items-center justify-center w-full min-h-screen">
                    <label for="dropzone-file" class="flex flex-col items-center justify-center w-1/2 h-1/2 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600">
                        <div class="flex flex-col items-center justify-center pt-5 pb-6"
                            ondrop={ctx.link().callback(|event: DragEvent| {
                                event.prevent_default();
                                let files = event.data_transfer().unwrap().files();
                                Self::upload_files(files)
                            })}
                            ondragover={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                            ondragenter={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                        })}
                        >
                            <svg aria-hidden="true" class="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"></path></svg>
                            <p class="mb-2 text-sm text-gray-500 dark:text-gray-400"><span class="font-semibold"> {"Click to upload"} </span> {" or drag and drop"} </p>
                        </div>
                        <input 
                            id="dropzone-file" 
                            type="file" 
                            class="hidden" 
                            accept="*"
                            onchange={ctx.link().callback(move |e: Event| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Self::upload_files(input.files())
                            })}
                        />
                    </label>
                </div> 
            }
        }
        else {
            let class = self.class.as_ref().unwrap();
            let general_data = vec![
            String::from(class.name()), format!("{}", class.major_version()), format!("{}", class.minor_version()), 
            format!("{}", class.cp_entries().len()), format!("{}", class.access_flags()), format!("{:?}", class.super_name()),
            format!("{}", class.interfaces().len()), format!("{}", class.fields().len()), format!("{}", class.methods().len())];

            let cp_entries = class.cp_entries().clone();
            let mut idx = 0;
            let cp_entries: Vec<Vec<String>> = cp_entries.into_iter().map(|entry| {
                idx += 1;
                vec![format!("{}", idx - 1), format!("{entry}")]
            }).collect();
            
            let fields: Vec<Vec<String>> = class.fields().clone().into_iter().map(|field| {
                idx += 1;
                vec![format!("{}", idx - 1), format!("{field:?}")]
            }).collect();

            html! {
                <>
                    <LabeledBadge header={GENERAL_HEADER.clone()} labels={GENERAL_LABELS.clone()} data={general_data} />
                    <DisplayTable<String> header={String::from("Constant Pool")} labels={CP_LABELS.clone()} 
                    data={cp_entries} table_size_str={&*CP_TABLE_SIZE_STR}/>

                    //<DisplayTable<String> header={String::from("Fields")} labels={CP_LABELS.clone()} 
                    //    data={fields} table_size={CP_TABLE_SIZE} position_class={FIELDS_POSITION.clone()} />
                </>
            }
        }
        
    }
}

impl App {
    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
}

}

#[cfg(target_family = "wasm")]
fn main() {
    use crate::app::App;
    yew::Renderer::<App>::new().render();
}

#[cfg(not(target_family = "wasm"))] 
fn main() {}
