use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct InputProps {
    #[props(default = InputTheme::Default)]
    theme: InputTheme,
    #[props(default = InputType::Text)]
    r#type: InputType,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            class: props.theme.get_classes(),
            r#type: props.r#type.get_str(),
            oninput: move |evt| {
                props.oninput.inspect(|oninput| oninput(evt));
            },
            onchange: move |evt| {
                props.onchange.inspect(|onchange| onchange(evt));
            },
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl InputType {
    pub fn get_str(&self) -> &str {
        match self {
            InputType::Button => "button",
            InputType::Checkbox => "checkbox",
            InputType::Color => "color",
            InputType::Date => "date",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Email => "email",
            InputType::File => "file",
            InputType::Hidden => "hidden",
            InputType::Image => "image",
            InputType::Month => "month",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Radio => "radio",
            InputType::Range => "range",
            InputType::Reset => "reset",
            InputType::Search => "search",
            InputType::Submit => "submit",
            InputType::Tel => "tel",
            InputType::Text => "text",
            InputType::Time => "time",
            InputType::Url => "url",
            InputType::Week => "week",
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum InputTheme {
    Default,
}

impl InputTheme {
    pub fn get_classes(&self) -> &str {
        match self {
            InputTheme::Default => "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
        }
    }
}
