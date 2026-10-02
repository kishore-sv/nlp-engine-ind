use crate::gen_common::language_module;
use crate::Language;

language_module!(
    generator,
    Language::Hindi,
    "../../../data/hindi/lexicon.json"
);
