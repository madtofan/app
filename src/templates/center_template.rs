use yew::{function_component, html, use_state, Callback, Children, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct CenterTemplateProps {
    pub children: Children,
}

#[function_component(CenterTemplate)]
pub fn dropdown(props: &CenterTemplateProps) -> Html {
    html! {
        <div>
    </div>
    }
}
