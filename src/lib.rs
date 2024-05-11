use std::rc::Rc;

use floem::{
    ext_event::create_signal_from_channel, reactive::ReadSignal, views::Decorators, IntoView,
};
use livebucket::{client::LVBClient, shared::KVPair};

pub fn watch<const N: usize, V: IntoView, F: FnOnce([ReadSignal<Option<Vec<KVPair>>>; N]) -> V>(
    conn: &LVBClient,
    queries: [&str; N],
    f: F,
) -> impl IntoView {
    let Ok(responses) = queries
        .into_iter()
        .map(|q| Rc::new(conn.watch(q)))
        .collect::<Vec<_>>()
        .try_into()
    else {
        unreachable!();
    };
    let responses: [_; N] = responses;

    let Ok(read_sigs) = responses
        .iter()
        .map(|response| create_signal_from_channel(response.rx.clone()))
        .collect::<Vec<_>>()
        .try_into()
    else {
        unreachable!();
    };

    let read_sigs: [_; N] = read_sigs;

    f(read_sigs).on_cleanup(move || drop(responses.clone()))
}
