use yew::prelude::*;
enum Message {
    Add,
    Sub,
    Change,
}
struct App {
    value: i32,
    websites: Vec<Website>,
}
impl App {
    fn get_website(&self) -> Option<Html> {
        let num = self.value.clone() as usize;
        if num < self.websites.len() {
            Some(html!(
                <>
                    <p>{"Nom: "} {self.websites[num].name.to_string()}</p>
                    <iframe src= {self.websites[num].link.to_string() } width="400" height="400"></iframe>
                    <p>{"Actif: "}{self.websites[num].active}</p>
                </>
            ))
        } else {
            None
        }
    }
}
struct Website {
    name: String,
    link: String,
    active: bool,
}
impl Website {
    fn new(name: String, link: String, active: bool) -> Self {
        Self { name, link, active }
    }
}

impl Component for App {
    type Message = Message;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
            websites: vec![
                Website::new(String::from("NF ISO 2600"), String::from("https://www.boutique.afnor.org/fr-fr/norme/nf-iso-26000/lignes-directrices-relatives-a-la-responsabilite-societale/fa142230/1106"), false),
                Website::new(String::from("FD X30-021"), String::from("https://www.boutique.afnor.org/fr-fr/norme/fd-x30021/sd-21000-developpement-durable-responsabilite-societale-des-entreprises-gui/fa125485/60046"), false),
                Website::new(String::from("NF EN ISO 13485"), String::from("https://www.boutique.afnor.org/fr-fr/norme/nf-en-iso-13485/dispositifs-medicaux-systemes-de-management-de-la-qualite-exigences-a-des-f/fa161550/1575"), false),
                Website::new(String::from("FD X30-021 2"), String::from("https://www.boutique.afnor.org/fr-fr/norme/nf-en-iso-13485/dispositifs-medicaux-systemes-de-management-de-la-qualite-exigences-a-des-f/fa177846/39718"), false),
                Website::new(String::from("FD X30-021 2"), String::from("https://www.boutique.afnor.org/fr-fr/norme/nf-en-iso-13485/dispositifs-medicaux-systemes-de-management-de-la-qualite-exigences-a-des-f/fa161550/1575"), false),
                Website::new(String::from("FD EN ISO 13485 2"), String::from("https://www.boutique.afnor.org/fr-fr/norme/nf-en-iso-13485/dispositifs-medicaux-systemes-de-management-de-la-qualite-exigences-a-des-f/fa177846/39718"), false),
                Website::new(String::from("FD X30-021 3"), String::from("https://www.boutique.afnor.org/fr-fr/norme/fd-x30021/sd-21000-developpement-durable-responsabilite-societale-des-entreprises-gui/fa125485/60046"), false),
            ],
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Add => {
                if self.value + 1 < self.websites.len() as i32 {
                    self.value += 1;
                    true
                } else {
                    false
                }
            }
            Message::Sub => {
                if self.value - 1 >= 0i32 {
                    self.value -= 1;
                    true
                } else {
                    false
                }
            }
            Message::Change => {
                let index = self.value as usize;
                if index < self.websites.len() {
                    self.websites[index].active = !self.websites[index].active;
                    true
                } else {
                    false
                }
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Update normes"}</h1>
                <button onclick={ctx.link().callback(|_| Message::Sub)}>{"Previous norm"}</button>
                <button onclick={ctx.link().callback(|_| Message::Add)}>{"Next norm"}</button>
                <p>{"Norme n°"}{self.value + 1}{" sur "}{self.websites.len()}</p>
                <p>{self.get_website()}</p>
                <button onclick={ctx.link().callback(|_| Message::Change)}>{"CHANGE STATUS"}</button>
            </div>
        }
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
