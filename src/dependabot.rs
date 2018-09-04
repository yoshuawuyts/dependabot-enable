/// Dependabot URL to update configs.
pub const UPDATE_CONFIGS_URL: &str =
  "https://api.dependabot.com/update_configs";

/// Enable dependabot for a repo.
pub fn enable(token: String, user: String, repo: String) -> ::Result<()> {
  unimplemented!();
}

/// Rules on how to merge dependencies.
enum AutomergeRule {
  /// Never merge.
  Never,
  /// Only merge security updates.
  Security,
  /// Only merge patch versions
  Patch,
  /// Merge minor versions & patch versions.
  Minor,
}

/// Configuration for a repo.
///
/// https://github.com/dependabot/api-docs#create-an-update-config-for-a-repo
///
/// ```txt
/// POST https://api.dependabot.com/update_configs
/// ```
struct UpdateConfigs {
  /// GitHub repo id.
  ///
  /// Found by requesting: `https://api.github.com/repos/<username>/<repo>`
  repo_id: usize,
  /// Package manager.
  package_manager: String,
  /// How often to update. Set to "daily" in most cases.
  update_schedule: String,
  /// Directory to target. Set to "/" in most cases.
  directory: String,
  /// GitHub account id (e.g. id for an org).
  account_id: usize,
  /// Either user or org. Should be found from GitHub.
  account_type: String,
  /// The branch to create PRs against.
  target_branch: Option<String>,
  /// Ignore updates that are out-of-range of the manifest file.
  lockfile_only: Option<bool>,
  /// Only generate PRs for updates that fix a security vulnerability.
  security_updates_only: Option<bool>,
  /// One of "never", "security", "patch", or "minor".
  automerge_rule_development_deps: Option<AutomergeRule>,
  /// One of "never", "security", "patch", or "minor".
  automerge_rule_runtime_deps: Option<AutomergeRule>,
}
