//! The resume header

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HdrProp {
  pub name: String, 
  pub website: Option<String>,
  pub github: Option<String>,
  pub phone: String,
  pub email: String,
}

#[function_component]
pub fn Hdr(data: &HdrProp) -> Html {
  let placeholder = html! {<p>{" "}</p>};

  let website_html = match &data.website {
    Some(link) => html! {
      <p>{format!("🔍 {}", link)}</p>
    },
    None => placeholder.clone()
  };

  let github_html = match &data.github {
    Some(link) => html! {
      <p>
        <img class="inline-icon" src="/imgs/github-mark.png"/> 
        { link }
      </p>
    },
    None => placeholder
  };

  let phone_html = html! { <p>{format!("📞 {}", data.phone)}</p> };

  let email_html = html! {
    <p>
      <img class="inline-icon" src="../imgs/email.jpg"/> 
      { &data.email }
    </p>
  };

  html! {
    <div class="hdr flexrow">
      <h3>{&data.name}</h3>
      <div class="contacts flexcol">
        { website_html }
        { phone_html }
      </div>
      <div class="contacts flexcol">
        { github_html }
        { email_html }
      </div>
    </div>
  }
}