extern crate redux;

#[test]
fn add_todo() {
    enum Action {
        AddTodo(String),
        Nothing,
    }

    let reducer = |state: &Vec<String>, action: Action| -> Vec<String> {
        let mut new_state = state.clone();
        match action {
            Action::AddTodo(new_todo) => {
                new_state.push(new_todo);
                new_state
            }
            Action::Nothing => new_state,
        }
    };

    let init_state: Vec<String> = Vec::new();
    let mut store = redux::Store::create_store(reducer, init_state);

    assert_eq!(store.get_state(), &<Vec<String>>::new());

    store.dispatch(Action::AddTodo(String::from("do something")));
    assert_eq!(store.get_state(), &vec!["do something"]);
}
