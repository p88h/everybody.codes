use std::ops::Range;

/// Bellman-Ford: lowers the potentials `pot` along the edges out of `nodes`.
///
/// `edges(s)` lists the edges out of node `s` and `edge(e)` gives an edge's target and weight.
/// Edges may lead out of `nodes`, but only those out of `nodes` are relaxed, so a graph made of
/// pieces joined one way can be done piece by piece. `pot` is the starting point; all zeros is
/// like a virtual source with a 0-weight edge to every node.
///
/// Without a negative loop, leaves `pot[t] <= pot[s] + weight` for every edge s -> t out of
/// `nodes`. Otherwise returns the edges of some negative loop, in order.
/// O(nodes × edges out of them).
pub fn bellman_ford<'a, E>(
    nodes: Range<usize>,
    pot: &mut [i64],
    edges: impl Fn(usize) -> &'a [E],
    edge: impl Fn(&E) -> (usize, i64),
) -> Result<(), Vec<&'a E>> {
    // parent[t]: the node and edge that last lowered pot[t]
    let mut parent: Vec<Option<(usize, &E)>> = vec![None; pot.len()];
    let mut changed = None;
    // a path through the range has at most one edge per node in it, so with no negative loop
    // a round per node settles everything
    for _ in 0..=nodes.len() {
        changed = None;
        for s in nodes.clone() {
            for e in edges(s) {
                let (t, weight) = edge(e);
                if pot[s] + weight < pot[t] {
                    pot[t] = pot[s] + weight;
                    parent[t] = Some((s, e));
                    changed = Some(t);
                }
            }
        }
        if changed.is_none() {
            return Ok(());
        }
    }

    // still improving after that: stepping back as far along the parents lands on a negative loop
    let mut on_loop = changed.unwrap();
    for _ in 0..=nodes.len() {
        on_loop = parent[on_loop].unwrap().0;
    }
    let mut cycle = Vec::new();
    let mut s = on_loop;
    loop {
        let (from, e) = parent[s].unwrap();
        cycle.push(e);
        s = from;
        if s == on_loop {
            break;
        }
    }
    cycle.reverse();
    Err(cycle)
}

#[cfg(test)]
mod tests {
    use super::*;

    type Edges = Vec<Vec<(usize, i64)>>;

    fn run<'a>(adj: &'a Edges, nodes: Range<usize>, pot: &mut [i64]) -> Result<(), Vec<&'a (usize, i64)>> {
        bellman_ford(nodes, pot, |s| &adj[s][..], |&e| e)
    }

    #[test]
    fn potentials_without_negative_loop() {
        // 0 -> 1 -> 2 -> 0 weighs 3 - 4 + 2 = 1
        let adj: Edges = vec![vec![(1, 3)], vec![(2, -4)], vec![(0, 2), (3, -1)], vec![]];
        let mut pot = vec![0; 4];
        assert!(run(&adj, 0..4, &mut pot).is_ok());
        for (s, out) in adj.iter().enumerate() {
            for &(t, w) in out {
                assert!(pot[t] <= pot[s] + w);
            }
        }
    }

    #[test]
    fn finds_negative_loop() {
        // 1 -> 2 -> 3 -> 1 weighs 1 - 3 + 1 = -1, reached from 0
        let adj: Edges = vec![vec![(1, 5)], vec![(2, 1)], vec![(3, -3)], vec![(1, 1), (4, 0)], vec![]];
        let mut pot = vec![0; 5];
        let cycle = run(&adj, 0..5, &mut pot).unwrap_err();
        assert_eq!(cycle.iter().map(|e| e.1).sum::<i64>(), -1);
        // consecutive edges chain up and the last one closes the loop
        let start = cycle.last().unwrap().0;
        let mut at = start;
        for &&(t, w) in &cycle {
            assert!(adj[at].contains(&(t, w)));
            at = t;
        }
        assert_eq!(at, start);
    }

    #[test]
    fn only_relaxes_edges_out_of_the_range() {
        // a negative loop among 2 and 3 is out of range; 0 -> 1 -> 2 leads into it
        let adj: Edges = vec![vec![(1, -1)], vec![(2, -1)], vec![(3, -5)], vec![(2, 1)]];
        let mut pot = vec![0; 4];
        assert!(run(&adj, 0..2, &mut pot).is_ok());
        assert_eq!(pot, vec![0, -1, -2, 0]);
        // then pick up from there with the next piece
        assert!(run(&adj, 2..4, &mut pot).is_err());
    }
}
