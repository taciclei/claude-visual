//! User profile hover card component

use super::utils::format_count;
use gpui::prelude::*;
use gpui::*;

/// User profile hover card content
#[derive(IntoElement)]
pub struct UserHoverCard {
    name: SharedString,
    username: SharedString,
    avatar: Option<SharedString>,
    bio: Option<SharedString>,
    followers: Option<u32>,
    following: Option<u32>,
    verified: bool,
    online: bool,
}

impl UserHoverCard {
    pub fn new(name: impl Into<SharedString>, username: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            username: username.into(),
            avatar: None,
            bio: None,
            followers: None,
            following: None,
            verified: false,
            online: false,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<SharedString>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    pub fn bio(mut self, bio: impl Into<SharedString>) -> Self {
        self.bio = Some(bio.into());
        self
    }

    pub fn followers(mut self, count: u32) -> Self {
        self.followers = Some(count);
        self
    }

    pub fn following(mut self, count: u32) -> Self {
        self.following = Some(count);
        self
    }

    pub fn verified(mut self, verified: bool) -> Self {
        self.verified = verified;
        self
    }

    pub fn online(mut self, online: bool) -> Self {
        self.online = online;
        self
    }
}

impl RenderOnce for UserHoverCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .min_w(px(280.0))
            // Header with avatar
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_3()
                    .child(
                        div()
                            .relative()
                            .child(
                                div()
                                    .w(px(48.0))
                                    .h(px(48.0))
                                    .rounded_full()
                                    .bg(Hsla {
                                        h: 0.58,
                                        s: 0.5,
                                        l: 0.4,
                                        a: 1.0,
                                    })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(18.0))
                                    .text_color(Hsla {
                                        h: 0.0,
                                        s: 0.0,
                                        l: 1.0,
                                        a: 1.0,
                                    })
                                    .child(self.name.chars().next().unwrap_or('?').to_string()),
                            )
                            .when(self.online, |d| {
                                d.child(
                                    div()
                                        .absolute()
                                        .bottom_0()
                                        .right_0()
                                        .w(px(12.0))
                                        .h(px(12.0))
                                        .rounded_full()
                                        .bg(Hsla {
                                            h: 0.38,
                                            s: 0.7,
                                            l: 0.5,
                                            a: 1.0,
                                        })
                                        .border_2()
                                        .border_color(Hsla {
                                            h: 0.0,
                                            s: 0.0,
                                            l: 0.15,
                                            a: 1.0,
                                        }),
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_size(px(15.0))
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(Hsla {
                                                h: 0.0,
                                                s: 0.0,
                                                l: 0.95,
                                                a: 1.0,
                                            })
                                            .child(self.name.clone()),
                                    )
                                    .when(self.verified, |d| {
                                        d.child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(Hsla {
                                                    h: 0.58,
                                                    s: 0.7,
                                                    l: 0.6,
                                                    a: 1.0,
                                                })
                                                .child("✓"),
                                        )
                                    }),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(Hsla {
                                        h: 0.0,
                                        s: 0.0,
                                        l: 0.5,
                                        a: 1.0,
                                    })
                                    .child(format!("@{}", self.username)),
                            ),
                    ),
            )
            // Bio
            .when_some(self.bio, |d, bio| {
                d.child(
                    div()
                        .text_size(px(13.0))
                        .text_color(Hsla {
                            h: 0.0,
                            s: 0.0,
                            l: 0.7,
                            a: 1.0,
                        })
                        .line_height(px(18.0))
                        .child(bio),
                )
            })
            // Stats
            .when(self.followers.is_some() || self.following.is_some(), |d| {
                d.child(
                    div()
                        .flex()
                        .gap_4()
                        .text_size(px(13.0))
                        .when_some(self.followers, |d, count| {
                            d.child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(Hsla {
                                                h: 0.0,
                                                s: 0.0,
                                                l: 0.9,
                                                a: 1.0,
                                            })
                                            .child(format_count(count)),
                                    )
                                    .child(
                                        div()
                                            .text_color(Hsla {
                                                h: 0.0,
                                                s: 0.0,
                                                l: 0.5,
                                                a: 1.0,
                                            })
                                            .child("Followers"),
                                    ),
                            )
                        })
                        .when_some(self.following, |d, count| {
                            d.child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .text_color(Hsla {
                                                h: 0.0,
                                                s: 0.0,
                                                l: 0.9,
                                                a: 1.0,
                                            })
                                            .child(format_count(count)),
                                    )
                                    .child(
                                        div()
                                            .text_color(Hsla {
                                                h: 0.0,
                                                s: 0.0,
                                                l: 0.5,
                                                a: 1.0,
                                            })
                                            .child("Following"),
                                    ),
                            )
                        }),
                )
            })
    }
}
