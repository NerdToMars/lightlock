use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::marker::PhantomData;

use leptos::*;
use leptos::{add_event_listener, body, create_element, document, log, window};
use web_sys::{Event, MouseEvent};

#[derive(Default)]
struct Runtime {
    signal_values: RefCell<Vec<Box<RefCell<dyn Any>>>>,

    signal_subscribers: RefCell<HashMap<SignalId, HashSet<EffectId>>>,

    running_effect: Cell<Option<EffectId>>,

    effects: RefCell<Vec<Box<dyn Fn()>>>,
}

impl Runtime {
    pub fn create_signal<T: Copy + Clone + 'static>(&'static self, value: T) -> Signal<T> {
        self.signal_values
            .borrow_mut()
            .push(Box::new(RefCell::new(value)));

        let id = self.signal_values.borrow().len() - 1;
        Signal {
            cx: self,
            id: SignalId(id),
            ty: PhantomData,
        }
    }

    pub fn create_effect(&'static self, f: impl Fn() + 'static) {
        self.effects.borrow_mut().push(Box::new(f));

        let effect_id = EffectId(self.effects.borrow().len() - 1);
        self.run_effect(effect_id);
    }

    pub fn run_effect(&'static self, effect_id: EffectId) {
        let prev = self.running_effect.replace(Some(effect_id));

        let effect = &self.effects.borrow()[effect_id.0];
        effect();

        if prev.is_some() {
            self.running_effect.replace(prev);
        }
    }
}

#[derive(Clone, Copy)]
struct Signal<T> {
    cx: &'static Runtime,
    // value: T,
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T> Signal<T>
where
    T: Clone + 'static + Copy + fmt::Debug,
{
    /// this is called in create effect function
    /// create effect will add a effect first before calling this get function
    pub fn get(&self) -> T {
        log!("get signal id: {}", self.id.0);
        let v = &self.cx.signal_values.borrow()[self.id.0];
        let value = v.borrow().downcast_ref::<T>().unwrap().clone();
        log!("signal value borrowed in get function");

        if let Some(effect_id) = self.cx.running_effect.get() {
            log!("running effect id: {:?}", effect_id);

            let mut sub = self.cx.signal_subscribers.borrow_mut();
            log!("sub has signal ids: {:?}", sub.keys());
            let sub = sub.entry(self.id);
            let sub = sub.or_default();

            sub.insert(effect_id);
            log!("add effect id to signal subscribers");
        }

        log!("get value: {:?}", value);

        value
    }

    pub fn set(&self, value: T) {
        {
            log!("set signal id: {}", self.id.0);
            let v = &self.cx.signal_values.borrow()[self.id.0];
            log!("signal value borrowed");
            // *v.borrow_mut() = value;
            let mut v = v.borrow_mut();
            let value_cell = v.downcast_mut::<T>().unwrap();
            *value_cell = value;
            log!("set signal value");
        }

        let subs = {
            let binding = self.cx.signal_subscribers.borrow();
            binding.get(&self.id).cloned()
        };

        if let Some(subs) = subs {
            for effect_id in subs.iter() {
                self.cx.run_effect(*effect_id);
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SignalId(usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct EffectId(usize);

pub fn main() {
    // mount_to_body(|cx| view! { cx,  <SimpleCounter initial_value=3 /> })

    let body = body().unwrap();
    let p = create_element("p");
    p.set_text_content(Some("Hello, world!!!"));
    body.append_child(&p);

    let inc = create_element("button");
    inc.set_text_content(Some("Increment"));
    body.append_child(&inc);

    let dec = create_element("button");
    dec.set_text_content(Some("Decrement"));
    body.append_child(&dec);

    let p2 = create_element("p");
    body.append_child(&p2);

    // setup reactive system

    let cx: &'static Runtime = Box::leak(Box::default());
    let count = cx.create_signal(0);
    let double_count = move || count.get() * 2;

    cx.create_effect(move || {
        p.set_text_content(Some(&count.get().to_string()));
    });

    cx.create_effect(move || {
        p2.set_text_content(Some(&double_count().to_string()));
    });

    add_event_listener(&inc, "click", move |_: MouseEvent| {
        log!("Increment button clicked");
        let current = count.get();
        count.set(current + 1);
    });

    add_event_listener(&dec, "click", move |_: MouseEvent| {
        log!("Decrement button clicked");
        let current = count.get();
        count.set(current - 1);
    });
}
