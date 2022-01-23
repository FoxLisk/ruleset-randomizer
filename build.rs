use std::fs::{File, read_to_string};
use std::io::Read;
use std::io::Write;

// TODO: this should probably use a template
fn main () {
    println!("cargo:rerun-if-changed=techniques");
    let cont = read_to_string("techniques").unwrap();
    let techs: Vec<&str> = cont.lines().map(|l| l.trim()).collect();

    let mut structs_file = File::create("src/techniques.rs").unwrap();

    writeln!(&mut structs_file, "use crate::rules::{{IsAllowed, TemplateState}};").unwrap();
    writeln!(&mut structs_file, "use rand::Rng;").unwrap();
    writeln!(&mut structs_file, "").unwrap();

    writeln!(&mut structs_file, "pub(crate) struct Ruleset {{").unwrap();
    for t in &techs {
        writeln!(&mut structs_file, "    pub(crate) {}: IsAllowed,", t).unwrap();
    }
    writeln!(&mut structs_file, "}}").unwrap();


    writeln!(&mut structs_file, "pub(crate) struct RulesetTemplate {{").unwrap();
    for t in &techs {
        writeln!(&mut structs_file, "    pub(crate) {}: TemplateState,", t).unwrap();
    }
    writeln!(&mut structs_file, "}}").unwrap();

    writeln!(&mut structs_file, r#"
impl RulesetTemplate {{
    fn apply_rule(&self, default: &IsAllowed, rule: &TemplateState) -> IsAllowed {{
        match rule {{
            TemplateState::STATIC(i) => i.clone(),
            TemplateState::PERCENT(p) => {{
                let roll = {{
                    rand::thread_rng().gen_range(0u16..100)
                }};
                if *p < roll {{
                    IsAllowed::ALLOWED
                }} else {{
                    default.clone()
                }}
            }}
        }}
    }}

    pub(crate) fn apply(&self, defaults: Ruleset) -> Ruleset {{
        Ruleset {{"#).unwrap();

    for t in &techs {
        writeln!(&mut structs_file, "            {}: self.apply_rule(&defaults.{}, &self.{}),", t, t, t).unwrap();
    }
    writeln!(&mut structs_file,
             r#"
        }}
    }}
}}"#);


}