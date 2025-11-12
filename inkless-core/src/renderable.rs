use core::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{canvas::Canvas, tag::Tag};

pub trait RenderContext {
    fn handle_error<'b>(&'b mut self, error: &'b dyn core::error::Error);
}

pub trait Renderable<T: Tag> {
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut dyn Canvas<T>,
    ) -> Result<(), RenderableError>;
}

impl<'a, T: Tag, O: Renderable<T>> Renderable<T> for &'a O {
    fn render_into<'buffer_reference>(
        &self,
        canvas: &mut dyn Canvas<T>,
    ) -> Result<(), RenderableError> {
        (**self).render_into(canvas)
    }
}

pub struct RenderableError {
    provider: for<'a> fn(&'a mut dyn RenderContext) -> (),
}

impl RenderableError {
    pub fn of(provider: for<'a> fn(&'a mut dyn RenderContext) -> ()) -> Self {
        Self { provider }
    }
}

impl From<for<'a> fn(&'a mut dyn RenderContext) -> ()> for RenderableError {
    fn from(provider: for<'a> fn(&'a mut dyn RenderContext) -> ()) -> Self {
        Self { provider }
    }
}

impl Debug for RenderableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        struct DebugHandler<'a, 'b> {
            formatter: &'a mut core::fmt::Formatter<'b>,
            failure: Option<core::fmt::Error>,
        }

        impl<'a, 'b> RenderContext for DebugHandler<'a, 'b> {
            fn handle_error<'c>(&'c mut self, error: &'c dyn core::error::Error) {
                if let Err(e) = Debug::fmt(error, self.formatter) {
                    self.failure = Some(e)
                }
            }
        }

        let mut handler = DebugHandler {
            formatter: f,
            failure: None,
        };

        (self.provider)(&mut handler);

        if let Some(e) = handler.failure {
            return Err(e);
        }

        Ok(())
    }
}

impl Display for RenderableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        struct DisplayHandler<'a, 'b> {
            formatter: &'a mut core::fmt::Formatter<'b>,
            failure: Option<core::fmt::Error>,
        }

        impl<'a, 'b> RenderContext for DisplayHandler<'a, 'b> {
            fn handle_error<'c>(&'c mut self, error: &'c dyn core::error::Error) {
                if let Err(e) = Display::fmt(error, self.formatter) {
                    self.failure = Some(e)
                }
            }
        }

        let mut handler = DisplayHandler {
            formatter: f,
            failure: None,
        };

        (self.provider)(&mut handler);

        if let Some(e) = handler.failure {
            return Err(e);
        }

        Ok(())
    }
}

impl Error for RenderableError {}
