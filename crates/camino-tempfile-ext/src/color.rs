// Copyright (c) The camino-tempfile Contributors
// Adapted from assert_fs: Copyright (c) The assert_fs Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Palette {
    inner: Inner,
}

impl Palette {
    #[cfg(feature = "color")]
    pub(crate) fn color() -> Self {
        Self {
            inner: Inner::Color {
                key: anstyle::AnsiColor::Blue.on_default() | anstyle::Effects::BOLD,
                value: anstyle::AnsiColor::Yellow.on_default() | anstyle::Effects::BOLD,
            },
        }
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn color() -> Self {
        Self::default()
    }

    pub(crate) fn key<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.inner.key_style())
    }

    pub(crate) fn value<D: std::fmt::Display>(self, display: D) -> Styled<D> {
        Styled::new(display, self.inner.value_style())
    }
}

#[derive(Copy, Clone, Debug, Default)]
enum Inner {
    #[cfg(feature = "color")]
    Color {
        key: anstyle::Style,
        value: anstyle::Style,
    },
    #[default]
    Plain,
}

impl Inner {
    fn key_style(&self) -> StyledInner {
        match self {
            #[cfg(feature = "color")]
            Inner::Color { key, .. } => StyledInner::Color(*key),
            Inner::Plain => StyledInner::Plain,
        }
    }

    fn value_style(&self) -> StyledInner {
        match self {
            #[cfg(feature = "color")]
            Inner::Color { value, .. } => StyledInner::Color(*value),
            Inner::Plain => StyledInner::Plain,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Styled<D> {
    display: D,
    inner: StyledInner,
}

#[derive(Debug)]
enum StyledInner {
    #[cfg(feature = "color")]
    Color(anstyle::Style),
    Plain,
}

impl<D: std::fmt::Display> Styled<D> {
    fn new(display: D, inner: StyledInner) -> Self {
        Self { display, inner }
    }
}

impl<D: std::fmt::Display> std::fmt::Display for Styled<D> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            match self.inner {
                #[cfg(feature = "color")]
                StyledInner::Color(style) => {
                    write!(f, "{}", style.render())?;
                    self.display.fmt(f)?;
                    write!(f, "{}", style.render_reset())?;
                    Ok(())
                }
                StyledInner::Plain => self.display.fmt(f),
            }
        } else {
            self.display.fmt(f)
        }
    }
}
