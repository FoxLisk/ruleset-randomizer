
use crate::techniques::{Ruleset, RulesetTemplate};
use rand::Rng;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use custom_error::custom_error;
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseIntError;

lazy_static! {
    static ref PERCENT_WEIGHT_PATTERN: Regex = Regex::new("(\\d+)%?").unwrap();
}

#[allow(non_upper_case_globals)]
static NMGRules: Ruleset = Ruleset {
    FakeFlippers: IsAllowed::ALLOWED,
    OverworldClipping: IsAllowed::DISALLOWED,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum IsAllowed {
    ALLOWED,
    DISALLOWED,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum TemplateState {
    STATIC(IsAllowed),
    PERCENT(u16),
}


custom_error! {
    TemplateStateParseError { err: String } = "{err}"

}

impl TemplateState {
    const ERR: &'static str = r#"Expected "true", "false", or a number."#;

    fn _maybe_from_user_input(user_input: String) -> Option<Self> {
        if user_input.to_lowercase() == "true" {
            Some(Self::STATIC(IsAllowed::ALLOWED))
        } else if user_input.to_lowercase() == "false" {
            Some(Self::STATIC(IsAllowed::DISALLOWED))
        } else if let Some(m) =  PERCENT_WEIGHT_PATTERN.captures(&*user_input) {
            match m[1].parse::<u16>() {
                Ok(w) => {
                    if (0..=100).contains(&w) {
                        Some(Self::PERCENT(w))
                    } else {
                        None
                    }
                }
                Err(e) => {
                    println!("Error parsing {} as a u16: {}", user_input, e);
                    None
                }
            }
        } else {
            None
        }

    }

    fn from_user_input(user_input: String) -> Result<Self, TemplateStateParseError> {
        match Self::_maybe_from_user_input(user_input) {
            Some(s) => Ok(s),
            None =>
                Err(TemplateStateParseError { err: Self::ERR.to_string() })
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct InputWeights {
    pub(crate) name: String,
    pub(crate) defaults: String,
    pub(crate) weights: HashMap<String, String>,
}

mod test {
    use super::TemplateState;
    use crate::rules::IsAllowed;

    #[test]
    fn test_parsing_template_state() {
        assert_eq!(None, TemplateState::_maybe_from_user_input("asdf".to_string()));
        assert_eq!(None, TemplateState::_maybe_from_user_input("123".to_string()));
        assert_eq!(None, TemplateState::_maybe_from_user_input("-123".to_string()));
        assert_eq!(None, TemplateState::_maybe_from_user_input("x123x".to_string()));

        assert_eq!(Some(TemplateState::STATIC(IsAllowed::ALLOWED)), TemplateState::_maybe_from_user_input("true".to_string()));
        assert_eq!(Some(TemplateState::STATIC(IsAllowed::ALLOWED)), TemplateState::_maybe_from_user_input("TRUE".to_string()));
        assert_eq!(Some(TemplateState::STATIC(IsAllowed::ALLOWED)), TemplateState::_maybe_from_user_input("tRuE".to_string()));

        assert_eq!(Some(TemplateState::STATIC(IsAllowed::DISALLOWED)), TemplateState::_maybe_from_user_input("FALSE".to_string()));
        assert_eq!(Some(TemplateState::STATIC(IsAllowed::DISALLOWED)), TemplateState::_maybe_from_user_input("false".to_string()));
        assert_eq!(Some(TemplateState::STATIC(IsAllowed::DISALLOWED)), TemplateState::_maybe_from_user_input("fALse".to_string()));

        assert_eq!(Some(TemplateState::PERCENT(69)), TemplateState::_maybe_from_user_input("69".to_lowercase()));
        assert_eq!(Some(TemplateState::PERCENT(69)), TemplateState::_maybe_from_user_input("69%".to_lowercase()));

    }
}

/*
we want something like:

then we want to be able to dump that out into a webpage and have it perhaps group things nicely etc.
a button to say "show diff from {NMG, standard rando, HMG, No EG}" or something would be dope


probably we want something like "what i'm diverging from"

so

    name: not_quite_nmg
    defaults: nmg
    weights:
        FakeFlippers: 95
        STC: 5
        MirrorSuperbunny: ALLOWED

would have, say, OWEG banned, but if you changed defaults to rmg it would have OWEG legal.
but both would have fake flippers 95% chance, etc

Perhaps the way to manage the groups thing is to have something like:

   name: whatever


 */