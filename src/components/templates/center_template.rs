use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CenterTemplateProps {
    pub children: Children,
}

#[function_component(CenterTemplate)]
pub fn dropdown(props: &CenterTemplateProps) -> Html {
    html! {
        <div class="flex flex-col items-center justify-center px-6 py-8 mx-auto md:h-screen lg:py-0">
            {for props.children.iter()}
        </div>
    }
}
