use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownProps {
    pub children: Children,
    #[prop_or(false)]
    pub default: bool,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    let is_open = use_state(|| props.default);

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class="relative">
            <button class="flex flex-row items-center w-full px-4 py-2 mt-2 text-sm font-semibold text-left bg-transparent rounded-lg dark-mode:bg-transparent dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:focus:bg-gray-600 dark-mode:hover:bg-gray-600 md:block hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline" {onclick}>
                <span>{ "Dropdown" }</span>
                <svg fill="currentColor" class="inline w-4 h-4 mt-1 ml-1 transition-transform duration-200 transform md:-mt-1"><path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
            </button>
            { match *is_open {
                // TODO - revert "x-transition-enter" to "x-transition:enter" when possible
                true => html!{
                    <div x-transition-enter="transition ease-out origin-top-left duration-200"
                        x-transition-enter-start="opacity-0 transform scale-90"
                        x-transition-enter-end="opacity-100 transform scale-100"
                        x-transition-leave="transition origin-top-left ease-in duration-100"
                        x-transition-leave-start="opacity-100 transform scale-100"
                        x-transition-leave-end="opacity-0 transform scale-90" class="px-2 py-2 bg-white rounded-md shadow dark-mode:bg-gray-800">
                        { for props.children.iter() }
                    </div>}
                ,
                false => html!{<></>},
            }}
        </div>
    }
}
