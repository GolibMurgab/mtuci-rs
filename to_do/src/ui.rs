use druid::{
    widget::{Button, Checkbox, Flex, Label, List, TextBox},
    Widget, WidgetExt,
    Color,
};

use crate::{controllers::*, data::*};

//Фунция создает поле вввода и добавляет кнопки добавить и убрать выполненное
fn todo_textbox() -> impl Widget<Todo> {
    let new_todo_textbox = TextBox::new()
        .with_placeholder("Добавьте новую задачу")
        .expand_width()
        .lens(Todo::new_todo)
        .controller(Enter);

    let add_todo_button = Button::new("+").on_click(Todo::add_item);
    let remove_done_tasks = Button::new("-").on_click(Todo::remove_done);

    Flex::row()
        .with_flex_child(new_todo_textbox, 1.)
        .with_child(add_todo_button)
        .with_child(remove_done_tasks)
}

//Функция создает задачу и кнопки удалить и выполнено
fn todo_item() -> impl Widget<TodoItem> {
    let checkbox = Checkbox::new("").lens(TodoItem::done);
    let label = Label::raw().
        with_text_color(Color::rgb8(0, 0, 0)).with_text_size(20.0)
        .lens(TodoItem::text);
    let emoji = char::from_u32(0x1F5D1).expect("Неверная кодировка").to_string();
    let delete_button = Button::new(emoji).on_click(TodoItem::delete_item);

    Flex::row()
        .with_child(checkbox)
        .with_child(delete_button)
        .with_flex_child(label, 1.).controller(TodoItemController)
}


//Функция строит основной ui
pub fn build_ui() -> impl Widget<Todo> {
    let todos = List::new(todo_item).lens(Todo::todos).scroll().vertical();
    Flex::column()
        .with_child(todo_textbox())
        .with_flex_child(todos, 1.0)
}



