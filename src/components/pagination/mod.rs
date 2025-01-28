use std::cmp::min;

use leptos::{either::Either, prelude::*};

use crate::{
    classnames, components::button::Button, 
    hooks::use_merged_signal::{use_merged_signal, MergedSignalOption},
    utils::{callback::BoxOneCallback, dom::mount_style}
};



#[component]
pub fn Pagination(
    #[prop(optional, default = 1)] default_current: usize,
    #[prop(optional)] current: Option<ReadSignal<usize>>,
    #[prop(optional)] class: String,
    #[prop(optional)] on_click: Option<BoxOneCallback<usize>>,
    total: usize,
) -> impl IntoView {
    mount_style("pagination", include_str!("pagination.css"));
    let class_str = class.as_str();
    let pagination_class = classnames![
        "kapla-pagination",
        class_str
    ];

    let (current, set_current) = use_merged_signal(
        default_current, 
        MergedSignalOption { value: current }
    );

    // let on_click_prev = move |_| {
        // page_count.update(|val| *value + 1);
    // };


    let on_click = move |e| {
        let Some(on_click) = on_click.as_ref() else {
            return;
        };
        on_click(e);
    };


    view! {
        <div class=pagination_class>
            <Button>"Prev"</Button>
            {move || {
                use_pagination(current.get(), total, 1)
                .into_iter()
                .map(|item| {
                    if let PaginationItem::Number(num) = item {
                        Either::Left(view! {
                            <Button on_click=move |_| {
                                set_current.set(num);
                            }>{num}</Button>
                        })
                    } else {
                        Either::Right(view! {
                            <Button>"..."</Button>
                        })
                    }
                }).collect_view()
            }}
            <Button>"Next"</Button>
        </div>
    }
}

fn range(start: usize, end: usize) -> Vec<PaginationItem> {
    let mut ret = vec![];
    for idx in start..=end {
        ret.push(PaginationItem::Number(idx));
    }
    ret
}

enum PaginationItem {
    DotLeft,
    DotRight,
    Number(usize),
}

fn use_pagination(page: usize, count: usize, sibling_count: usize) -> Vec<PaginationItem> {
    let total_page_numbers = sibling_count + 5;

    if total_page_numbers >= count {
        return range(1, count);
    }

    let current_page = page;
    let left_sibling_index = if current_page > sibling_count + 1 {
        current_page - sibling_count
    } else {
        1
    };
    let right_sibling_index = min(current_page + sibling_count, count);
    // We do not show dots just when there is just one page number to be inserted between the extremes of sibling and the page limits i.e 1 and totalPageCount.
    // Hence we are using leftSiblingIndex > 2 and rightSiblingIndex < totalPageCount - 2
    let should_show_left_dots = left_sibling_index > 2;
    let should_show_right_dots = right_sibling_index < count - 2;

    let first_page_index = 1;
    let last_page_index = count;

    // Case 2: No left dots to show, but rights dots to be shown
    if !should_show_left_dots && should_show_right_dots {
        let left_item_count = 3 + 2 * sibling_count;
        let mut left_range = range(1, left_item_count);
        left_range.push(PaginationItem::DotRight);
        left_range.push(PaginationItem::Number(count));
        left_range
    } else if should_show_left_dots && !should_show_right_dots {
        // Case 3: No right dots to show, but left dots to be shown
        let right_item_count = 3 + 2 * sibling_count;
        let mut right_range = range(count - right_item_count + 1, count);
        let mut ret = vec![
            PaginationItem::Number(first_page_index),
            PaginationItem::DotLeft,
        ];
        ret.append(&mut right_range);
        ret
    } else {
        // Case 4: Both left and right dots to be shown
        let mut middle_range = range(left_sibling_index, right_sibling_index);
        let mut range = vec![
            PaginationItem::Number(first_page_index),
            PaginationItem::DotLeft,
        ];
        range.append(&mut middle_range);
        range.append(&mut vec![
            PaginationItem::DotRight,
            PaginationItem::Number(last_page_index),
        ]);
        range
    }
}