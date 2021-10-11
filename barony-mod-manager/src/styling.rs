use iced::{
    button, checkbox, container,
    pick_list::{self, Menu},
    text_input, Color,
};

pub struct GeneralUiStyles;

impl container::StyleSheet for GeneralUiStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(26, 26, 26).into(),
            ..container::Style::default()
        }
    }
}

impl text_input::StyleSheet for GeneralUiStyles {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.1)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(230, 230, 230)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgba8(255, 255, 255, 0.1)
    }
}

impl checkbox::StyleSheet for GeneralUiStyles {
    fn active(&self, _: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color::from_rgb8(35, 35, 35).into(),
            checkmark_color: Color::from_rgb8(220, 220, 220),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }

    fn hovered(&self, _: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color::from_rgb8(35, 35, 35).into(),
            checkmark_color: Color::from_rgb8(220, 220, 220),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
        }
    }
}

impl button::StyleSheet for GeneralUiStyles {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(23, 23, 23).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(27, 27, 27).into()),
            text_color: Color::from_rgb8(230, 230, 230),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            ..button::Style::default()
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Color::from_rgb8(20, 20, 20).into()),
            text_color: Color::from_rgb8(210, 210, 210),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            ..button::Style::default()
        }
    }
}

impl pick_list::StyleSheet for GeneralUiStyles {
    fn menu(&self) -> Menu {
        Menu {
            text_color: Color::from_rgb8(220, 220, 220),
            background: Color::from_rgb8(23, 23, 23).into(),
            border_width: 1.5,
            border_color: Color::from_rgb8(35, 35, 35),
            selected_background: Color::from_rgb8(35, 35, 35).into(),
            selected_text_color: Color::from_rgb8(220, 220, 220),
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::from_rgb8(220, 220, 220),
            background: Color::from_rgb8(23, 23, 23).into(),
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(35, 35, 35),
            icon_size: 0.5,
            // placeholder_color: Default::default(),
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Color::from_rgb8(30, 30, 30).into(),
            text_color: Color::from_rgb8(220, 220, 220),
            border_width: 1.0,
            border_radius: 1.0,
            border_color: Color::from_rgb8(45, 45, 45),
            icon_size: 0.5,
            // placeholder_color: Default::default(),
        }
    }
}

pub struct ModCardUiStyles;

impl container::StyleSheet for ModCardUiStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb8(22, 22, 22).into(),
            border_width: 3.0,
            border_color: Color::from_rgb8(80, 80, 80),
            border_radius: 4.0,
            ..container::Style::default()
        }
    }
}
