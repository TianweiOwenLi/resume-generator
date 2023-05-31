use yew::prelude::*;

use crate::skill::*;

#[derive(Properties, PartialEq, Clone)]
struct HdrData {
  name: String, 
  website: String,
  github: String,
  phone: String,
  email: String,
}

#[function_component(Hdr)]
fn hdr(data: &HdrData) -> Html {
  html! {
    <div class="hdr">
      <h3>{&data.name}</h3>
      <div class="contacts">
        <p>{format!("🔍 {}", data.website)}</p>
        <p>{format!("📞 {}", data.phone)}</p>
      </div>
      <div class="contacts">
        <p>
          <img class="inline-icon" src="/imgs/github-mark.png"/> 
          {format!("{}", data.github)}
        </p>
        <p>
          <img class="inline-icon" src="../imgs/email.jpg"/> 
          {format!("{}", data.email)}
        </p>
      </div>
    </div>
  }
}

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

  html! {
    <div class="letter">
      <div class="content" style={ format!("--margin: {}in", 0.6) }>
        <Hdr 
          name={"OWEN LI"} 
          website={"owen-li.com"} 
          github={"github.com/TianweiOwenLi"}
          phone={"412-758-3468"}
          email={"tianwei2@andrew.cmu.edu"}/>
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
