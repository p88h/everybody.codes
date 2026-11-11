/// Size of the minimum vertex cover of a bipartite graph.
///
/// `adj[u]` lists the right-side neighbours (in `0..n_right`) of left node `u`.
/// By König's theorem the minimum vertex cover equals the maximum matching,
/// which is found with Kuhn's algorithm: for each left node, look for an
/// augmenting path, re-routing already matched nodes when needed.
/// Worst case O(left nodes × edges).
pub fn min_vertex_cover(adj: &[Vec<usize>], n_right: usize) -> usize {
    // owner[v] = which left node is currently matched to right node v
    let mut owner = vec![usize::MAX; n_right];

    // one attempt per left node, fresh `seen` each time; count successes
    let mut seen = vec![false; n_right];
    let mut matched = 0;
    for u in 0..adj.len() {
        seen.fill(false);
        if augment(u, adj, &mut owner, &mut seen) {
            matched += 1;
        }
    }
    matched
}

// try to find a partner for left node u, re-routing existing owners if needed
fn augment(u: usize, adj: &[Vec<usize>], owner: &mut [usize], seen: &mut [bool]) -> bool {
    for &v in &adj[u] {
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
        assert_eq!(min_vertex_cover(&[], 0), 0);
    }

    #[test]
    fn test_needs_rerouting() {
        // left 0 -> {0, 1}, left 1 -> {0}: greedy gives 1, augmenting path gives 2
        assert_eq!(min_vertex_cover(&[vec![0, 1], vec![0]], 2), 2);
    }

    #[test]
    fn test_star() {
        // one left node touching every right node: cover it alone
        assert_eq!(min_vertex_cover(&[vec![0, 1, 2, 3]], 4), 1);
    }

    #[test]
    fn test_complete_2x3() {
        let adj = vec![vec![0, 1, 2], vec![0, 1, 2]];
        assert_eq!(min_vertex_cover(&adj, 3), 2);
    }
}
