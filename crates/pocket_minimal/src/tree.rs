use egui::{CollapsingHeader, Ui};

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tree {
    pub children: Vec<Tree>,
    pub root: String,
    pub selected: Option<String>,
}

impl Default for Tree {
    fn default() -> Self {
        Self::new("root")
    }
}

impl Tree {

    pub fn new(root: &str) -> Self {
        Self { children: Vec::new(), root: root.to_string(), selected: None }
    }
    pub fn ui(&self, ui: &mut Ui, show_root: bool) {
        if show_root {
            self.ui_impl(ui, 0, &self.root);
        } else {
            self.children_ui(ui, 1)
        }
    }

    fn ui_impl(&self, ui: &mut Ui, depth: usize, name: &str) {
        // If this node has children made collapsible header
        if self.children.len() > 0 {
            CollapsingHeader::new(name)
                .default_open(depth < 1)
                .show(ui, |ui| {
                    self.children_ui(ui, depth)
                });
        } 
        // Otherwise just indent it for alignment
        else {
            ui.indent(name, |ui| {
                if ui
                    .selectable_label(self.selected == Some(name.to_string()), name)
                    .clicked() {}
            });
        }
    }

    fn children_ui(&self, ui: &mut Ui, depth: usize) {
        self
            .children
            .iter()
            .for_each(|tree| { tree.ui_impl(ui, depth + 1, &tree.root);});
    }

    pub fn toc() -> Self {
        let mut section_1  = Tree::new("Section 1");
        let section_1_1    = Tree::new("Section 1.1");
        section_1.children = vec![section_1_1];
        let section_2      = Tree::new("Section 2");
        let mut tree = Tree::new("Table of Contents");
        tree.children      = vec![section_1, section_2];
        tree
    }
}