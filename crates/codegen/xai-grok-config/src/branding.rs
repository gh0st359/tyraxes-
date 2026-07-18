//! Customer-facing product branding for Tyraxes.
//!
//! Internal crate names (`xai-grok-*`) stay unchanged. This module is the
//! single source of truth for display names, CLI names, and preferred
//! filesystem locations used by the surface rebrand.

/// Public product name shown in the TUI, docs, and CLI about text.
pub const PRODUCT_NAME: &str = "Tyraxes";

/// Short CLI / binary name preferred for installs and help.
pub const CLI_NAME: &str = "tyraxes";

/// Legacy CLI / binary name kept for compatibility.
pub const LEGACY_CLI_NAME: &str = "grok";

/// Preferred per-user config directory basename under `$HOME`.
pub const HOME_DIR_NAME: &str = ".tyraxes";

/// Legacy per-user config directory basename (still read when present).
pub const LEGACY_HOME_DIR_NAME: &str = ".grok";

/// Preferred project-local config directory name.
pub const PROJECT_DIR_NAME: &str = ".tyraxes";

/// Legacy project-local config directory name.
pub const LEGACY_PROJECT_DIR_NAME: &str = ".grok";

/// Env var for overriding the user config home (preferred).
pub const HOME_ENV: &str = "TYRAXES_HOME";

/// Legacy env var for the user config home.
pub const LEGACY_HOME_ENV: &str = "GROK_HOME";

/// Preferred system-wide config directory on Unix.
pub const SYSTEM_CONFIG_DIR: &str = "/etc/tyraxes";

/// Legacy system-wide config directory on Unix.
pub const LEGACY_SYSTEM_CONFIG_DIR: &str = "/etc/grok";

/// One-line CLI about string.
pub const CLI_ABOUT: &str = "Tyraxes — offensive security terminal agent";

/// Default identity injected into system prompts (`You are <label>…`).
pub const SYSTEM_PROMPT_LABEL: &str = "Tyraxes";

/// Red-team agent display identity.
pub const RED_TEAM_LABEL: &str = "Tyraxes Red Team";
