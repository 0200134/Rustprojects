use yew::prelude::*;

enum Msg {
    Click,
}

struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            link: ComponentLink::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Click => {
                // Handle click event
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.link().callback(|_| Msg::Click)}>
                {"Click me"}
            </button>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
