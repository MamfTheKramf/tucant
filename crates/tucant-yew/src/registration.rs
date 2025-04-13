use std::ops::Deref as _;

use tucant_types::{
    Tucan,
    moduledetails::ModuleDetailsRequest,
    registration::{AnmeldungRequest, AnmeldungResponse, RegistrationState},
};
use yew::{Html, HtmlResult, Properties, classes, function_component, html};
use yew_router::{hooks::use_navigator, prelude::Link};

use crate::{RcTucanType, Route, common::use_data_loader};

#[derive(Properties, PartialEq)]
pub struct AnmeldungRequestProps {
    pub registration: AnmeldungRequest,
}

#[function_component(Registration)]
pub fn registration<TucanType: Tucan + 'static>(AnmeldungRequestProps { registration }: &AnmeldungRequestProps) -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.anmeldung(current_session, revalidation_strategy, additional).await;

    let navigator = use_navigator().unwrap();

    use_data_loader(handler, registration.to_owned(), 28 * 24 * 60 * 60, 24 * 60 * 60, |data, reload| {
        if data.submenus.len() == 1 && data.additional_information.is_empty() && data.entries.is_empty() {
            navigator.replace(&Route::Registration { registration: data.submenus[0].1.clone() });
            return ::yew::html! {
                <></>
            };
        }
        ::yew::html! {
            <div class="container pb-2">
                <h2 class="text-center">
                    { "Registration " }
                    <button onclick={reload} type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h2>
                <nav style="min-height: 5.5rem" aria-label="breadcrumb">
                    <ol class="breadcrumb">
                        {
                            data.path
                                .iter()
                                .map(|entry| {
                                    yew::html!{<li class="breadcrumb-item"><Link<Route> to={Route::Registration { registration: entry.1.clone()}}>{entry.0.clone()}</Link<Route>></li>}
                                })
                                .collect::<Html>()
                        }
                    </ol>
                </nav>
                // TODO FIXME this is dangerous
                { Html::from_html_unchecked(data.additional_information.join("\n").into()) }

                <h2 class="text-center">
                    { "Submenus" }
                </h2>
                <ul class="list-group">
                    {
                        data.submenus
                            .iter()
                            .map(|entry| {
                                yew::html!{<Link<Route> to={Route::Registration { registration: entry.1.clone()}} classes="list-group-item list-group-item-action">{ format!("{}", entry.0) }</Link<Route>>}
                            })
                            .collect::<Html>()
                    }
                </ul>
                {
                    if !data.entries.is_empty() {
                        yew::html!{
                            <>
                                <h2 class="text-center">
                                    { "Modules and courses" }
                                </h2>
                                <div class="container border">
                                    {
                                        data.entries
                                            .iter()
                                            .map(|entry| {
                                                let module = entry.module.as_ref();
                                                yew::html!{
                                                    <>
                                                        <div class="row bg-dark bg-gradient text-white p-1 py-3">
                                                            <div class="col-9">
                                                                <h5 class="mb-1"><Link<Route> classes={classes!("text-white")} to={Route::ModuleDetails { module: module.unwrap().url.clone() }}>{ format!("Modul {} {}", module.map(|module| module.id.clone()).unwrap_or_default(), module.map(|module| module.name.clone()).unwrap_or_default())}</Link<Route>></h5>
                                                                <h6 class="mb-1">{ format!("{}", module.map(|module| module.lecturer.clone().unwrap_or_default()).unwrap_or_default()) }</h6>
                                                            </div>
                                                            <div class="col-2 d-flex flex-column align-items-start justify-content-center">
                                                                if let Some(module) = module {
                                                                    if let Some(date) = &module.date {
                                                                        <small>{ format!("Anmeldung bis {}", date) }</small>
                                                                    }
                                                                }
                                                                if let Some(module) = module {
                                                                    if let Some(limit_and_size) = &module.limit_and_size {
                                                                        <small>{ ("Teilnehmerlimit ".to_owned() + limit_and_size) }</small>
                                                                    }
                                                                }
                                                            </div>
                                                            <div class="col-1 d-flex align-items-center justify-content-end">
                                                                {
                                                                    module.map(|module| {
                                                                        match &module.registration_state {
                                                                            RegistrationState::Unknown => yew::html! { },
                                                                            RegistrationState::Registered { unregister_link } => yew::html! { <a class="btn btn-danger btn-sm mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",unregister_link.clone())}>{"Abmelden"}</a> },
                                                                            RegistrationState::NotRegistered { register_link } => yew::html! { <a class="btn btn-outline-success btn-sm mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone())}>{"Anmelden"}</a> },
                                                                        }
                                                                    })
                                                                }
                                                            </div>
                                                        </div>
                                                        {
                                                            for entry.courses.iter().map(|course|
                                                                {
                                                                yew::html! {
                                                                    <div class="row p-1 ps-3 border-bottom">
                                                                        <div class="col-9">
                                                                            <h5 class="mb-1"><Link<Route> classes={classes!("text-dark")} to={Route::CourseDetails { course: course.1.url.clone() }}>{ format!("Kurs {} {}", course.1.id, course.1.name) }</Link<Route>></h5>
                                                                            <h6 class="mb-1">{ format!("{}", course.1.lecturers.clone().unwrap_or_default()) }</h6>
                                                                            <h6 class="mb-1">{ format!("{}", course.1.begin_and_end.clone().unwrap_or_default()) }</h6>
                                                                        </div>
                                                                        <div class="col-2 d-flex flex-column align-items-start justify-content-center">
                                                                            if let Some(registration_until) = &course.1.registration_until {
                                                                                <small class="text-body-secondary">{ format!("Anmeldung bis {}", registration_until) }</small>
                                                                            }
                                                                            if let Some(limit_and_size) = &course.1.limit_and_size {
                                                                                <small class="text-body-secondary">{ ("Teilnehmerlimit ".to_owned() + limit_and_size) }</small>
                                                                            }
                                                                        </div>
                                                                        <div class="col-1 d-flex align-items-center justify-content-end">
                                                                            {
                                                                                match &course.1.registration_button_link {
                                                                                    RegistrationState::Unknown => yew::html! { },
                                                                                    RegistrationState::Registered { unregister_link } => yew::html! { <a class="btn btn-danger btn-sm mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",unregister_link.clone())}>{"Abmelden"}</a> },
                                                                                    RegistrationState::NotRegistered { register_link } => yew::html! { <a class="btn btn-outline-success btn-sm mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",register_link.clone())}>{"Anmelden"}</a> },
                                                                                }
                                                                            }
                                                                        </div>
                                                                    </div>
                                                                }
                                                            })
                                                        }
                                                    </>
                                                }
                                            })
                                            .collect::<Html>()
                                    }
                                </div>
                            </>
                        }
                    } else {
                        yew::html!{<></>}
                    }
                }
            </div>
        }
    })
}
