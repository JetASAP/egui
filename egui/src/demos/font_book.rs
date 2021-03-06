use crate::*;

pub struct FontBook {
    standard: bool,
    emojis: bool,
    filter: String,
    text_style: TextStyle,
}

impl Default for FontBook {
    fn default() -> Self {
        Self {
            standard: false,
            emojis: true,
            filter: Default::default(),
            text_style: TextStyle::Button,
        }
    }
}

impl FontBook {
    fn characters_ui(&self, ui: &mut Ui, characters: &[(u32, char, &str)]) {
        for &(_, chr, name) in characters {
            if self.filter.is_empty()
                || name.contains(&self.filter)
                || self.filter == chr.to_string()
            {
                let button = Button::new(chr).text_style(self.text_style).frame(false);

                let tooltip_ui = |ui: &mut Ui| {
                    ui.add(Label::new(chr).text_style(self.text_style));
                    ui.label(format!("{}\nU+{:X}\n\nClick to copy", name, chr as u32));
                };

                if ui.add(button).on_hover_ui(tooltip_ui).clicked {
                    ui.output().copied_text = chr.to_string();
                }
            }
        }
    }
}

impl demos::Demo for FontBook {
    fn name(&self) -> &str {
        "🔤 Font Book"
    }

    fn show(&mut self, ctx: &crate::CtxRef, open: &mut bool) {
        Window::new(self.name()).open(open).show(ctx, |ui| {
            use demos::View;
            self.ui(ui);
        });
    }
}

impl demos::View for FontBook {
    fn ui(&mut self, ui: &mut Ui) {
        use crate::demos::font_contents_emoji::FULL_EMOJI_LIST;
        use crate::demos::font_contents_ubuntu::UBUNTU_FONT_CHARACTERS;

        ui.label(format!(
            "Egui supports {} standard characters and {} emojis.\nClick on a character to copy it.",
            UBUNTU_FONT_CHARACTERS.len(),
            FULL_EMOJI_LIST.len(),
        ));

        ui.separator();

        combo_box_with_label(ui, "Text style", format!("{:?}", self.text_style), |ui| {
            for style in TextStyle::all() {
                ui.selectable_value(&mut self.text_style, style, format!("{:?}", style));
            }
        });

        ui.horizontal(|ui| {
            ui.label("Show:");
            ui.checkbox(&mut self.standard, "Standard");
            ui.checkbox(&mut self.emojis, "Emojis");
        });

        ui.horizontal(|ui| {
            ui.label("Filter:");
            ui.text_edit_singleline(&mut self.filter);
            self.filter = self.filter.to_lowercase();
            if ui.button("ｘ").clicked {
                self.filter.clear();
            }
        });

        ui.separator();

        crate::ScrollArea::auto_sized().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::splat(2.0);

                if self.standard {
                    self.characters_ui(ui, UBUNTU_FONT_CHARACTERS);
                }
                if self.emojis {
                    self.characters_ui(ui, FULL_EMOJI_LIST);
                }
            });
        });
    }
}
