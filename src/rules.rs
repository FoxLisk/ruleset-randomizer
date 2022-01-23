use crate::techniques::{Ruleset, RulesetTemplate};
use rand::Rng;


#[derive(Copy, Clone)]
pub(crate) enum IsAllowed {
    ALLOWED,
    DISALLOWED,
}

#[derive(Copy, Clone)]
pub(crate) enum TemplateState {
    STATIC(IsAllowed),
    PERCENT(u16),
}

impl RulesetTemplate {


}


/*
we want something like:

let user_weights = load_yaml(...);
let ruleset = RulesetTemplate::default();
ruleset.update(user_weights);
now we have a ruleset template that has things like
  FakeFlippers: 95%
  STC: 4%

and then we can run something like

let ruleset = RulesetTemplate.instantiate(seed)

and now we have something like
  FakeFlippers: ALLOWED
  STC: DISALLOWED
  something_i_cant_think_of_right_now: IN_BETWEEN

then we want to be able to dump that out into a webpage and have it perhaps group things nicely etc.
a button to say "show diff from {NMG, standard rando, HMG, No EG}" or something would be dope



user input should be yaml that maybe looks like:

    name: my_fun_game
    weights:
        FakeFlippers: ALLOWED
        STC: 5
        OverworldClipping: ALLOWED
        UnderworldClipping: 75

and maybe you should be able to say things like

    name: go_big_or_go_home
    groups:
        MajorFun:
            STC: ALLOWED
            OverworldYBA: ALLOWED
            TreeWarp: ALLOWED
        GDO:
            FakeFlippers: DISALLOWED
            BombJump: DISALLOWED
    weights:
        MajorFun: 3
        GDO: 3
        NMG: 94

and then force weights to add up to 100 and provide some understood defaults such as NMG (which of
course is not actually understood, but that's a different problem).

====

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