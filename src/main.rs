#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use facet::Facet;
use facet_pretty::FacetPretty;
use facet_reflect::Wip;
use freya::prelude::*;

#[derive(Facet, PartialEq, Eq, Debug, Clone)]
struct Outer {
    name: String,
    inner: Inner,
}

#[derive(Facet, PartialEq, Eq, Debug, Clone)]
struct Inner {
    x: i32,
    b: i32,
}

fn main() {
    launch_with_title(app, "Rust Struct Editor");
}

fn app() -> Element {
    let mut outer = use_signal(|| Outer {
        name: String::new(),
        inner: Inner { x: 0, b: 0 },
    });

    let mut outer_show = use_signal(|| Outer {
        name: String::new(),
        inner: Inner { x: 0, b: 0 },
    });

    use_init_theme(|| DARK_THEME);

    rsx!(
        Body {
            padding: "10",
            spacing: "10",
            label {
                "Outer Name:"
            }
            Input {
                value: outer().name.clone(),
                placeholder: "Enter outer name...",
                width: "fill",
                onchange: move |txt| {
                    outer.write().name = txt;
                }
            }
            label {
                "Inner X:"
            }
            Input {
                value: outer().inner.x.to_string(),
                placeholder: "Enter inner x...",
                width: "fill",
                onvalidate: |validator: InputValidator| {
                    validator.set_valid(validator.text().parse::<i32>().is_ok())
                },
                onchange: move |txt: String| {
                    if let Ok(x) = txt.parse::<i32>() {
                        outer.write().inner.x = x;
                    }
                }
            }
            label {
                "Inner B:"
            }
            Input {
                value: outer().inner.b.to_string(),
                placeholder: "Enter inner b...",
                width: "fill",
                onvalidate: |validator: InputValidator| {
                    validator.set_valid(validator.text().parse::<i32>().is_ok())
                },
                onchange: move |txt: String| {
                    if let Ok(b) = txt.parse::<i32>() {
                        outer.write().inner.b = b;
                    }
                }
            }
            rect {
                width: "fill",
                content: "flex",
                direction: "horizontal",
                spacing: "10",
                Button {
                    theme: theme_with!(ButtonTheme {
                        width: "flex(1)".into(),
                    }),
                    onpress: move |_| {
                        *outer.write() = Outer {
                            name: String::new(),
                            inner: Inner { x: 0, b: 0 },
                        };
                    },
                    label {
                        "Reset Form"
                    }
                }
                FilledButton {
                    theme: theme_with!(ButtonTheme {
                        width: "flex(1)".into(),
                    }),
                    onpress: move |_| {
                        let current = outer_show.read();
                        println!("Current struct: {}", current.pretty());
                    },
                    label {
                        "Print to Console"
                    }
                }
                FilledButton {
                    theme: theme_with!(ButtonTheme {
                        width: "flex(1)".into(),
                    }),
                    onpress: move |_| {
                        let current = outer.read();
                        let built = Wip::alloc::<Outer>()
                            .field_named("name").unwrap()
                            .put(current.name.clone()).unwrap()
                            .pop().unwrap()
                            .field_named("inner").unwrap()
                            .field_named("x").unwrap()
                            .put(current.inner.x).unwrap()
                            .pop().unwrap()
                            .field_named("b").unwrap()
                            .put(current.inner.b).unwrap()
                            .pop().unwrap()
                            .pop().unwrap()
                            .build().unwrap();
                        *outer_show.write() = built.materialize::<Outer>().expect("Could not materialilze");
                    },
                    label {
                        "Submit"
                    }
                }
            }
            rect {
                width: "fill",
                content: "flex",
                direction: "horizontal",
                spacing: "10",
                ScrollView {
                    rect {
                        label {
                            font_family: "Inter",
                            "Name: {outer_show.read().name.clone()}"
                        }
                        label {
                            font_family: "Inter",
                            "Inner.x: {outer_show.read().inner.x}"
                        }
                        label {
                            font_family: "Inter",
                            "Inner.b: {outer_show.read().inner.b}"
                        }
                    }
                }
            }

        }
    )
}
