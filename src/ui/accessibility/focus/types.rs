//! Focus types and configurations

use gpui::{px, Hsla, Pixels};
use serde::{Deserialize, Serialize};

/// Focus ring style configuration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FocusRingStyle {
    /// Width of the focus ring
    pub width: Pixels,
    /// Offset from the element (can be negative for inset)
    pub offset: Pixels,
    /// Color of the focus ring
    pub color: Hsla,
    /// Border radius (matches element by default)
    pub radius: Option<Pixels>,
    /// Use outline style instead of border
    pub use_outline: bool,
}

impl Default for FocusRingStyle {
    fn default() -> Self {
        Self::standard()
    }
}

impl FocusRingStyle {
    /// Standard focus ring (2px solid, 2px offset)
    pub fn standard() -> Self {
        Self {
            width: px(2.0),
            offset: px(2.0),
            color: Hsla {
                h: 210.0 / 360.0,
                s: 1.0,
                l: 0.60,
                a: 1.0,
            },
            radius: None,
            use_outline: true,
        }
    }

    /// High contrast focus ring (3px solid, bright color)
    pub fn high_contrast() -> Self {
        Self {
            width: px(3.0),
            offset: px(2.0),
            color: Hsla {
                h: 60.0 / 360.0, // Yellow for maximum visibility
                s: 1.0,
                l: 0.50,
                a: 1.0,
            },
            radius: None,
            use_outline: true,
        }
    }

    /// Subtle focus ring for less intrusive indication
    pub fn subtle() -> Self {
        Self {
            width: px(1.0),
            offset: px(1.0),
            color: Hsla {
                h: 210.0 / 360.0,
                s: 0.8,
                l: 0.55,
                a: 0.8,
            },
            radius: None,
            use_outline: true,
        }
    }

    /// Inset focus ring (inside the element)
    pub fn inset() -> Self {
        Self {
            width: px(2.0),
            offset: px(-2.0),
            color: Hsla {
                h: 210.0 / 360.0,
                s: 1.0,
                l: 0.60,
                a: 1.0,
            },
            radius: None,
            use_outline: false,
        }
    }

    /// Custom focus ring
    pub fn custom(width: f32, offset: f32, color: Hsla) -> Self {
        Self {
            width: px(width),
            offset: px(offset),
            color,
            radius: None,
            use_outline: true,
        }
    }

    /// Set the corner radius
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Some(px(radius));
        self
    }

    /// Set the color
    pub fn with_color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }
}

/// Focus zone for grouping focusable elements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FocusZone {
    /// Main content area
    Main,
    /// Sidebar navigation
    Sidebar,
    /// Tab bar
    TabBar,
    /// Chat input area
    ChatInput,
    /// Modal dialog
    Modal,
    /// Command palette
    CommandPalette,
    /// Settings panel
    Settings,
    /// File explorer
    FileExplorer,
    /// Terminal
    Terminal,
    /// Custom zone
    Custom(String),
}

impl FocusZone {
    /// Get the tab order for this zone (lower = earlier)
    pub fn tab_order(&self) -> u32 {
        match self {
            FocusZone::Modal => 0,           // Modals always first
            FocusZone::CommandPalette => 1,  // Command palette next
            FocusZone::Main => 10,           // Main content
            FocusZone::ChatInput => 11,      // Chat input in main area
            FocusZone::Sidebar => 20,        // Sidebar
            FocusZone::FileExplorer => 21,   // File explorer
            FocusZone::TabBar => 30,         // Tab bar
            FocusZone::Settings => 40,       // Settings
            FocusZone::Terminal => 50,       // Terminal
            FocusZone::Custom(_) => 100,     // Custom zones last
        }
    }

    /// Get the display name for screen readers
    pub fn aria_label(&self) -> &str {
        match self {
            FocusZone::Main => "Main content area",
            FocusZone::Sidebar => "Sidebar navigation",
            FocusZone::TabBar => "Tab bar",
            FocusZone::ChatInput => "Chat input",
            FocusZone::Modal => "Dialog",
            FocusZone::CommandPalette => "Command palette",
            FocusZone::Settings => "Settings",
            FocusZone::FileExplorer => "File explorer",
            FocusZone::Terminal => "Terminal",
            FocusZone::Custom(name) => name,
        }
    }
}

/// Focus trap configuration for modals
#[derive(Debug, Clone)]
pub struct FocusTrap {
    /// The zone to trap focus in
    pub zone: FocusZone,
    /// Whether to return focus to previous element on close
    pub restore_focus: bool,
    /// First focusable element in the trap
    pub first_focusable: Option<String>,
    /// Last focusable element in the trap
    pub last_focusable: Option<String>,
}

impl FocusTrap {
    /// Create a new focus trap for a zone
    pub fn new(zone: FocusZone) -> Self {
        Self {
            zone,
            restore_focus: true,
            first_focusable: None,
            last_focusable: None,
        }
    }

    /// Set whether to restore focus when trap is released
    pub fn with_restore_focus(mut self, restore: bool) -> Self {
        self.restore_focus = restore;
        self
    }
}

/// Focusable element information
#[derive(Debug, Clone)]
pub struct FocusableElement {
    /// Unique identifier
    pub id: String,
    /// Focus zone this element belongs to
    pub zone: FocusZone,
    /// Tab index within the zone (lower = earlier)
    pub tab_index: i32,
    /// Whether this element is currently visible
    pub visible: bool,
    /// Whether this element is currently enabled
    pub enabled: bool,
    /// Accessible label for screen readers
    pub aria_label: Option<String>,
    /// Role for screen readers
    pub role: Option<String>,
}

impl FocusableElement {
    /// Create a new focusable element
    pub fn new(id: impl Into<String>, zone: FocusZone) -> Self {
        Self {
            id: id.into(),
            zone,
            tab_index: 0,
            visible: true,
            enabled: true,
            aria_label: None,
            role: None,
        }
    }

    /// Set the tab index
    pub fn with_tab_index(mut self, index: i32) -> Self {
        self.tab_index = index;
        self
    }

    /// Set the aria label
    pub fn with_aria_label(mut self, label: impl Into<String>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Set the role
    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    /// Check if this element can receive focus
    pub fn can_focus(&self) -> bool {
        self.visible && self.enabled && self.tab_index >= 0
    }
}
