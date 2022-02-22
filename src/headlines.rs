const PADDING: f32 = 25.0;
const WHITE: Color32 = Color32::from_rgb(255,255,255);
const CYAN: Color32 = Color32::from_rgb(0,255,255);

use eframe::{
             egui::{Separator, TopBottomPanel, RichText, Label, Hyperlink, Layout, Color32, Vec2, FontDefinitions, FontData, FontFamily}
            };

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

pub struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl Headlines {
    pub fn new() -> Headlines {
        
        let iter = (0..20).map(|a: i32| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}",a),
        });
        
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &eframe::egui::CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert("MesloLGS".to_string(), FontData::from_static(include_bytes!("../MesloLGS_NF_Regular.ttf")));

        font_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, 20.));
        
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "MesloLGS".to_string());

        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles{
            let title = format!("> {}", a.title);
            ui.colored_label(WHITE, title);
            ui.add_space(PADDING);
        
            let desc: Label = Label::new(RichText::new(&a.desc).text_style(eframe::egui::TextStyle::Button));
            ui.add(desc);
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);

            ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), 24.0), Layout::right_to_left(), |ui| {
                ui.add(Hyperlink::from_label_and_url(RichText::new("read more ^"), &a.url)); 
            });
            ui.add(Separator::default());
        }
    }

    pub fn render_footer(&self, ctx: &eframe::egui::CtxRef) {
        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.add(Label::new(RichText::new("API source: newsapi.org").monospace()));
                ui.add(Hyperlink::from_label_and_url(RichText::new("Made with egui").monospace(), "https://github.com/emilk/egui"));
                ui.add(Hyperlink::from_label_and_url(RichText::new("Update of CreativCoder/Headlines").monospace(), "https:://github.com/creativcoder/headlines"));
                ui.add_space(10.);
            });
        });
    }

    pub fn render_header(&self, ui: &mut eframe::egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("headlines");
        });

        ui.add_space(PADDING);
        let sep = Separator::default().spacing(20.);
        ui.add(sep);
    }
}
