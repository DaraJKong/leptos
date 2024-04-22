mod api;
use crate::api::*;
use leptos::{
    component,
    prelude::*,
    reactive_graph::{
        computed::AsyncDerived,
        effect::Effect,
        owner::{provide_context, use_context, Owner},
        signal::ArcRwSignal,
    },
    suspend,
    tachys::either::Either,
    view, IntoView, Params, Suspense, Transition,
};
use log::{debug, info};
use routing::{
    components::{ParentRoute, Route, Router, Routes},
    hooks::{use_location, use_params},
    Outlet,
};
use routing::{
    location::{BrowserUrl, Location},
    params::Params,
    MatchNestedRoutes, NestedRoute, ParamSegment, StaticSegment,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct ExampleContext(i32);

#[component]
pub fn RouterExample() -> impl IntoView {
    info!("rendering <RouterExample/>");

    // contexts are passed down through the route tree
    provide_context(ExampleContext(0));

    view! {
        <Router>
            <nav>
                // ordinary <a> elements can be used for client-side navigation
                // using <A> has two effects:
                // 1) ensuring that relative routing works properly for nested routes
                // 2) setting the `aria-current` attribute on the current link,
                // for a11y and styling purposes

                <a href="/contacts">"Contacts"</a>
                <a href="/about">"About"</a>
                <a href="/settings">"Settings"</a>
                <a href="/redirect-home">"Redirect to Home"</a>
            </nav>
            <main>
                <Routes fallback=|| "This page could not be found.">
                    <ContactRoutes/>
                    <Route path=StaticSegment("settings") view=Settings/>
                    <Route path=StaticSegment("about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}

// You can define other routes in their own component.
// Routes implement the MatchNestedRoutes
#[component]
pub fn ContactRoutes() -> impl MatchNestedRoutes<Dom> + Clone {
    view! {
        <ParentRoute path=StaticSegment("contacts") view=ContactList>
            <Route path=StaticSegment("") view=|| "Select a contact."/>
            <Route path=ParamSegment("id") view=Contact/>
        </ParentRoute>
    }
}

#[component]
pub fn ContactList() -> impl IntoView {
    info!("rendering <ContactList/>");

    // contexts are passed down through the route tree
    provide_context(ExampleContext(42));

    Owner::on_cleanup(|| {
        info!("cleaning up <ContactList/>");
    });

    let location = use_location();
    let contacts =
        AsyncDerived::new(move || get_contacts(location.search.get()));
    let contacts = suspend!(
            // this data doesn't change frequently so we can use .map().collect() instead of a keyed <For/>
            contacts.await
                .into_iter()
                .map(|contact| {
                    // TODO <A>
                    view! {
                        <li><a href=contact.id.to_string()><span>{contact.first_name} " " {contact.last_name}</span></a></li>
                    }
                })
                .collect::<Vec<_>>()
    );
    view! {
        <div class="contact-list">
            <h1>"Contacts"</h1>
            <Suspense fallback=move || view! { <p>"Loading contacts..."</p> }>
                <ul>{contacts}</ul>
            </Suspense>
            <Outlet/>
        </div>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ContactParams {
    // Params isn't implemented for usize, only Option<usize>
    id: Option<usize>,
}

#[component]
pub fn Contact() -> impl IntoView {
    info!("rendering <Contact/>");

    info!(
        "ExampleContext should be Some(42). It is {:?}",
        use_context::<ExampleContext>()
    );

    Owner::on_cleanup(|| {
        info!("cleaning up <Contact/>");
    });

    let params = use_params::<ContactParams>();

    let contact = AsyncDerived::new(move || {
        get_contact(
            params
                .get()
                .map(|params| params.id.unwrap_or_default())
                .ok(),
        )
    });

    let contact_display = suspend!(match contact.await {
        None =>
            Either::Left(view! { <p>"No contact with this ID was found."</p> }),
        Some(contact) => Either::Right(view! {
            <section class="card">
                <h1>{contact.first_name} " " {contact.last_name}</h1>
                <p>{contact.address_1} <br/> {contact.address_2}</p>
            </section>
        }),
    });

    view! {
        <div class="contact">
            <Transition fallback=move || {
                view! { <p>"Loading..."</p> }
            }>{contact_display}</Transition>
        </div>
    }
}

#[component]
pub fn About() -> impl IntoView {
    info!("rendering <About/>");

    Owner::on_cleanup(|| {
        info!("cleaning up <About/>");
    });

    info!(
        "ExampleContext should be Some(0). It is {:?}",
        use_context::<ExampleContext>()
    );

    // use_navigate allows you to navigate programmatically by calling a function
    // TODO
    // let navigate = use_navigate();

    view! {
        <h1>"About"</h1>
        <p>
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
        </p>
    }
}

#[component]
pub fn Settings() -> impl IntoView {
    info!("rendering <Settings/>");

    Owner::on_cleanup(|| {
        info!("cleaning up <Settings/>");
    });

    view! {
        <h1>"Settings"</h1>
        <form>
            <fieldset>
                <legend>"Name"</legend>
                <input type="text" name="first_name" placeholder="First"/>
                <input type="text" name="last_name" placeholder="Last"/>
            </fieldset>
            <pre>"This page is just a placeholder."</pre>
        </form>
    }
}
