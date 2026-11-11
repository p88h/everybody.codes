/// Size of the minimum vertex cover of a bipartite graph.
///
/// `adj(u)` lists the right-side neighbours (in `0..n_right`) of left node `u`
/// in `0..n_left`, e.g. `|u| &lists[u][..]` or a slice of a flat edge array.
/// By König's theorem the minimum vertex cover equals the maximum matching,
/// which is found with Kuhn's algorithm: for each left node, look for an
/// augmenting path, re-routing already matched nodes when needed.
/// Worst case O(left nodes × edges).
pub fn min_vertex_cover<'a>(n_left: usize, n_right: usize, adj: impl Fn(usize) -> &'a [usize]) -> usize {
    // owner[v] = which left node is currently matched to right node v
    let mut owner = vec![usize::MAX; n_right];
    let mut done = vec![false; n_left];
    let mut matched = 0;

    // greedy start: match each left node to any free neighbour
    for u in 0..n_left {
        if let Some(&v) = adj(u).iter().find(|&&v| owner[v] == usize::MAX) {
            owner[v] = u;
            done[u] = true;
            matched += 1;
        }
    }

    // passes over the unmatched left nodes, clearing `seen` once per pass:
    // within a pass, nodes seen in a failed search stay dead ends until the
    // matching changes, so only a pass without progress proves we're done
    let mut seen = vec![false; n_right];
    loop {
        seen.fill(false);
        let mut progress = false;
        for u in 0..n_left {
            if !done[u] && augment(u, &adj, &mut owner, &mut seen) {
                done[u] = true;
                matched += 1;
                progress = true;
            }
        }
        if !progress {
            return matched;
        }
    }
}

// try to find a partner for left node u, re-routing existing owners if needed
fn augment<'a>(u: usize, adj: &impl Fn(usize) -> &'a [usize], owner: &mut [usize], seen: &mut [bool]) -> bool {
    for &v in adj(u) {
        if !seen[v] {
            seen[v] = true;
            // v is free, or its owner can move elsewhere
            if owner[v] == usize::MAX || augment(owner[v], adj, owner, seen) {
                owner[v] = u;
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(min_vertex_cover(0, 0, |_| &[]), 0);
    }

    #[test]
    fn test_needs_rerouting() {
        // left 0 -> {0, 1}, left 1 -> {0}: greedy gives 1, augmenting path gives 2
        let adj = [vec![0, 1], vec![0]];
        assert_eq!(min_vertex_cover(2, 2, |u| &adj[u][..]), 2);
    }

    #[test]
    fn test_star() {
        // one left node touching every right node: cover it alone
        let adj = [vec![0, 1, 2, 3]];
        assert_eq!(min_vertex_cover(1, 4, |u| &adj[u][..]), 1);
    }

    #[test]
    fn test_complete_2x3() {
        let adj = vec![vec![0, 1, 2], vec![0, 1, 2]];
        assert_eq!(min_vertex_cover(2, 3, |u| &adj[u][..]), 2);
    }
}
