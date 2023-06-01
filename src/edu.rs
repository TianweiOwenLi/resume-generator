//! The educations section of resume.

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct EduProp {
  pub school: String,
  pub location: String,
  pub time: String,
  pub degree: String, 
  pub gpa: Option<String>, 
  pub courseworks: Option<Vec<String>>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct EduVecProp {
  pub content: Vec<EduProp>
}

#[function_component]
pub fn Edu(data: &EduProp) -> Html {

  let gpa_opt = match &data.gpa {
    Some(x) => format!(" (GPA: {})", x),
    None => "".to_string()
  };

  let degree_html = html! {
    <div class="entry">
      <h5>{"Degree"}</h5>
      <p>{format!("{}{}", &data.degree, gpa_opt)}</p>
    </div>
  };

  let course_html = match &data.courseworks {
    Some(v) => html! {
      <div class="entry">
        <h5>{"Relevant Coursework"}</h5>
        <ul>
          { v.iter().map(|item|
            html!{<li>{ item }</li>}
          ).collect::<Html>() }
        </ul>
      </div>
    },
    None => html! {}
  };

  html! {
    <div class="section-body">
      <p class="xp-hdr"><b>{&data.school}</b></p>
      <div class="modifier">
        <p>{&data.location}</p>
        <p>{&data.time}</p>
      </div>
      <div class="lined-body">
        { degree_html }
        { course_html }
      </div>
    </div>
  }
}

#[function_component]
pub fn EduVec(data: &EduVecProp) -> Html {
  html! {
    <>
      <h4>{"Educations"}</h4>
      <div class="section-body">
        {
          data.content.iter().map(|item|
            html!{
              <Edu school={item.school.clone()}
                location={item.location.clone()}
                time={item.time.clone()}
                degree={item.degree.clone()}
                gpa={item.gpa.clone()}
                courseworks={item.courseworks.clone()}/>
            }
          ).collect::<Html>()
        }
      </div>
    </>
  }
}
