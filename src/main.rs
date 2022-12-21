use yew::prelude::*;

enum Msg {
    Increment,
}

struct CounterApplication {
    counter: i64,
}

impl Component for CounterApplication {
    type Message = Msg;

    // this displays the view 
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.counter }</p>
                <button onclick={link.callback(|_| Msg::Increment)}>{ "+1" }</button>
            </div>
        }
    }

    // this initialises the application 
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            counter: 0,
        }
    }

    // this updates the counter 
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.counter += 1;
                true 
            }
        }
    }
}

fn main() {
    yew::start_app::<CounterApplication>();
}