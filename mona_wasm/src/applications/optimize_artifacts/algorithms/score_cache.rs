//! Request-local memoization of exact solved numeric inputs, never item identity.
use mona::attribute::{AttributeNode, AttributeType, SimpleAttributeResult};
use std::collections::HashMap;
use std::mem::size_of;

const MAX_ENTRIES: usize = 256;
const MAX_NODES: usize = 256;
const MAX_KEY_BYTES: usize = 512 * 1024;
const MISS_LIMIT: usize = 64;
const LOOKUP_WINDOW: usize = 128;
const MIN_WINDOW_HITS: usize = LOOKUP_WINDOW / 4;

#[derive(Hash, Eq, PartialEq)]
struct ScoreKey {
    character_id: usize,
    nodes: Vec<(AttributeNode, u64)>,
}

fn node_order(node: AttributeNode) -> (usize, u8, usize, usize, usize, usize) {
    match node.attribute_type {
        AttributeType::Panel(name) => (node.character_id, 0, name as usize, 0, 0, 0),
        AttributeType::Invisible(kind) => (
            node.character_id,
            1,
            kind.attribute_variable_type as usize,
            kind.element.map_or(0, |v| v as usize + 1),
            kind.skill.map_or(0, |v| v as usize + 1),
            kind.reaction.map_or(0, |v| v as usize + 1),
        ),
    }
}

impl ScoreKey {
    fn new(solved: &SimpleAttributeResult) -> Self {
        let mut nodes: Vec<_> = solved
            .result
            .map
            .iter()
            .map(|(&node, &value)| (node, value.to_bits()))
            .collect();
        nodes.sort_unstable_by_key(|&(node, _)| node_order(node));
        Self {
            character_id: solved.character_id,
            nodes,
        }
    }

    fn bytes(&self) -> usize {
        size_of::<Self>() + self.nodes.capacity() * size_of::<(AttributeNode, u64)>()
    }
}

/// Construct once inside a single optimization call, with immutable evaluation
/// context. Not exported as a reusable cross-request or cross-target cache.
pub(super) struct SolvedScoreCache {
    entries: HashMap<ScoreKey, f64>,
    key_bytes: usize,
    consecutive_misses: usize,
    window_lookups: usize,
    window_hits: usize,
    disabled: bool,
}

impl Default for SolvedScoreCache {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            key_bytes: 0,
            consecutive_misses: 0,
            window_lookups: 0,
            window_hits: 0,
            disabled: false,
        }
    }
}

impl SolvedScoreCache {
    fn disable(&mut self) {
        // Release retained keys and table capacity when there is no useful reuse.
        self.entries = HashMap::new();
        self.key_bytes = 0;
        self.disabled = true;
    }

    fn record_lookup(&mut self, hit: bool) {
        self.window_lookups += 1;
        self.window_hits += usize::from(hit);
        if self.window_lookups == LOOKUP_WINDOW {
            // Occasional hits must not keep an expensive, mostly idle cache
            // alive indefinitely. This is a cost heuristic, not a speed claim.
            if self.window_hits < MIN_WINDOW_HITS {
                self.disable();
            }
            self.window_lookups = 0;
            self.window_hits = 0;
        }
    }

