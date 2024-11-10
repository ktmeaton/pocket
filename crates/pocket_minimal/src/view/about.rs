use egui::{collapsing_header::CollapsingState, CollapsingHeader, Ui};

pub struct About {
    table_of_contents: Tree,
    selected: String
}

impl About {
    fn create_table_of_contents() -> Tree {
        let subtree1 = Tree::new("Section 1", vec!["Section 1.2"]);
        let subtree2 = Tree::new("Section 2", vec![]);
        Tree {
            root: "Table of Contents".to_string(),
            children: vec![subtree1, subtree2]
        }
    }
}

impl Default for About {
    fn default() -> Self {
        Self {
            table_of_contents: About::create_table_of_contents(),
            selected: "Table of Contents".to_string()
        }
    }
}

impl eframe::App for About {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left Panel: Table of Contents
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(4.0);
                ui.vertical_centered(|ui| {
                    ui.heading("Table of Contents");
                });
                ui.separator();
                self.table_of_contents.ui(ui, &self.selected);
            });
        // Central Panel: Articles
        // egui::CentralPanel::default()
        //     .show(ctx, |ui| { self.about.ui(ui); });
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tree {
    pub root: String,
    pub children: Vec<Tree>,
}

impl Default for Tree {
    fn default() -> Self {
        Self::new("Root", Vec::new())
    }
}

impl Tree {

    pub fn new(root: &str, children: Vec<&str>) -> Self {
        Self {
            root: root.to_string(),
            children: children.iter().map(|c| Tree::new(c, vec![])).collect()
        }
    }
    pub fn ui<'s>(&self, ui: &mut Ui, selected: &'s str) -> &'s str{
        let selected = self.ui_recursive(ui, 0, &self.root, selected);
        return selected
    }

    fn ui_recursive<'s>(&self, ui: &mut Ui, depth: usize, name: &str, selected: &'s str) -> &'s str {

        let response = match self.children.len() > 0 {
            true => {
                let id = ui.make_persistent_id(format!("tree_{name}"));
                CollapsingState::load_with_default_open(ui.ctx(), id, false)
                    .show_header(ui, |ui| {
                        ui.selectable_label(selected == name, name);
                    })
                    .body(|ui| {
                        self
                            .children
                            .iter()
                            .for_each(|child| { child.ui_recursive(ui, depth + 1, &child.root, selected);});
                    })
                    .0
            },
            false => ui.selectable_label(selected == name, name)
        };
        response.context_menu(|ui| {ui.label("Shown on right-clicks");});

        match response.clicked(){
            true => return name,
            false => return selected,
        }
    }
}
