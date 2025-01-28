use leptos::prelude::*;

pub struct MergedSignalOption<T: Send + Sync + 'static> {
  pub value: Option<ReadSignal<T>>,
}

pub fn use_merged_signal<T: Send + Sync + Clone + 'static>(
    default_value: T,
    option: MergedSignalOption<T>,
) -> (ReadSignal<T>, WriteSignal<T>) {
    let value = match &option.value {
        Some(value) => value.get(),
        None => default_value,
    };

    let (inner_signal, set_inner_signal) = signal(value);

    let merged_signal = match option.value {
        Some(value) => value,
        None => inner_signal,
    };

    (merged_signal, set_inner_signal)
}