    pub(super) fn score(
        &mut self,
        solved: &SimpleAttributeResult,
        compute: impl FnOnce() -> Option<f64>,
    ) -> Option<f64> {
        if self.disabled {
            return compute();
        }
        if solved.result.map.len() > MAX_NODES {
            self.disable();
            return compute();
        }
        let key = ScoreKey::new(solved);
        // Hash collisions are resolved by the complete key, including every
        // node and the original f64 bits. No rounding or signed-zero merging.
        if let Some(&score) = self.entries.get(&key) {
            self.consecutive_misses = 0;
            self.record_lookup(true);
            return Some(score);
        }
        let score = compute();
        self.consecutive_misses += 1;
        self.record_lookup(false);
        if self.consecutive_misses >= MISS_LIMIT || score.is_none() {
            self.disable();
            return score;
        }
        if self.disabled {
            return score;
        }
        if let Some(value) = score.filter(|value| value.is_finite()) {
            let bytes = key.bytes();
            if self.entries.len() >= MAX_ENTRIES || self.key_bytes + bytes > MAX_KEY_BYTES {
                self.entries.clear();
                self.key_bytes = 0;
            }
            self.key_bytes += bytes;
            self.entries.insert(key, value);
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mona::attribute::{
        AttributeName, AttributeVariableType, InvisibleAttributeType, SimpleAttributeGraphResult,
    };
    use mona::common::{Element, ReactionType, SkillType};

    fn solved(values: &[(AttributeNode, f64)]) -> SimpleAttributeResult {
        SimpleAttributeResult {
            character_id: 0,
            result: SimpleAttributeGraphResult {
                map: values.iter().copied().collect(),
                characters: Vec::new(),
            },
        }
    }

    #[test]
    fn keys_are_order_independent_but_preserve_every_numeric_bit() {
        let a = AttributeNode::new_panel(0, AttributeName::ATKBase);
        let b = AttributeNode::new_panel(0, AttributeName::HPBase);
        let mut cache = SolvedScoreCache::default();
        assert_eq!(
            cache.score(&solved(&[(a, 1.0), (b, 2.0)]), || Some(7.0)),
            Some(7.0)
        );
        assert_eq!(
            cache.score(&solved(&[(b, 2.0), (a, 1.0)]), || panic!("must hit")),
            Some(7.0)
        );
        assert_eq!(
            cache.score(
                &solved(&[(a, f64::from_bits(1.0f64.to_bits() + 1)), (b, 2.0)]),
                || Some(8.0)
            ),
            Some(8.0)
        );
        for (value, score) in [(0.0, 9.0), (-0.0, 10.0)] {
            assert_eq!(
                cache.score(&solved(&[(a, value)]), || Some(score)),
                Some(score)
            );
        }
        assert_eq!(cache.score(&solved(&[]), || Some(11.0)), Some(11.0));
        let mut other_character = solved(&[(a, 1.0), (b, 2.0)]);
        other_character.character_id = 1;
        assert_eq!(cache.score(&other_character, || Some(12.0)), Some(12.0));
    }

    #[test]
    fn invisible_node_dimensions_never_alias() {
        let mut cache = SolvedScoreCache::default();
        for id in 0..2 {
            for mask in 0..8 {
                let node = AttributeNode::new_invisible(
                    id,
                    InvisibleAttributeType::new(
                        AttributeVariableType::Bonus,
                        (mask & 1 != 0).then_some(Element::Pyro),
                        (mask & 2 != 0).then_some(SkillType::ChargedAttack),
                        (mask & 4 != 0).then_some(ReactionType::Vaporize),
                    ),
                );
                let input = solved(&[(node, 1.0)]);
                let score = (id * 8 + mask) as f64;
                assert_eq!(cache.score(&input, || Some(score)), Some(score));
                assert_eq!(cache.score(&input, || panic!("must hit")), Some(score));
            }
        }
        assert_eq!(cache.entries.len(), 16);
    }

    #[test]
    fn unhelpful_or_oversized_cache_disables_and_releases_memory() {
        let node = AttributeNode::new_panel(0, AttributeName::ATKBase);
        let mut cache = SolvedScoreCache::default();
        for i in 0..MISS_LIMIT {
            assert_eq!(
                cache.score(&solved(&[(node, i as f64)]), || Some(i as f64)),
                Some(i as f64)
            );
        }
        assert!(cache.disabled);
        assert!(cache.entries.is_empty());
        assert_eq!(cache.entries.capacity(), 0);
        assert_eq!(
            cache.score(&solved(&[(node, 0.0)]), || Some(123.0)),
            Some(123.0)
        );
        let large = solved(
            &(0..=MAX_NODES)
                .map(|i| (AttributeNode::new_panel(i, AttributeName::ATKBase), 1.0))
                .collect::<Vec<_>>(),
        );
        let mut cache = SolvedScoreCache::default();
        assert_eq!(cache.score(&large, || Some(4.0)), Some(4.0));
        assert!(cache.disabled);
        assert!(cache.entries.is_empty());
    }

    #[test]
    fn capacity_is_bounded_and_eviction_does_not_change_scores() {
        let node = AttributeNode::new_panel(0, AttributeName::ATKBase);
        let mut cache = SolvedScoreCache::default();
        for i in 0..MAX_ENTRIES * 3 {
            let input = solved(&[(node, i as f64)]);
            assert_eq!(cache.score(&input, || Some(i as f64)), Some(i as f64));
            assert_eq!(cache.score(&input, || panic!("must hit")), Some(i as f64));
            assert!(cache.entries.len() <= MAX_ENTRIES);
            assert!(cache.key_bytes <= MAX_KEY_BYTES);
        }
        assert!(!cache.disabled);
    }

    #[test]
    fn occasional_hits_do_not_keep_a_mostly_idle_cache_alive() {
        let node = AttributeNode::new_panel(0, AttributeName::ATKBase);
        let mut cache = SolvedScoreCache::default();
        for i in 0..LOOKUP_WINDOW {
            assert!(!cache.disabled);
            let value = if i % 8 == 0 { 0.0 } else { i as f64 };
            assert_eq!(
                cache.score(&solved(&[(node, value)]), || Some(value)),
                Some(value)
            );
        }
        assert!(cache.disabled);
        assert_eq!(cache.entries.capacity(), 0);
        assert_eq!(cache.key_bytes, 0);
    }

    #[test]
    fn wide_keys_evict_at_the_byte_budget_before_the_entry_limit() {
        let mut cache = SolvedScoreCache::default();
        let mut previous_len = 0;
        let mut evictions = 0;
        for i in 0..MAX_ENTRIES {
            let input = solved(
                &(0..MAX_NODES)
                    .map(|id| {
                        (
                            AttributeNode::new_panel(id, AttributeName::ATKBase),
                            i as f64,
                        )
                    })
                    .collect::<Vec<_>>(),
            );
            assert_eq!(cache.score(&input, || Some(i as f64)), Some(i as f64));
            assert_eq!(cache.score(&input, || panic!("must hit")), Some(i as f64));
            if cache.entries.len() < previous_len {
                evictions += 1;
            }
            previous_len = cache.entries.len();
            assert!(cache.key_bytes <= MAX_KEY_BYTES);
            assert!(cache.entries.len() < MAX_ENTRIES);
        }
        assert!(evictions > 0);
        assert!(!cache.disabled);
    }

    #[test]
    fn invalid_scores_and_missing_hooks_are_not_cached() {
        let input = solved(&[]);
        let mut cache = SolvedScoreCache::default();
        assert!(cache.score(&input, || Some(f64::NAN)).unwrap().is_nan());
        assert!(cache.entries.is_empty());
        assert_eq!(
            cache.score(&input, || Some(f64::INFINITY)),
            Some(f64::INFINITY)
        );
        assert!(cache.entries.is_empty());
        assert_eq!(cache.score(&input, || None), None);
        assert!(cache.disabled);
        assert_eq!(cache.score(&input, || Some(1.0)), Some(1.0));
    }
}
