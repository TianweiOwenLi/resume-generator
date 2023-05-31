//! The skills section of resume.

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SkillsetData {
  pub title: String,
  pub content: Vec<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SkillsData {
  pub content: Vec<SkillsetData>
}

#[function_component(Skillset)]
pub fn skillset(data: &SkillsetData) -> Html {
  html! {
    <div class="named-ul">
      <h5 class="list-title">{ data.title.clone() }</h5>
      <ul class="double-bullets">
        { data.content.iter().map(|item|
          html!{<li>{ item }</li>}
        ).collect::<Html>() }
      </ul>
    </div>
  }
}

#[function_component(Skills)]
pub fn skills(data: &SkillsData) -> Html {
  html! {
    <>
      <h4>{"Skills"}</h4>
      <div class="section-body">
        {
          data.content.iter().map(|item|
            html!{
              <Skillset title={item.title.clone()} 
                content={item.content.clone()}/>
            }
          ).collect::<Html>()
        }
      </div>
    </>
  }
}