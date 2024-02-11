use std::cmp::min;
use std::time::Duration;
use lazy_static::lazy_static;
use gloo_timers::future::TimeoutFuture;
use sycamore::futures::spawn_local_scoped;
use rand::Rng;
use sycamore::easing;
use sycamore::motion::create_tweened_signal;
use sycamore::prelude::*;
use sycamore::web::html::ev::progress;
use web_sys::Event;

lazy_static! {
    static ref SCALES: Vec<Scale> = vec![
        Scale::new("E0"),
        Scale::new("F0"),
        Scale::new("G0"),
        Scale::new("A1"),
        Scale::new("B1"),
        Scale::new("C1"),
        Scale::new("D1"),
        Scale::new("F1"),
        Scale::new("G1"),
        Scale::new("A2"),
        Scale::new("B2"),
        Scale::new("C2"),
        Scale::new("D2"),
        Scale::new("F2"),
        Scale::new("G2"),
    ];
}


fn display_tab() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let tab = document.query_selector("#tab").unwrap();
    tab.unwrap().class_list().remove_1("hidden").unwrap();
}


#[component]
fn App<G: Html>() -> View<G> {
    let mut scale_index_state = create_signal(0usize);
    let mut timer = create_signal(0);
    let mut display_tab_duration = create_signal(3);
    let mut duration = create_signal(6);

    let scale_progress = create_tweened_signal(0f64, Duration::from_secs_f64(1f64), easing::quint_out);
    let tab_progress = create_tweened_signal(0f64, Duration::from_secs_f64(1f64), easing::quint_out);


    let hide_tab = move ||{
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let tab = document.query_selector("#tab").unwrap();
        tab.unwrap().class_list().add_1("hidden");
        tab_progress.set(0f64);
    };
    let do_next = move || {
        let mut rnd = rand::thread_rng();
        let count = rnd.gen_range(0usize..SCALES.len());
        scale_index_state.set(count);
        timer.set(0);
        scale_progress.set(0f64);
        hide_tab();
    };
    let on_click = move |_| {
        do_next();
    };


    spawn_local_scoped(async move {
        loop {
            TimeoutFuture::new(1000).await;
            timer += 1;

            let pro: f64 = ((timer.get()) as f64 / display_tab_duration.get() as f64).clamp(0f64, 1f64);
            scale_progress.set(pro);

            let tab: f64 = ((timer.get() - display_tab_duration.get()) as f64 / (duration.get() - display_tab_duration.get()) as f64).clamp(0f64, 1f64);
            tab_progress.set(tab);

            if timer == display_tab_duration {
                display_tab();
            }

            if timer >= duration {
                do_next();
            }
        }
    });


    let mut duration_bind_value = create_signal(6f64);
    let mut display_tab_duration_bind_value = create_signal(3f64);
    let on_duration_input = move |value: Event| {
        duration.set(duration_bind_value.get().round() as i32);
    };
    let on_display_tab_duration_input = move |value: Event| {
        let it = display_tab_duration_bind_value.get();
        let min = min(it.round() as i32, duration.get());
        display_tab_duration_bind_value.set(min as f64);
        display_tab_duration.set(min);
    };


    view! {
        div {
            div(class="settings"){
                div(class="item"){
                    label{"Duration:" (duration.get())}
                    input(bind:valueAsNumber=duration_bind_value,on:input=on_duration_input,type="number")
                }
                div(class="item"){
                    label{"Tab Display Duration:" (display_tab_duration.get())}
                    input(bind:valueAsNumber=display_tab_duration_bind_value,on:input=on_display_tab_duration_input,type="number")
                }
            }
            div(class="scale-part") {
                h1{"SCALE"}
                img(src=(SCALES.get(scale_index_state.get()).unwrap().image_path()))
                progress(id="scale-progress-bar",max="1",value=(scale_progress.get()))
                p(class="info") {
                    "Value: "
                    (SCALES.get(scale_index_state.get()).unwrap().name)
                }

                div(id="tab"){
                    img(src=(SCALES.get(scale_index_state.get()).unwrap().tab_image_path()))
                    div{
                        label {"TAB"}
                        progress(id="tab-progress-bar",max="1",value=(tab_progress.get()))
                    }
                }

                button(on:click=on_click) { "Next" }
            }

        }
    }
}

fn main() {
    sycamore::render(App);
}

#[derive(Copy, Clone)]
pub struct Scale {
    name: &'static str,
}

impl Scale {
    fn new(name: &'static str) -> Self {
        Self {
            name,
        }
    }
    fn image_path(&self) -> String {
        format!("./assets/images/{}.png", self.name)
    }
    fn tab_image_path(&self) -> String {
        format!("./assets/images/{}_tab.png", self.name)
    }
}

