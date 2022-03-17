use crate::techniques::{Ruleset, RulesetTemplate, TECHNIQUE_NAMES};
use custom_error::custom_error;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use rand::{SeedableRng, RngCore, Rng};

lazy_static! {
    static ref PERCENT_WEIGHT_PATTERN: Regex = Regex::new("(\\d+)%?").unwrap();

    #[allow(non_upper_case_globals)]
    pub(crate) static ref NMGRules: Ruleset = Ruleset {
        name: "NMG".to_owned(),
        SaveAndQuit: IsAllowed::DISALLOWED,
        FakeFlippers: IsAllowed::ALLOWED,
        BombJump: IsAllowed::ALLOWED,
        SilverlessGanon: IsAllowed::ALLOWED,
        ItemDash: IsAllowed::ALLOWED,
        OverworldClipping: IsAllowed::DISALLOWED,
        OverworldYBA: IsAllowed::DISALLOWED,
        SuperSpeed: IsAllowed::ALLOWED,
        OverworldEG: IsAllowed::DISALLOWED,
        Misslotting: IsAllowed::DISALLOWED,
        HeraPot: IsAllowed::UNSPECIFIED,
        HookShopping: IsAllowed::DISALLOWED,
        LayerDisparity: IsAllowed::DISALLOWED,
        OverworldSwimmyG: IsAllowed::DISALLOWED,
        UnderworldClipping: IsAllowed::DISALLOWED,
        AncillaOverload: IsAllowed::ALLOWED,
        DiverDown: IsAllowed::DISALLOWED,
        DoorJukes: IsAllowed::DISALLOWED,
        DoorStateExtension: IsAllowed::DISALLOWED,
        HammerJump: IsAllowed::ALLOWED,
        Hover: IsAllowed::ALLOWED,
        OverworldBunnyRevival: IsAllowed::UNSPECIFIED,
        OverworldMirrorGlitches: IsAllowed::DISALLOWED,
        SomariaTransitionCorruption: IsAllowed::DISALLOWED,
        UnderworldDeathHole: IsAllowed::DISALLOWED,
        UnderworldYBA: IsAllowed::DISALLOWED,
    };

    #[allow(non_upper_case_globals)]
    pub(crate) static ref RMGRules: Ruleset = Ruleset {
        name: "RMG".to_owned(),
        SaveAndQuit: IsAllowed::ALLOWED,
        FakeFlippers: IsAllowed::ALLOWED,
        BombJump: IsAllowed::ALLOWED,
        SilverlessGanon: IsAllowed::ALLOWED,
        ItemDash: IsAllowed::ALLOWED,
        OverworldClipping: IsAllowed::ALLOWED,
        OverworldYBA: IsAllowed::ALLOWED,
        SuperSpeed: IsAllowed::ALLOWED,
        OverworldEG: IsAllowed::ALLOWED,
        Misslotting: IsAllowed::DISALLOWED,
        HeraPot: IsAllowed::ALLOWED,
        HookShopping: IsAllowed::UNSPECIFIED,
        LayerDisparity: IsAllowed::DISALLOWED,
        OverworldSwimmyG: IsAllowed::UNSPECIFIED,
        UnderworldClipping: IsAllowed::ALLOWED,
        AncillaOverload: IsAllowed::ALLOWED,
        DiverDown: IsAllowed::ALLOWED,
        DoorJukes: IsAllowed::DISALLOWED,
        DoorStateExtension: IsAllowed::ALLOWED,
        HammerJump: IsAllowed::ALLOWED,
        Hover: IsAllowed::ALLOWED,
        OverworldBunnyRevival: IsAllowed::ALLOWED,
        OverworldMirrorGlitches: IsAllowed::ALLOWED,
        SomariaTransitionCorruption: IsAllowed::DISALLOWED,
        UnderworldDeathHole: IsAllowed::DISALLOWED,
        UnderworldYBA: IsAllowed::DISALLOWED,
    };

    #[allow(non_upper_case_globals)]
    pub(crate) static ref NoEGRules: Ruleset = Ruleset {
        name: "No EG".to_owned(),
        SaveAndQuit: IsAllowed::ALLOWED,
        FakeFlippers: IsAllowed::ALLOWED,
        BombJump: IsAllowed::ALLOWED,
        SilverlessGanon: IsAllowed::ALLOWED,
        ItemDash: IsAllowed::ALLOWED,
        OverworldClipping: IsAllowed::ALLOWED,
        OverworldYBA: IsAllowed::ALLOWED,
        SuperSpeed: IsAllowed::ALLOWED,
        OverworldEG: IsAllowed::ALLOWED,
        Misslotting: IsAllowed::ALLOWED,
        HeraPot: IsAllowed::ALLOWED,
        HookShopping: IsAllowed::ALLOWED,
        LayerDisparity: IsAllowed::DISALLOWED,
        OverworldSwimmyG: IsAllowed::UNSPECIFIED,
        UnderworldClipping: IsAllowed::ALLOWED,
        AncillaOverload: IsAllowed::ALLOWED,
        DiverDown: IsAllowed::ALLOWED,
        DoorJukes: IsAllowed::ALLOWED,
        DoorStateExtension: IsAllowed::ALLOWED,
        HammerJump: IsAllowed::ALLOWED,
        Hover: IsAllowed::ALLOWED,
        OverworldBunnyRevival: IsAllowed::ALLOWED,
        OverworldMirrorGlitches: IsAllowed::ALLOWED,
        SomariaTransitionCorruption: IsAllowed::ALLOWED,
        UnderworldDeathHole: IsAllowed::ALLOWED,
        UnderworldYBA: IsAllowed::ALLOWED,
    };


    #[allow(non_upper_case_globals)]
    pub(crate) static ref MGRules: Ruleset = Ruleset {
        name: "MG".to_owned(),
        SaveAndQuit: IsAllowed::ALLOWED,
        FakeFlippers: IsAllowed::ALLOWED,
        BombJump: IsAllowed::ALLOWED,
        SilverlessGanon: IsAllowed::ALLOWED,
        ItemDash: IsAllowed::ALLOWED,
        OverworldClipping: IsAllowed::ALLOWED,
        OverworldYBA: IsAllowed::ALLOWED,
        SuperSpeed: IsAllowed::ALLOWED,
        OverworldEG: IsAllowed::ALLOWED,
        Misslotting: IsAllowed::ALLOWED,
        HeraPot: IsAllowed::ALLOWED,
        HookShopping: IsAllowed::ALLOWED,
        LayerDisparity: IsAllowed::ALLOWED,
        OverworldSwimmyG: IsAllowed::ALLOWED,
        UnderworldClipping: IsAllowed::ALLOWED,
        AncillaOverload: IsAllowed::ALLOWED,
        DiverDown: IsAllowed::ALLOWED,
        DoorJukes: IsAllowed::ALLOWED,
        DoorStateExtension: IsAllowed::ALLOWED,
        HammerJump: IsAllowed::ALLOWED,
        Hover: IsAllowed::ALLOWED,
        OverworldBunnyRevival: IsAllowed::ALLOWED,
        OverworldMirrorGlitches: IsAllowed::ALLOWED,
        SomariaTransitionCorruption: IsAllowed::ALLOWED,
        UnderworldDeathHole: IsAllowed::ALLOWED,
        UnderworldYBA: IsAllowed::ALLOWED,
    };
}


