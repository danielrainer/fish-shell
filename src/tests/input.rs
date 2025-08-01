use crate::env::EnvStack;
use crate::input::{EventQueuePeeker, InputMappingSet, KeyNameStyle, DEFAULT_BIND_MODE};
use crate::input_common::{CharEvent, InputData, InputEventQueuer, KeyEvent, TerminalQuery};
use crate::key::Key;
use crate::wchar::prelude::*;
use std::cell::{RefCell, RefMut};

struct TestInputEventQueuer {
    input_data: InputData,
    blocking_query: RefCell<Option<TerminalQuery>>,
}

impl InputEventQueuer for TestInputEventQueuer {
    fn get_input_data(&self) -> &InputData {
        &self.input_data
    }
    fn get_input_data_mut(&mut self) -> &mut InputData {
        &mut self.input_data
    }
    fn blocking_query(&self) -> RefMut<'_, Option<TerminalQuery>> {
        self.blocking_query.borrow_mut()
    }
}

#[test]
fn test_input() {
    let vars = EnvStack::new();
    let mut input = TestInputEventQueuer {
        input_data: InputData::new(i32::MAX), // value doesn't matter since we don't read from it
        blocking_query: RefCell::new(None),
    };
    // Ensure sequences are order independent. Here we add two bindings where the first is a prefix
    // of the second, and then emit the second key list. The second binding should be invoked, not
    // the first!
    let prefix_binding: Vec<Key> = "qqqqqqqa".chars().map(Key::from_raw).collect();
    let mut desired_binding = prefix_binding.clone();
    desired_binding.push(Key::from_raw('a'));

    let default_mode = || DEFAULT_BIND_MODE.to_owned();

    let mut input_mappings = InputMappingSet::default();
    input_mappings.add1(
        prefix_binding,
        KeyNameStyle::Plain,
        WString::from_str("up-line"),
        default_mode(),
        None,
        true,
    );
    input_mappings.add1(
        desired_binding.clone(),
        KeyNameStyle::Plain,
        WString::from_str("down-line"),
        default_mode(),
        None,
        true,
    );

    // Push the desired binding to the queue.
    for key in desired_binding {
        input
            .input_data
            .queue_char(CharEvent::from_key(KeyEvent::from(key)));
    }

    let mut peeker = EventQueuePeeker::new(&mut input);
    let mapping = peeker.find_mapping(&vars, &input_mappings);
    assert!(mapping.is_some());
    assert!(mapping.unwrap().commands == ["down-line"]);
    peeker.restart();
}
