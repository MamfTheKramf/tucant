use std::ops::Deref;

use tucant_types::Tucan;
use yew::{Html, HtmlResult, function_component, html};

use crate::{RcTucanType, common::use_data_loader};

#[function_component(MyDocuments)]
pub fn my_documents<TucanType: Tucan + 'static>() -> Html {
    let handler = async |tucan: RcTucanType<TucanType>, current_session, revalidation_strategy, additional| tucan.0.my_documents(&current_session, revalidation_strategy).await;

    use_data_loader(handler, (), 14 * 24 * 60 * 60, 60 * 60, |documents, reload| {
        ::yew::html! {
            <div>
                <h1>
                    { "Meine Dokumente" }
                    { " " }
                    <button onclick={reload} type="button" class="btn btn-light">
                        // https://github.com/twbs/icons
                        // The MIT License (MIT)
                        // Copyright (c) 2019-2024 The Bootstrap Authors

                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-arrow-clockwise" viewBox="0 0 16 16">
                            <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2z" />
                            <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466" />
                        </svg>
                    </button>
                </h1>
                <table class="table">
                    <thead>
                        <tr>
                            <th scope="col">
                                { "Name" }
                            </th>
                            <th scope="col">
                                { "Datum" }
                            </th>
                            <th scope="col">
                                { "URL" }
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            documents
                                .documents
                                .iter()
                                .map(|document| {
                                    ::yew::html! {
                                        <tr>
                                            <th scope="row">
                                                { &document.name }
                                            </th>
                                            <td>
                                                { &document.date }
                                                { " " }
                                                { &document.time }
                                            </td>
                                            <td>
                                                <a href={format!("https://www.tucan.tu-darmstadt.de{}", document.url)}>
                                                    { "Download" }
                                                </a>
                                            </td>
                                        </tr>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </tbody>
                </table>
            </div>
        }
    })
}
