use leptos::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

// Often, you want to pass some kind of child view to another
// component. There are two basic patterns for doing this:
// - "render props": creating a component prop that takes a function
//   that creates a view
// - the `children` prop: a special property that contains content
//   passed as the children of a component in your view, not as a
//   property

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (items, _set_items) = create_signal(cx, vec![0, 1, 2]);
    let render_prop = move || {
        // items.with(...) reacts to the value without cloning
        // by applying a function. Here, we pass the `len` method
        // on a `Vec<_>` directly
        let len = move || items.with(Vec::len);
        view! { cx,
            <p>"Length: " {len}</p>
        }
    };

    view! { cx,
        // This component just displays the two kinds of children,
        // embedding them in some other markup
        <TakesChildren
            // for component props, you can shorthand
            // `render_prop=render_prop` => `render_prop`
            // (this doesn't work for HTML element attributes)
            render_prop
        >
            // these look just like the children of an HTML element
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </TakesChildren>
        <hr/>
        // This component actually iterates over and wraps the children
        <WrapsChildren>
            <p>"Here's a child."</p>
            <p>"Here's another child."</p>
        </WrapsChildren>
    }
}

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
    cx: Scope,
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` takes the `Children` type this is an alias for `Box<dyn FnOnce(Scope) -> Fragment>`
    /// If you need a Fn or FnMut, ChildrenFn and ChildrenMut aliases are also provided
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { cx,
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children(cx)}
    }
}

/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
pub fn WrapsChildren(cx: Scope, children: Children) -> impl IntoView {
    // children(cx) returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children(cx)
        .nodes
        .into_iter()
        .map(|child| view! { cx, <li>{child}</li> })
        .collect_view(cx);

    // wrap our wrapped children in a UL
    view! { cx,
        <h1><code>"<WrapsChildren/>"</code></h1>
        <ul>{children}</ul>
    }
}
