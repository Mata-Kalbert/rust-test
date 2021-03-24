use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;


#[derive(Default)]
struct Model {
    items: Vec<String>,
    error: Option<String>,
    new_todo_title: String,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    NewTodoTitleChanged(String),
    ClearTodoList,
    CreateNewTodoItem,
    //ClearItem(String),
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        }

        NewTodoTitleChanged(title) => {
            model.new_todo_title = title;
        }

        ClearTodoList => {
            model.items.clear()
        }

        CreateNewTodoItem => {
            let title = &model.new_todo_title;
            model.items.push(
            title.to_string(),  
            )
        }

        //ClearItem(item) => {
        //    model.items.remove(item);
        //}

    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["container"],
        div![
    
        img![
            attrs!{At::Src => "LVLogo_small.png"}
        ],],
        C!["image"],
        h1!["Add to List"],
        input![
            C!["new-todo"],
            attrs! {
                At::Placeholder => "Add item to list",
                At::AutoFocus => AtValue::None,
            },
            input_ev(Ev::Input, Msg::NewTodoTitleChanged),
            //keyboard_ev(Ev::KeyDown, |keyboard_event| {
            //    IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateNewTodoItem)
            //}),
        ],
        button![
            C!["submit"],
            ["Add"],
            ev(Ev::Click, |_| Msg::CreateNewTodoItem)
        ],
        button![
            C!["submit"],
            ["Delete All"],
            ev(Ev::Click, |_| Msg::ClearTodoList)
        ],
        ul![
            model.items.iter().map(|item| {
                li![item ,
                    button![
                    C!["deleteItem"],
                    [  "X"],
                    //ev(Ev::Click, |_| Msg:ClearItem(item))
                    ]
                ]
            })
        ]
    ]
}

async fn get_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
