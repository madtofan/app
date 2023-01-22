use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProtectedPageProps {
    pub children: Children,
}

#[function_component(ProtectedPage)]
pub fn protected_page(props: &ProtectedPageProps) -> Html {
    html! {
        <>
            {for props.children.iter()}
        </>
    }
}
