// Reference solver frozen from e9543ad1 for numerical / scheduling regressions.
use super::*;
use crate::attribute::{
    AttributeName, AttributeType, AttributeVariableType, InvisibleAttributeType,
};
use crate::common::ReactionType;
use std::sync::Arc;
impl SimpleAttributeGraph2 {
    fn solve_reference(&self) -> SimpleAttributeGraphResult {
        let mut result: HashMap<AttributeNode, f64> = self
            .nodes
            .iter()
            .map(|(node, values)| (*node, values.sum()))
            .collect();
        let mut temp = result.clone();

        let sum_parent_values = |nodes: &HashMap<AttributeNode, f64>, node: AttributeNode| {
            node.get_parents()
                .iter()
                .map(|pa| *nodes.get(pa).unwrap_or(&0.0))
                .sum::<f64>()
        };

        let solve_edge = |edge: &Edge,
                          nodes_old: &HashMap<AttributeNode, f64>,
                          nodes_new: &mut HashMap<AttributeNode, f64>,
                          c: f64| {
            let from1_value = sum_parent_values(nodes_old, edge.from1);
            let from2_value = sum_parent_values(nodes_old, edge.from2);
            let value = (edge.func)(from1_value, from2_value) * c;
            *nodes_new.entry(edge.to).or_insert(0.0) += value;
        };

        let mut edge_lists = BTreeMap::new();
        let mut edge_static = Vec::new();
        for edge in self.edges.iter() {
            if edge.priority == EdgePriority::Static {
                edge_static.push(edge);
                continue;
            }
            edge_lists
                .entry(edge.priority as usize)
                .or_insert(Vec::new())
                .push(edge);
        }

        for edge in edge_static.iter() {
            solve_edge(edge, &result, &mut temp, 1.0);
        }
        result = temp.clone();

        for list in edge_lists.values() {
            for edge in list.iter() {
                solve_edge(edge, &result, &mut temp, 1.0);
            }
            for edge in edge_static.iter() {
                solve_edge(edge, &result, &mut temp, -1.0);
            }
            result = temp.clone();
            for edge in edge_static.iter() {
                solve_edge(edge, &result, &mut temp, 1.0);
            }
            result = temp.clone();
        }

        SimpleAttributeGraphResult {
            map: result,
            characters: self.characters.clone(),
        }
    }
}

fn invisible(mask: usize) -> AttributeNode {
    AttributeNode::new_invisible(
        7,
        InvisibleAttributeType::new(
            AttributeVariableType::Bonus,
            (mask & 1 != 0).then_some(Element::Pyro),
            (mask & 2 != 0).then_some(SkillType::ChargedAttack),
            (mask & 4 != 0).then_some(ReactionType::Vaporize),
        ),
    )
}

#[test]
fn parent_visit_preserves_order_for_every_wildcard_shape() {
    for node in (0..8)
        .map(invisible)
        .chain([AttributeNode::new_panel(7, AttributeName::ATK)])
    {
        let mut visited = Vec::new();
        node.for_each_parent(|parent| visited.push(parent));
        assert!(visited == node.get_parents());
    }
}

#[test]
fn solver_matches_reference_bits_across_priorities_and_wildcards() {
    let nodes: Vec<_> = (0..8)
        .map(invisible)
        .chain([
            AttributeNode::new_panel(0, AttributeName::ATKBase),
            AttributeNode::new_panel(0, AttributeName::ATK),
            AttributeNode::new_panel(1, AttributeName::HP),
        ])
        .collect();
    for seed in 0..64 {
        let mut graph = SimpleAttributeGraph2::new_with_characters(Vec::new());
        for (i, &node) in nodes.iter().enumerate() {
            graph.set_value_to_internal(node, "a", ((i * 17 + seed) % 23) as f64 / 10.0);
            graph.set_value_by_internal(node, "b", -0.3);
        }
        for (i, priority) in [
            EdgePriority::Static,
            EdgePriority::Base,
            EdgePriority::Common,
            EdgePriority::Invisible,
            EdgePriority::Last,
        ]
        .into_iter()
        .enumerate()
        {
            for j in 0..9 {
                let from = nodes[(j + seed) % nodes.len()];
                let from2 = if j % 2 == 0 {
                    from
                } else {
                    nodes[(j + 2) % nodes.len()]
                };
                let to = nodes[(i + j + 5) % nodes.len()];
                graph.add_edge(
                    from,
                    from2,
                    to,
                    Arc::new(|x, y| (x * 0.1 - y * 0.2).clamp(-1000.0, 1000.0)),
                    "edge",
                    priority,
                );
            }
        }
        let expected = graph.solve_reference();
        let actual = graph.solve();
        assert_eq!(actual.map.len(), expected.map.len());
        for (node, &value) in &expected.map {
            assert_eq!(actual.map[node].to_bits(), value.to_bits(), "seed {seed}");
        }
    }
}

#[test]
fn parent_sums_preserve_cancellation_signed_zero_and_nonfinite_values() {
    let query = invisible(7);
    for values in [
        [1e16, 1.0, -1e16, 1.0, -0.0, 0.1, 0.2, -0.3],
        [-0.0; 8],
        [f64::NAN; 8],
        [f64::INFINITY; 8],
    ] {
        let mut graph = SimpleAttributeGraph2::new_with_characters(Vec::new());
        for (parent, value) in query.get_parents().into_iter().zip(values) {
            graph.set_value_to_internal(parent, "value", value);
        }
        let to = AttributeNode::new_panel(0, AttributeName::ATK);
        graph.add_edge(
            query,
            query,
            to,
            Arc::new(|a, b| a + b),
            "edge",
            EdgePriority::Common,
        );
        let actual = graph.solve();
        let expected = graph.solve_reference();
        assert_eq!(actual.map[&to].to_bits(), expected.map[&to].to_bits());
        let old_query = query.get_parents().iter().fold(0.0, |sum, node| {
            sum + actual.map.get(node).copied().unwrap_or(0.0)
        });
        assert_eq!(
            actual.get_attribute_value(query).to_bits(),
            old_query.to_bits()
        );
    }
}
