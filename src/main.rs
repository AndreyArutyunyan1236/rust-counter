use iced::widget::{column, button, text};
use iced::{Element, Settings, Sandbox};
use crate::Message::Increment;

fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    money: i32,
}

#[derive(Copy, Clone, Debug)]
enum Message {
    Increment,
    Score,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A cool Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Increment => self.value += 1,
            Message::Score => {
                self.money = self.money + self.value;
                self.value = 0;
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            text(self.value).size(50),
            button("Нажимай! Нажимай! Нажимай!").on_press(Increment).height(50).width(250),
            button("Ввывести деньги.").on_press(Message::Score),
            {
                if self.money == 0 {
                text("У вас нету денег. Это легко исправить! Нажимайте на первую кнопку!")
                } else {
                text(format!("Ваш баланс на карте: {} рублей!", self.money))
                }
            }
        ]
            .spacing(30)
            .padding(10)
            .into()
    }
}

