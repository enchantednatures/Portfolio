use leptos::*;
use web_sys::MouseEvent;

#[component]
fn Plot() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    let (y, set_y) = create_signal(0);
    view! {
        <div
            on:mousemove=move |e: MouseEvent| {
                set_x.set(e.x());
                set_y.set(e.y());
            }
            style="position: absolute;"
            style="width: 25%;"
            style="height: 100vh;"
            // style:top=move || format!("{}px", y.get() + 100)
            // style:left=move || format!("{}px", x.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 255)", x.get(), y.get())
            style=("--columns", x)
        >
             <App/>
        </div>
    }
}

#[component]
fn ProgressBar<F>(#[prop()] max: u16, progress: F) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! {
        <progress value=progress max=max></progress>
    }
}

#[component]
fn App() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counterProgress = counters.map(|counter| {
        let (count, set_count) = counter;
        view! {

            <button
                on:click= move |_| {
                    set_count.update(| x| *x = &*x*2);
                }
                style= "font-size: 20px; padding: 10px; background: #eee; border: 1px solid #ddd; border-radius: 5px;"
                class:red=move || count.get() % 2 == 1
            >
                    { "Click me: " }
            { count }
            </button>
            <ProgressBar max=100 progress=move || count.get() />
        }
    }).collect_view();

    view! {
        { counterProgress }
    }
}

fn main() {
    mount_to_body(|| {
        view! { <App /> }
    })
}
