//! The skills section of resume.

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SkillsProp {
  pub title: String,
  pub content: Vec<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct SkillsVecProp {
  pub content: Vec<SkillsProp>
}

#[function_component]
pub fn Skills(data: &SkillsProp) -> Html {
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

#[function_component]
pub fn SkillsVec(data: &SkillsVecProp) -> Html {
  html! {
    <>
      <h4>{"Skills"}</h4>
      <div class="section-body">
        {
          data.content.iter().map(|item|
            html!{
              <Skills title={item.title.clone()} 
                content={item.content.clone()}/>
            }
          ).collect::<Html>()
        }
      </div>
    </>
  }
}