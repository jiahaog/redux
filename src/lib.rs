pub struct Store<T, A> {
    reducer: Reducer<T, A>,
    data: T,
}

type Reducer<T, A> = fn(state: &T, action: A) -> T;

impl<T, A> Store<T, A> {
    pub fn create_store(reducer: Reducer<T, A>, data: T) -> Store<T, A> {
        Store { reducer, data }
    }

    pub fn dispatch(&mut self, action: A) {
        let result = (self.reducer)(&self.data, action);
        self.data = result;
    }

    pub fn get_state(&self) -> &T {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_state() {
        struct Action {}
        let reducer = |state: &i32, _action: Action| -> i32 { *state };
        let store = Store::create_store(reducer, 1);

        assert_eq!(store.get_state(), &1);
    }

    #[test]
    fn dispatch() {
        struct Action {}
        let reducer = |state: &i32, _action: Action| -> i32 { *state + 1 };
        let mut store = Store::create_store(reducer, 1);

        assert_eq!(store.get_state(), &1);

        store.dispatch(Action {});
        assert_eq!(store.get_state(), &2);
    }
}
