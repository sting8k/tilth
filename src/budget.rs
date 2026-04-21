use crate::types::estimate_tokens;

const OUTLINE_KEYWORDS: &[&str] = &[
    "fn ",
    "class ",
    "prop ",
    "mod ",
    "struct ",
    "enum ",
    "trait ",
    "interface ",
    "type ",
];

/// Apply token budget to output. Works backwards from the cap:
/// 1. Reserve 50 tokens for header
/// 2. Keep a progressive line-wise prefix of the body (never empty when possible)
/// 3. Prefer clean section boundaries when they are not overly aggressive
/// 4. Never exceed the budget
///
/// For outline outputs, guarantees at least a few structural entries so the
/// agent still sees the file skeleton even with a tight budget.
pub fn apply(output: &str, budget: u64) -> String {
    let current = estimate_tokens(output.len() as u64);
    if current <= budget {
        return output.to_string();
    }

    let header_reserve = 50u64;
    let content_budget = budget.saturating_sub(header_reserve);
    let max_bytes = (content_budget * 4) as usize; // inverse of estimate_tokens

    // Find the first newline after the header (first line)
    let header_end = output.find('\n').unwrap_or(output.len());
    let header = &output[..header_end];
    let body = &output[header_end..];

    if body.len() <= max_bytes {
        return output.to_string();
    }

    // Detect outline mode from the header line.
    let is_outline = output
        .lines()
        .next()
        .is_some_and(|l| l.ends_with("[outline]"));
    let min_entries = if is_outline { 5usize } else { 0usize };

    // Build a progressive prefix line-by-line. For outlines we may exceed the
    // raw byte cap slightly so that at least `min_entries` symbols are kept.
    let mut progressive_cut = 0usize;
    let mut entry_count = 0usize;
    for line in body.split_inclusive('\n') {
        let would_exceed = progressive_cut + line.len() > max_bytes;
        let has_enough = entry_count >= min_entries;
        if would_exceed && has_enough {
            break;
        }
        progressive_cut += line.len();
        if is_outline && OUTLINE_KEYWORDS.iter().any(|kw| line.contains(kw)) {
            entry_count += 1;
        }
    }
    if progressive_cut == 0 {
        progressive_cut = body.floor_char_boundary(max_bytes.min(body.len()));
    }

    let truncated = &body[..progressive_cut];

    // Prefer section boundaries (search output blocks), but only when that still
    // preserves a meaningful chunk of the selected prefix.
    let section_cut = truncated
        .rfind("\n\n##")
        .or_else(|| truncated.rfind("\n\n"));
    let line_cut = truncated.rfind('\n');

    let mut cut_point = section_cut
        .filter(|p| *p >= progressive_cut / 3)
        .or(line_cut)
        .unwrap_or(progressive_cut);

    if cut_point == 0 {
        cut_point = progressive_cut;
    }

    let clean_body = &body[..cut_point];

    let omitted_bytes = output.len().saturating_sub(header_end + cut_point);
    let remaining_tokens = estimate_tokens(omitted_bytes as u64);
    format!(
        "{header}{clean_body}\n\n... truncated ({remaining_tokens} tokens omitted, budget: {budget})"
    )
}
