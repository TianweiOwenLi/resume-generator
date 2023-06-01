use yew::prelude::*;

use crate::edu::*;
use crate::skill::*;
use crate::hdr::*;

#[function_component(App)]
pub fn app() -> Html {
  let strv = |x: Vec<&str>| x.into_iter().map(|x| x.to_string()).collect();

  let pl = SkillsProp {
    title: "Programming Languages".to_string(),
    content: strv(vec!["Rust", "Dart", "C++", "Prolog", "Python", 
      "Standard ML", "WebAssembly"]),
  };

  let frame = SkillsProp {
    title: "Frameworks".to_string(),
    content: strv(vec!["Yew", "Flutter"]),
  };

  let lib = SkillsProp {
    title: "Libraries and Packages".to_string(),
    content: strv(vec!["Eigen", "NumPy", "CVXPy", "Selenium"])
  };

  let sw = SkillsProp {
    title: "Softwares".to_string(),
    content: strv(vec!["YACC", "Git", "LaTeX", "LLVM", "Vim", "Docker"])
  };

  let sk = SkillsVecProp {
    content: vec![pl, frame, lib, sw]
  };

  let hdr = HdrProp {
    name: "OWEN LI".to_string(),
    website: Some("owen-li.com".to_string()),
    github: Some("github.com/TianweiOwenLi".to_string()),
    phone: "412-758-3468".to_string(),
    email: "tianwei2@andrew.cmu.edu".to_string(),
  };

  let cmu = EduProp {
    school: "Carnegie Mellon University".to_string(),
    location: "Pittsburgh, PA".to_string(),
    time: "Sep 2020 - Present".to_string(),
    degree: "B.Sc in Computer Science, with an additional major in Math".to_string(),
    gpa: Some("3.7".to_string()),
    courseworks: Some(strv(vec!["Probabilistic Combinatorics", "Compiler Design"]))
  };

  let edus = EduVecProp {
    content: vec![cmu]
  };

  html! {
    <div class="letter">
      <div class="content" style={ format!("--margin: {}in", 0.6) }>
        <Hdr name={"OWEN LI"} website={ hdr.website } github={ hdr.github }
          phone={ hdr.phone } email={ hdr.email }/>
        <div class="double-line"></div>
        <div class="columns-body">
          <div class="minor-col">
            <SkillsVec content={sk.content}/>
            <EduVec content={edus.content}/>
          </div>
          <div class="vert-space"></div>
          <div class="major-col">
          </div>
        </div>
      </div>
    </div>
  }
}
