use yew::{prelude::*, web_sys};

enum Msg {
    // Input输入事件
    InputChange(String),
    AddList,
}

struct Model {
    link: ComponentLink<Self>,
    value: String,
    dataList: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    /// 页面创建时触发的事件
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::from(""),
            dataList: vec![String::from("初始数据")],
        }
    }
    /// 页面中的相关数据发生变化时，需要更新页面
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputChange(_val) => {
                self.value = _val;
                true
            }
            Msg::AddList => {
                // 数据为空则不添加
                if self.value.is_empty() {
                    return true;
                }
                self.dataList.push(self.value.clone());
                self.value = String::from("");
                true
            }
        }
    }
    /// 父组件传递过来的props发生变化时，在这里返回true进行页面的更新
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
             <div class="container">
                <div class="header">
                    <input
                        class="ui-input w-200"
                        value={self.value.clone()}
                        oninput=self.link.callback(|e: InputData| Msg::InputChange(String::from(e.value)))
                        onkeypress=self.link.batch_callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { Some(Msg::AddList) } else {None}
                        })
                        />

                    <button class="ui-button m-l-20" datatype="danger" onclick=self.link.callback(|_|Msg::AddList)>{ "添加" }</button>
                </div>

                <div class="list-container">
                     {for self.dataList.iter().map(|item|self.view_list(item))}
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_list(&self, item: &String) -> Html {
        html! {
            <p class="list-item">
                {item}
            </p>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
