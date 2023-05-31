//! The resume header

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HdrData {
  pub name: String, 
  pub website: Option<String>,
  pub github: Option<String>,
  pub phone: String,
  pub email: String,
}

#[function_component(Hdr)]
pub fn hdr(data: &HdrData) -> Html {
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
    <div class="hdr">
      <h3>{&data.name}</h3>
      <div class="contacts">
        { website_html }
        { phone_html }
      </div>
      <div class="contacts">
        { github_html }
        { email_html }
      </div>
    </div>
  }
}