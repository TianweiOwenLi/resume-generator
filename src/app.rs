use yew::prelude::*;

use crate::skill::*;
use crate::hdr::*;

#[function_component(App)]
pub fn app() -> Html {
  let strv = |x: Vec<&str>| x.into_iter().map(|x| x.to_string()).collect();

  let pl = SkillsetData {
    title: "Programming Languages".to_string(),
    content: strv(vec!["Rust", "Dart", "C++", "Prolog", "Python", 
      "Standard ML", "WebAssembly"]),
  };

  let frame = SkillsetData {
    title: "Frameworks".to_string(),
    content: strv(vec!["Yew", "Flutter"]),
  };

  let lib = SkillsetData {
    title: "Libraries and Packages".to_string(),
    content: strv(vec!["Eigen", "NumPy", "CVXPy", "Selenium"])
  };

  let sw = SkillsetData {
    title: "Softwares".to_string(),
    content: strv(vec!["YACC", "Git", "LaTeX", "LLVM", "Vim", "Docker"])
  };

  let sk = SkillsData {
    content: vec![pl, frame, lib, sw]
  };

  let hdr = HdrData {
    name: "OWEN LI".to_string(),
    website: Some("owen-li.com".to_string()),
    github: Some("github.com/TianweiOwenLi".to_string()),
    phone: "412-758-3468".to_string(),
    email: "tianwei2@andrew.cmu.edu".to_string(),
  };

  html! {
    <div class="letter">
      <div class="content" style={ format!("--margin: {}in", 0.6) }>
        <Hdr 
          name={"OWEN LI"} 
          website={ hdr.website }
          github={ hdr.github }
          phone={ hdr.phone }
          email={ hdr.email }/>
        <div class="double-line"></div>
        <div class="columns-body">
          <div class="minor-col">
            <Skills content={sk.content}/>

            <h4>{"Educations"}</h4>
            <div class="section-body">
            
            </div>
          </div>
          <div class="vert-space"></div>
          <div class="major-col">
          </div>
        </div>
      </div>
    </div>
  }
}
