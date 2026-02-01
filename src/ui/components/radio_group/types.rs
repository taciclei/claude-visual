use gpui::*;

/// Radio size options
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RadioSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl RadioSize {
    pub(crate) fn outer_size(&self) -> f32 {
        match self {
            RadioSize::Small => 16.0,
            RadioSize::Medium => 20.0,
            RadioSize::Large => 24.0,
        }
    }

    pub(crate) fn inner_size(&self) -> f32 {
        match self {
            RadioSize::Small => 8.0,
            RadioSize::Medium => 10.0,
            RadioSize::Large => 12.0,
        }
    }

    pub(crate) fn font_size(&self) -> f32 {
        match self {
            RadioSize::Small => 13.0,
            RadioSize::Medium => 14.0,
            RadioSize::Large => 16.0,
        }
    }
}

/// Radio option for RadioGroup
#[derive(Clone)]
pub struct RadioOption {
    pub value: SharedString,
    pub label: SharedString,
    pub description: Option<SharedString>,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Radio group orientation
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RadioGroupOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Radio card option for RadioCardGroup
#[derive(Clone)]
pub struct RadioCardOption {
    pub value: SharedString,
    pub title: SharedString,
    pub description: Option<SharedString>,
    pub icon: Option<SharedString>,
    pub price: Option<SharedString>,
    pub badge: Option<SharedString>,
    pub disabled: bool,
}

impl RadioCardOption {
    pub fn new(value: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            description: None,
            icon: None,
            price: None,
            badge: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn price(mut self, price: impl Into<SharedString>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
