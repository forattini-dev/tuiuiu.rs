//! Templates - High-level layout patterns and page structures
//!
//! Templates provide reusable layout structures:
//! - **Stack Layouts**: VStack, HStack, Center, FullScreen, Spacer
//! - **App Structure**: AppShell, Page, Header, Footer, Sidebar, StatusBar
//! - **Containers**: Container, Card, Panel

mod stack;
mod app_shell;
mod header;
mod status_bar;
mod page;

// Re-export stack layouts
pub use stack::{
    VStack, HStack, Center, FullScreen, Spacer,
    VAlign, HAlign, HJustify, LayoutProps,
};

// Re-export app structure templates
pub use app_shell::{AppShell, SidebarPosition};
pub use header::{Header, NavItem};
pub use status_bar::{StatusBar, StatusItem, StatusMode};
pub use page::{Page, Breadcrumb};

// =============================================================================
// Stub Types (to be implemented)
// =============================================================================

/// Footer component (simple bottom section).
///
/// TODO: Implement with text, links, and copyright.
pub struct Footer;

/// Sidebar component (navigation panel).
///
/// TODO: Implement with nav items, sections, and collapse.
pub struct Sidebar;

/// Container with max-width constraint.
///
/// TODO: Implement with responsive sizing.
pub struct Container;