#[derive(Copy, Clone, PartialEq, Debug, Serialize)]
pub(crate) enum IsAllowed {
    ALLOWED,
    DISALLOWED,
    UNSPECIFIED,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum TemplateState {
    STATIC(IsAllowed),
    CHANCE_PER_THOUSAND(u16),
    USE_DEFAULT,
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
        } else if let Some(m) = PERCENT_WEIGHT_PATTERN.captures(&*user_input) {
            match m[1].parse::<u16>() {
                Ok(w) => {
                    if (0..=1000).contains(&w) {
                        Some(Self::CHANCE_PER_THOUSAND(w))
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
            None => Err(TemplateStateParseError {
                err: Self::ERR.to_string(),
            }),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub(crate) struct InputWeights {
    pub(crate) name: String,
    pub(crate) defaults: String,
    pub(crate) weights: HashMap<String, String>,
}

#[derive(Debug)]
pub(crate) struct MungedInputWeights {
    pub(crate) name: String,
    pub(crate) defaults: &'static Ruleset,
    pub(crate) weights: HashMap<String, TemplateState>,
}

custom_error! {
   #[derive(PartialEq)]
   pub(crate) UserInputError { err: String } = "{err}"
}


/*
Take user inputs (from yaml, probably). For each key that's a valid technique name,
parse it as a TemplateState. If any of these fail, return an error. Otherwise, return both all the
parsed TemplateStates and a list of unexpected keys (if any).
 */
fn parse_weights(
    mut input_weights: HashMap<String, String>,
) -> Result<(HashMap<String, TemplateState>, Option<Vec<String>>), UserInputError> {
    let mut parsed: HashMap<String, TemplateState> = Default::default();
    for k in TECHNIQUE_NAMES {
        if let Some(ts_input) = input_weights.remove(k) {
            match TemplateState::from_user_input(ts_input) {
                Ok(ts) => {
                    parsed.insert(k.to_string(), ts);
                }
                Err(e) => {
                    return Err(UserInputError {
                        err: format!("Error parsing user input for {}: {}", k, e),
                    });
                }
            }
        }
    }
    let keys: Option<Vec<String>>;
    if !input_weights.is_empty() {
        keys =
        Some(
            input_weights
                .keys()
                .map(|f| f.to_string())
                .collect::<Vec<String>>()

        );
    } else {
        keys = None;
    }

    Ok((parsed, keys))
}

fn find_default(defaults_name: String) -> Result<&'static Ruleset, UserInputError> {
    match defaults_name.as_str() {
        "NMGRules" => Ok(&NMGRules),
        _ => Err(UserInputError {
            err: format!("Unknown value for `defaults` field: {}", defaults_name),
        }),
    }
}

fn munge_user_input(user_input: InputWeights) -> Result<MungedInputWeights, UserInputError> {
    let (parsed, unknown_keys) = parse_weights(user_input.weights)?;
    if let Some(ks) = unknown_keys {
        println!("Unknown user input keys: {}", ks.join(", "));
    }
    let defaults = find_default(user_input.defaults)?;
    Ok(MungedInputWeights {
        name: user_input.name,
        defaults,
        weights: parsed,
    })
}

pub(crate) fn parse_user_input(yaml: String) -> Result<MungedInputWeights, UserInputError> {
    match serde_yaml::from_str::<InputWeights>(&yaml) {
        Ok(iw) => {
            munge_user_input(iw)
        }
        Err(e) => Err(UserInputError {
            err: format!("Invalid input yaml: {}", e),
        }),
    }

}

mod test {
    use super::TemplateState;
    use crate::rules::{find_default, IsAllowed, NMGRules, UserInputError, parse_weights, InputWeights, munge_user_input, MungedInputWeights, parse_user_input};
    use std::collections::HashMap;
    use crate::techniques::{RulesetTemplate, Ruleset};
    use rand::rngs::SmallRng;
    use rand::SeedableRng;

    #[test]
    fn test_parsing_template_state() {
        assert_eq!(
            None,
            TemplateState::_maybe_from_user_input("asdf".to_string())
        );
        assert_eq!(
            None,
            TemplateState::_maybe_from_user_input("123".to_string())
        );
        assert_eq!(
            None,
            TemplateState::_maybe_from_user_input("-123".to_string())
        );
        assert_eq!(
            None,
            TemplateState::_maybe_from_user_input("x123x".to_string())
        );

        assert_eq!(
            Some(TemplateState::STATIC(IsAllowed::ALLOWED)),
            TemplateState::_maybe_from_user_input("true".to_string())
        );
        assert_eq!(
            Some(TemplateState::STATIC(IsAllowed::ALLOWED)),
            TemplateState::_maybe_from_user_input("TRUE".to_string())
        );
        assert_eq!(
            Some(TemplateState::STATIC(IsAllowed::ALLOWED)),
            TemplateState::_maybe_from_user_input("tRuE".to_string())
        );

        assert_eq!(
            Some(TemplateState::STATIC(IsAllowed::DISALLOWED)),
            TemplateState::_maybe_from_user_input("FALSE".to_string())
        );
        assert_eq!(
            Some(TemplateState::STATIC(IsAllowed::DISALLOWED)),
            TemplateState::_maybe_from_user_input("false".to_string())
        );
        assert_eq!(
            Some(TemplateState::STATIC(IsAllowed::DISALLOWED)),
            TemplateState::_maybe_from_user_input("fALse".to_string())
        );

        assert_eq!(
            Some(TemplateState::CHANCE_PER_THOUSAND(69)),
            TemplateState::_maybe_from_user_input("69".to_lowercase())
        );
        assert_eq!(
            Some(TemplateState::CHANCE_PER_THOUSAND(69)),
            TemplateState::_maybe_from_user_input("69%".to_lowercase())
        );
    }

    #[test]
    fn test_find_defaults() {
        let failed = find_default("blargh".to_string());
        assert!(failed.is_err());

        assert!(find_default("NMGRules".to_string()).is_ok());
        // reference comparison bad
    }

    #[test]
    fn test_parse_weights_bad() {
        let mut ui: HashMap<String, String> = Default::default();
        ui.insert("FakeFlippers".to_string(), "blahhhh".to_string());
        assert_eq!(
        UserInputError { err: r#"Error parsing user input for FakeFlippers: Expected "true", "false", or a number."#.to_string() },
        parse_weights(ui).unwrap_err()
        )
    }

    #[test]
    fn test_parse_weights_clean() {
        let mut ui: HashMap<String, String> = Default::default();
        ui.insert("FakeFlippers".to_string(), "69%".to_string());
        let (mut parsed, extras) = parse_weights(ui).unwrap();
        assert!(extras.is_none());
        assert_eq!(
            TemplateState::CHANCE_PER_THOUSAND(69),
            parsed.remove("FakeFlippers").unwrap()
        );
    }

    #[test]
    fn test_parse_weights_extras() {
        let mut ui: HashMap<String, String> = Default::default();
        ui.insert("FakeFlippers".to_string(), "69%".to_string());
        ui.insert("unused".to_string(), "who cares".to_string());
        let (mut parsed, extras) = parse_weights(ui).unwrap();
        assert_eq!(vec!["unused".to_string()], extras.unwrap());
        assert_eq!(
            TemplateState::CHANCE_PER_THOUSAND(69),
            parsed.remove("FakeFlippers").unwrap()
        );
    }

    #[test]
    fn test_munge() {
        let mut iw = InputWeights {
            name: "a_name".to_string(),
            defaults: "NMGRules".to_string(),
            weights: Default::default()
        };

        iw.weights.insert("FakeFlippers".to_string(), "true".to_string());
        let munged = munge_user_input(iw).unwrap();
        assert_eq!(
            "a_name".to_string(),
            munged.name,
        );
        // meh
    }

    #[test]
    fn test_user_inputs_from_yaml() {
        let some_yaml = r#"
name: hello
defaults: NMGRules
weights:
    OverworldClipping: 40%
    unused: unused
"#.to_string();
        let parsed = parse_user_input(some_yaml);
        assert!(parsed.is_ok(), "Failed to parse yaml: {}", parsed.unwrap_err());
        let p = parsed.unwrap();
        assert_eq!("hello", p.name);
        assert_eq!(TemplateState::CHANCE_PER_THOUSAND(40), *p.weights.get("OverworldClipping").unwrap());

    }

    #[test]
    fn test_template_from_weights() {
        let mut weights: HashMap<String, TemplateState> = Default::default();
        weights.insert("OverworldClipping".to_string(), TemplateState::STATIC(IsAllowed::ALLOWED));
        let rt = RulesetTemplate::from_template_states(&weights);
        assert_eq!(TemplateState::STATIC(IsAllowed::ALLOWED), rt.OverworldClipping);
        assert_eq!(TemplateState::USE_DEFAULT, rt.FakeFlippers);
    }

    // N.B. These will probably break when the order of techniques changes; this is mostly just
    // to prove to myself that seeding RNG works and that I can spell it right, etc.
    #[test]
    fn test_apply_rule_with_rng() {
        let rt = RulesetTemplate {
            FakeFlippers: TemplateState::CHANCE_PER_THOUSAND(50),
            OverworldClipping: TemplateState::USE_DEFAULT,
        };

        let mut rng = SmallRng::seed_from_u64(1);
        assert_eq!(IsAllowed::DISALLOWED, rt.apply_rule_with_rng(&IsAllowed::DISALLOWED, &TemplateState::CHANCE_PER_THOUSAND(40), &mut rng));


        let mut rng2 = SmallRng::seed_from_u64(2);
        assert_eq!(IsAllowed::DISALLOWED, rt.apply_rule_with_rng(&IsAllowed::DISALLOWED, &TemplateState::CHANCE_PER_THOUSAND(40), &mut rng2));

        let mut rng3 = SmallRng::seed_from_u64(3);
        assert_eq!(IsAllowed::DISALLOWED, rt.apply_rule_with_rng(&IsAllowed::DISALLOWED, &TemplateState::CHANCE_PER_THOUSAND(40), &mut rng3));


        let mut rng4 = SmallRng::seed_from_u64(4);
        assert_eq!(IsAllowed::DISALLOWED, rt.apply_rule_with_rng(&IsAllowed::DISALLOWED, &TemplateState::CHANCE_PER_THOUSAND(40), &mut rng4));

        let mut rng5 = SmallRng::seed_from_u64(500);
        assert_eq!(IsAllowed::ALLOWED, rt.apply_rule_with_rng(&IsAllowed::DISALLOWED, &TemplateState::CHANCE_PER_THOUSAND(40), &mut rng5));
    }


    #[test]
    fn test_apply_with_rng() {
        let rt = RulesetTemplate {
            FakeFlippers: TemplateState::CHANCE_PER_THOUSAND(50),
            OverworldClipping: TemplateState::USE_DEFAULT,
        };

        let defaults = Ruleset {
            FakeFlippers: IsAllowed::ALLOWED,
            OverworldClipping: IsAllowed::DISALLOWED,
        };

        let mut rng = SmallRng::seed_from_u64(1);
        assert_eq!(IsAllowed::DISALLOWED, rt.apply_with_rng(&defaults, &mut rng).FakeFlippers);
        let mut rng2 = SmallRng::seed_from_u64(500);
        assert_eq!(IsAllowed::ALLOWED, rt.apply_with_rng(&defaults, &mut rng).FakeFlippers);
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
