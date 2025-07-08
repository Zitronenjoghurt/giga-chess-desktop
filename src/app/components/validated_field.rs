use egui::{Response, Ui};

pub struct ValidatedField<'a> {
    label: &'a str,
    value: &'a mut String,
    validator: Box<dyn Fn(&str) -> bool>,
    error_message: &'a str,
    is_password: bool,
    label_width: f32,
}

pub struct ValidatedFieldResponse {
    pub is_valid: bool,
    pub response: Response,
}

impl<'a> ValidatedField<'a> {
    pub fn new(label: &'a str, value: &'a mut String) -> Self {
        Self {
            label,
            value,
            validator: Box::new(|_| true),
            error_message: "",
            is_password: false,
            label_width: 50.0,
        }
    }

    pub fn validator(mut self, f: impl Fn(&str) -> bool + 'static) -> Self {
        self.validator = Box::new(f);
        self
    }

    pub fn error_message(mut self, message: &'a str) -> Self {
        self.error_message = message;
        self
    }

    pub fn password(mut self) -> Self {
        self.is_password = true;
        self
    }

    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }

    pub fn show(self, ui: &mut Ui) -> ValidatedFieldResponse {
        let is_valid = (self.validator)(self.value);
        let response = ui
            .horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.set_min_width(self.label_width);
                    ui.label(self.label);
                });

                let text_edit_response = ui.text_edit_singleline(self.value);
                if text_edit_response.has_focus() && !is_valid {
                    text_edit_response.show_tooltip_text(self.error_message);
                }

                if is_valid {
                    ui.label("✅");
                } else {
                    ui.colored_label(ui.visuals().error_fg_color, "❌");
                }

                text_edit_response
            })
            .inner;

        ValidatedFieldResponse { is_valid, response }
    }
}
