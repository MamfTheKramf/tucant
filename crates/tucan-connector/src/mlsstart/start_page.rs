use scraper::{ElementRef, Html};
use tucant_types::{
    LoginResponse,
    mlsstart::{MlsStart, Nachricht, StundenplanEintrag},
};

use crate::{
    MyClient, TucanError, authenticated_retryable_get,
    common::head::{footer, html_head, logged_in_head},
};
use html_handler::Root;

#[expect(clippy::too_many_lines)]
pub async fn after_login(client: &MyClient, login_response: &LoginResponse) -> Result<MlsStart, TucanError> {
    let id = login_response.id;
    let content = authenticated_retryable_get(client, &format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N{},-N000019,", login_response.id), &login_response.cookie_cnsc).await?;
    //let content = tokio::fs::read_to_string("input.html").await?;
    let document = Html::parse_document(&content);
    //tokio::fs::write("input.html", document.html()).await;
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
            <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
                <head>_
                    use html_head(html_handler)?;
                    <style type="text/css">
                        "XN0jaYaHLeXpiJk0Z7v_FOEBkC5jmdtJaIxqyRqEMj4"
                    </style>_
                </head>_
                <body class="currentevents">_
                    let head = logged_in_head(html_handler, login_response.id);
                    <!--"EkIRwtbzV1S0qAPx6If3Ye8Ey0JkAZsONsPW8C2Tf3Y"-->_
                    <script type="text/javascript">
                    </script>_
                    <h1>
                        _welcome_message
                    </h1>_
                    <h2>_
                    </h2>_
                    <h2>
                        _text
                    </h2>_
                    <div class="tb rw-table">_
                        <div class="tbhead">
                            "Heutige Veranstaltungen:"
                        </div>_
                        <div class="tbcontrol">_
                            <a href=_ class="img" name="schedulerLink">
                                "Stundenplan"
                            </a>_
                        </div>_
                        let stundenplan = if html_handler.peek().unwrap().value().as_element().unwrap().name() == "table" {
                            <table class="nb rw-table" summary="Studium Generale">_
                                <tbody>
                                    <tr class="tbsubhead">_
                                        <th id="Veranstaltung">
                                            "Veranstaltung"
                                        </th>_
                                        <th id="Name">
                                            "Name"
                                        </th>_
                                        <th id="von">
                                            "von"
                                        </th>_
                                        <th id="bis">
                                            "bis"
                                        </th>_
                                    </tr>_
                                    let stundenplan = while html_handler.peek().is_some() {
                                        <tr class="tbdata">_
                                            <td headers="Veranstaltung">
                                                "Kurse"
                                            </td>_
                                            <td headers="Name">_
                                                <a class="link" href=coursedetails_url name="eventLink">
                                                    course_name
                                                </a>_
                                            </td>_
                                            <td headers="von">
                                                <a class="link" href=courseprep_url>
                                                    from
                                                </a>
                                            </td>_
                                            <td headers="bis">
                                                <a class="link" href=courseprep_url2>
                                                    to
                                                </a>
                                            </td>_
                                        </tr>_
                                    } => StundenplanEintrag { course_name, coursedetails_url, courseprep_url, courseprep_url2, from, to };
                                </tbody>
                            </table>_
                        } => stundenplan else {
                            <div class="tbsubhead">
                                "\n        \tFür heute sind keine Termine angesetzt!\n\t\t"
                            </div>_
                        } => Vec::<StundenplanEintrag>::new();
                    </div>_
                    <!--"jcECXQ7Iovu3-f4IpT-2ykwKMYpSGOecnocvEf5bo3A"-->_
                    <div class="tb rw-table">_
                        <div class="tbhead">
                            "Eingegangene Nachrichten:"
                        </div>_
                        <div class="tbcontrol">_
                            <a href=_archive class="img">
                                "Archiv"
                            </a>_
                        </div>_
                        <table class="nb rw-table rw-all" summary="Eingegangene Nachrichten">_
                            <tbody>
                                <tr class="tbsubhead rw-hide">_
                                    <th id="Datum">
                                        "Datum"
                                    </th>_
                                    <th id="Uhrzeit">
                                        "Uhrzeit"
                                    </th>_
                                    <th id="Absender">
                                        "Absender"
                                    </th>_
                                    <th id="Betreff">
                                        "Betreff"
                                    </th>_
                                    <th id="Aktion">
                                        "Aktion"
                                    </th>_
                                </tr>_
                                let messages = while html_handler.peek().is_some() {
                                    <tr class="tbdata">_
                                        <td headers="Datum" class="rw rw-maildate">
                                            <a class="link" href=url1>
                                                date
                                            </a>
                                        </td>_
                                        <td headers="Uhrzeit" class="rw rw-mailtime">
                                            <a class="link" href=url2>
                                                hour
                                            </a>
                                        </td>_
                                        <td headers="Absender" class="rw rw-mailpers">
                                            <a class="link" href=url3>
                                                source
                                            </a>
                                        </td>_
                                        <td headers="Betreff" class="rw rw-mailsubject">
                                            <a class="link" href=url4>
                                                let message = html_handler.next_any_child();
                                            </a>
                                        </td>_
                                        <td headers="Aktion" class="rw rw-maildel">
                                            <a class="link" href=delete_url>
                                                "Löschen"
                                            </a>
                                        </td>_
                                    </tr>_
                                } => Nachricht {
                                    url1,
                                    url2,
                                    url3,
                                    url4,
                                    date,
                                    hour,
                                    source,
                                    message: match message.value() {
                                        scraper::Node::Text(text) => text.trim().to_owned(),
                                        scraper::Node::Element(_element) => ElementRef::wrap(message).unwrap().html(),
                                        _ => panic!(),
                                    },
                                    delete_url
                                };
                            </tbody>
                        </table>_
                    </div>_
                    <!--"fS28-ufck45gusNkaJA-yHsPF7qDLp0dqCxzpxz56og"-->_
                </div>_
            </div>_
        </div>_
    };
    let html_handler = footer(html_handler, id, 19);
    html_handler.end_document();
    Ok(MlsStart { logged_in_head: head, stundenplan: stundenplan.either_into(), messages })
}
