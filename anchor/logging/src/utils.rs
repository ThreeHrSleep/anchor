use std::collections::HashSet;

use tracing_subscriber::filter::FilterFn;
use workspace_members::workspace_crates;

const WORKSPACE_CRATES: &[&str] = workspace_crates!();

/// Constructs a filter which only permits logging from crates which are members of the workspace.
pub fn build_workspace_filter(deps: Vec<String>,)
-> Result<FilterFn<impl Fn(&tracing::Metadata) -> bool + Clone>, String> {
    let mut workspace_crates: HashSet<String> =
        WORKSPACE_CRATES.iter().map(|s| s.to_string()).collect();
        
    workspace_crates.extend(deps);

    Ok(tracing_subscriber::filter::FilterFn::new(move |metadata| {
        let target_crate = metadata.target().split("::").next().unwrap_or("");
        workspace_crates.contains(target_crate)
    }))
}
