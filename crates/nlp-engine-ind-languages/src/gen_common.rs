use crate::generator::{
    GeneratedOutput, GenerationContext, IntentKind, LanguageGenerator, TenseKind,
};
use crate::lexicon::Lexicon;

fn finalize(
    ctx: &GenerationContext,
    text: String,
    template_matched: bool,
    converted: usize,
) -> GeneratedOutput {
    let mut text = text;
    if !ctx.leading_expressive.is_empty() {
        text = format!("{} {}", ctx.leading_expressive.trim(), text);
    }
    if !ctx.trailing_punct.is_empty() {
        text.push_str(&ctx.trailing_punct);
    }
    let total = ctx.tokens.len().max(1);
    GeneratedOutput {
        text,
        conversion_ratio: converted as f32 / total as f32,
        template_matched,
    }
}

fn build_from_parts(
    ctx: &GenerationContext,
    template_matched: bool,
    parts: &[&str],
) -> GeneratedOutput {
    let text = parts
        .iter()
        .filter(|p| !p.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ");
    finalize(ctx, text, template_matched, parts.len())
}

pub fn generate_conversational(
    gen: &dyn LanguageGenerator,
    ctx: &GenerationContext,
) -> GeneratedOutput {
    let lex = gen.lexicon();
    let norm = ctx.normalized.to_ascii_lowercase();

    if let Some(template) = lex.match_template(&norm) {
        return finalize(ctx, template.to_string(), true, ctx.tokens.len());
    }

    if let Some(phrase) = lex.lookup_phrase(&norm) {
        return build_from_parts(ctx, true, &[phrase]);
    }

    if norm.contains("do not worry") && norm.contains("handle") {
        let worry = lex.lookup_phrase("dont worry").unwrap_or("do not worry");
        let handle = lex.verb_form("handle_future").unwrap_or("handle");
        return build_from_parts(ctx, true, &[worry, handle]);
    }

    if norm.contains("let us meet") && norm.contains("tomorrow") {
        let tomorrow = lex
            .particles
            .get("tomorrow")
            .map(String::as_str)
            .unwrap_or("tomorrow");
        let meet = lex.lookup_phrase("lets meet").unwrap_or("meet");
        return build_from_parts(ctx, true, &[tomorrow, meet]);
    }
    if norm.contains("let us meet") {
        let meet = lex.lookup_phrase("lets meet").unwrap_or("meet");
        return build_from_parts(ctx, true, &[meet]);
    }

    if ctx.intent == IntentKind::Request && norm.contains("can you") && norm.contains("send") {
        let me = lex.pronouns.get("me").map(String::as_str).unwrap_or("me");
        let obj = ctx
            .tokens
            .iter()
            .find(|t| {
                let l = t.to_ascii_lowercase();
                matches!(l.as_str(), "file" | "project" | "link" | "message")
            })
            .map(String::as_str)
            .unwrap_or("file");
        let send = lex.verb_form("send_request").unwrap_or("send");
        if norm.contains("bro") {
            return build_from_parts(ctx, true, &["bro", me, obj, send]);
        }
        return build_from_parts(ctx, true, &[me, obj, send]);
    }

    if (ctx.tense == TenseKind::Future || ctx.tense == TenseKind::Intentional)
        && norm.contains("call")
        && norm.contains("later")
    {
        let call = lex.verb_form("call_future").unwrap_or("call");
        return build_from_parts(ctx, true, &["later", call]);
    }

    if ctx.intent == IntentKind::Question && norm.contains("coming") {
        let q = lex.verb_form("come_question").unwrap_or("coming");
        return build_from_parts(ctx, true, &[q]);
    }

    if ctx.intent == IntentKind::Question && norm.contains("what") && norm.contains("doing") {
        let q = lex.lookup_phrase("what are you doing").unwrap_or("doing");
        return build_from_parts(ctx, true, &[q]);
    }

    if norm.contains("busy") && norm.contains("now") {
        let now = lex
            .particles
            .get("now")
            .map(String::as_str)
            .unwrap_or("now");
        let busy = lex.verb_form("busy_present").unwrap_or("busy");
        return build_from_parts(ctx, true, &[now, busy]);
    }

    if ctx.intent == IntentKind::Question && norm.contains("how are you") {
        let q = lex.lookup_phrase("how are you").unwrap_or("how are you");
        return build_from_parts(ctx, true, &[q]);
    }

    if ctx.intent == IntentKind::Thanks {
        return build_from_parts(ctx, true, &["thanks"]);
    }

    if ctx.intent == IntentKind::Greeting {
        if norm.contains("morning") {
            return build_from_parts(ctx, true, &["good morning"]);
        }
        return build_from_parts(ctx, true, &["hey"]);
    }

    fallback_token_generation(gen, ctx)
}

fn fallback_token_generation(
    gen: &dyn LanguageGenerator,
    ctx: &GenerationContext,
) -> GeneratedOutput {
    let mut converted = 0usize;
    let mut out = Vec::new();

    for token in &ctx.tokens {
        if gen.should_preserve_english(token, ctx) {
            out.push(token.clone());
            continue;
        }
        if let Some(tr) = gen.lexicon().lookup_word(token) {
            out.push(tr.to_string());
            converted += 1;
        } else {
            out.push(token.clone());
        }
    }

    finalize(ctx, out.join(" "), false, converted)
}

pub struct LanguageModule {
    pub language: crate::Language,
    pub lexicon: Lexicon,
}

impl LanguageGenerator for LanguageModule {
    fn language(&self) -> crate::Language {
        self.language
    }

    fn lexicon(&self) -> &Lexicon {
        &self.lexicon
    }

    fn generate(&self, ctx: &GenerationContext) -> GeneratedOutput {
        generate_conversational(self, ctx)
    }
}

macro_rules! language_module {
    ($fn_name:ident, $variant:expr, $path:expr) => {
        pub fn $fn_name() -> &'static $crate::gen_common::LanguageModule {
            static INSTANCE: std::sync::OnceLock<$crate::gen_common::LanguageModule> =
                std::sync::OnceLock::new();
            INSTANCE.get_or_init(|| $crate::gen_common::LanguageModule {
                language: $variant,
                lexicon: $crate::Lexicon::from_embedded(include_str!($path)),
            })
        }
    };
}

pub(crate) use language_module;
