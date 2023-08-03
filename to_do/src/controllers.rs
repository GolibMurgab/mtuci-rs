use druid::{widget::Controller, Env, UpdateCtx, Widget, EventCtx, Event, Code};

use crate::data::*;

pub struct TodoItemController;

impl<W: Widget<TodoItem>> Controller<TodoItem, W> for TodoItemController {
    fn update(&mut self, child: &mut W, ctx: &mut UpdateCtx, old_data: &TodoItem, data: &TodoItem, env: &Env,) {
        if old_data.done != data.done {
            ctx.submit_command(SAVE);
        }
        child.update(ctx, old_data, data, env);
    }
}

pub struct Enter;

impl<W: Widget<Todo>> Controller<Todo, W> for Enter {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut Todo, env: &Env) {
        if let Event::KeyUp(key) = event {
            if key.code == Code::Enter {
                Todo::add_item(ctx, data, env)
            }
        }
        child.event(ctx, event, data, env)
    }
}