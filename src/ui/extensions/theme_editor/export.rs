//! Export and save methods for ThemeEditor

use gpui::*;

use crate::ui::extensions::theme_editor::core::ThemeEditor;
use crate::ui::extensions::theme_editor::types::ThemeEditorEvent;
use crate::ui::extensions::theme_editor::helpers::hsla_to_hex;

impl ThemeEditor {
    /// Apply theme for live preview
    pub(crate) fn apply_preview(&mut self, cx: &mut Context<Self>) {
        self.app_state.theme.update(cx, |theme, _cx| {
            *theme = self.editing_theme.clone();
        });
        cx.emit(ThemeEditorEvent::PreviewApplied);
    }

    /// Save theme
    pub(crate) fn save(&mut self, cx: &mut Context<Self>) {
        self.editing_theme.name = self.theme_name.clone();

        // Export to JSON format
        let _json = self.export_to_json();

        // Save to theme loader (in memory for now)
        // In a real implementation, this would save to disk

        self.has_changes = false;
        cx.emit(ThemeEditorEvent::Saved(self.theme_name.clone()));
        cx.notify();
    }

    /// Export theme to Zed-compatible JSON
    pub(crate) fn export_to_json(&self) -> String {
        let theme = &self.editing_theme;

        format!(
            r#"{{
  "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
  "name": "{}",
  "author": "Claude Visual User",
  "themes": [
    {{
      "name": "{}",
      "appearance": "{}",
      "style": {{
        "background": "{}",
        "element.background": "{}",
        "element.hover": "{}",
        "border": "{}",
        "text": "{}",
        "text.muted": "{}",
        "text.accent": "{}",
        "status.success": "{}",
        "status.warning": "{}",
        "status.error": "{}",
        "status.info": "{}",
        "border.focused": "{}",
        "element.selected": "{}",
        "syntax": {{
          "keyword": {{ "color": "{}" }},
          "string": {{ "color": "{}" }},
          "number": {{ "color": "{}" }},
          "comment": {{ "color": "{}" }},
          "function": {{ "color": "{}" }},
          "variable": {{ "color": "{}" }},
          "constant": {{ "color": "{}" }},
          "type": {{ "color": "{}" }},
          "operator": {{ "color": "{}" }},
          "punctuation": {{ "color": "{}" }}
        }}
      }}
    }}
  ]
}}"#,
            theme.name,
            theme.name,
            if theme.is_dark { "dark" } else { "light" },
            hsla_to_hex(theme.colors.background),
            hsla_to_hex(theme.colors.surface),
            hsla_to_hex(theme.colors.surface_hover),
            hsla_to_hex(theme.colors.border),
            hsla_to_hex(theme.colors.text),
            hsla_to_hex(theme.colors.text_muted),
            hsla_to_hex(theme.colors.accent),
            hsla_to_hex(theme.colors.success),
            hsla_to_hex(theme.colors.warning),
            hsla_to_hex(theme.colors.error),
            hsla_to_hex(theme.colors.info),
            hsla_to_hex(theme.colors.focus_ring),
            hsla_to_hex(theme.colors.selection),
            hsla_to_hex(theme.syntax.keyword),
            hsla_to_hex(theme.syntax.string),
            hsla_to_hex(theme.syntax.number),
            hsla_to_hex(theme.syntax.comment),
            hsla_to_hex(theme.syntax.function),
            hsla_to_hex(theme.syntax.variable),
            hsla_to_hex(theme.syntax.constant),
            hsla_to_hex(theme.syntax.type_name),
            hsla_to_hex(theme.syntax.operator),
            hsla_to_hex(theme.syntax.punctuation),
        )
    }
}
