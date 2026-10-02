use crate::types::{Analysis, Diagnostics};

/// Compute confidence score for a conversion result.
pub fn compute_confidence(
    analysis: &Analysis,
    template_matched: bool,
    conversion_ratio: f32,
    unknown_count: usize,
    protected_count: usize,
) -> f32 {
    let mut score = 0.55f32;

    if template_matched {
        score += 0.30;
    } else {
        score += conversion_ratio * 0.20;
    }

    if analysis.intent != crate::intent::Intent::Statement {
        score += 0.05;
    }

    if analysis.verb.is_some() {
        score += 0.05;
    }

    let penalty = (unknown_count as f32 * 0.03).min(0.25);
    score -= penalty;

    if protected_count > 0 {
        score += 0.02;
    }

    score.clamp(0.0, 1.0)
}

pub fn build_diagnostics(
    template_matched: bool,
    conversion_ratio: f32,
    unknown_count: usize,
    protected_count: usize,
) -> Diagnostics {
    Diagnostics {
        template_matched,
        conversion_ratio,
        unknown_token_count: unknown_count,
        protected_token_count: protected_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::Intent;
    use crate::tense::Tense;

    #[test]
    fn high_confidence_for_template() {
        let analysis = Analysis {
            intent: Intent::Statement,
            tense: Tense::Future,
            verb: Some("call".into()),
            ..Default::default()
        };
        let c = compute_confidence(&analysis, true, 0.8, 0, 0);
        assert!(c >= 0.8);
    }

    #[test]
    fn lower_confidence_for_unknowns() {
        let analysis = Analysis::default();
        let c = compute_confidence(&analysis, false, 0.2, 5, 0);
        assert!(c < 0.6);
    }
}
