use egui::Ui;
use egui::collapsing_header::CollapsingState;

pub struct About
{
    table_of_contents: Tree,
    selected: String,
    article: Box<dyn Fn(&mut Ui)>,
}

impl About {
    fn create_table_of_contents() -> Tree {
        let mut tree = Tree::new("Table of Contents", vec![]);
        tree.add_child("Section 1", "Table of Contents");
        tree.add_child("Section 1.1", "Section 1");
        tree.add_child("Section 1.1.1", "Section 1.1");
        tree.add_child("Section 1.2", "Section 1");
        tree.add_child("Section 2", "Table of Contents");
        tree.add_child("Section 2.1", "Section 2");
        tree
    }
}

impl Default for About {
    fn default() -> Self {
        Self {
            table_of_contents: About::create_table_of_contents(),
            selected: "Table of Contents".to_string(),
            article: Box::new(|ui| {ui.heading("Table of Contents");}),
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
                let (show_root, default_depth) = (false, 1);
                self.selected = self.table_of_contents.ui(ui, show_root, default_depth, &self.selected);
            });
        // // Central Panel: Articles
        // egui::CentralPanel::default()
        //     .show(ctx, *self.article );
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


    pub fn add_child(&mut self, child: &str, parent: &str) {
        if &self.root == parent {
            self.children.push(Tree::new(child, vec![]))
        } else {
            self.children.iter_mut().for_each(|c| {c.add_child(child, parent)});
        }

    }

    pub fn new(root: &str, children: Vec<&str>) -> Self {
        Self {
            root: root.to_string(),
            children: children.iter().map(|c| Tree::new(c, vec![])).collect()
        }
    }

    pub fn ui(&self, ui: &mut Ui, show_root: bool, default_depth: u32, selected: &str) -> String {
        // Keep track of why tree button is selected
        let mut selected = selected.to_string();

        // If we don't want to show the root, we'll start the tree by iterating
        // over children
        let trees = match show_root {
            true  => vec![self],
            false => self.children.iter().collect(),
        };
        // Render each tree(s) UI
        trees.into_iter().enumerate().for_each(|(i, tree)| {
            let (depth, name, prefix) = (0, &tree.root, &vec![i as u32]);
            selected = tree.ui_recursive(ui, depth, name, default_depth, &selected, prefix);
        });

        // Return the tree button that is selected
        return selected
    }

    fn ui_recursive(&self, ui: &mut Ui, depth: u32, name: &str, default_depth: u32, selected: &str, prefix: &[u32]) -> String {

        // If the user selects a new tree button
        let mut selected = selected.to_string();

        // Configure a numeric prefix (ex. "1", "1.1", "2.3.1")
        let mut prefix = prefix.to_vec();
        if prefix.len() > 0 {
            let l = prefix.len() - 1;
            prefix[l] = prefix[l] + 1;
        }

        // Create a new collapsing element
        let state = CollapsingState::load_with_default_open(
            ui.ctx(), 
            ui.make_persistent_id(format!("tree_{name}")), 
            depth < default_depth
        );

        // THIS WORKS BELOW! BUT WE WANT: CUSTOM ICONS AND DEFAULT OPEN/CLOSE
        // Configure the header
        let header = state.show_header(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(prefix.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("."));
                let button = ui.selectable_label(selected == name, name);
                button.context_menu(|ui| {ui.label("You right clicked a header!");});
                if button.clicked() { selected = name.to_string();};
            })
        });
        // Configure the body
        let (collapse, _, _) = header.body(|ui| {
            self
                .children
                .iter()
                .enumerate()
                .for_each(|(i, c)| {
                    let c_prefix = prefix.clone().into_iter().chain([i as u32]).collect::<Vec<u32>>();
                    let (c_depth, c_name) = (depth + 1, &c.root);
                    selected = c.ui_recursive(ui, c_depth, c_name, default_depth, &selected, &c_prefix);
                })
        });
        collapse.context_menu(|ui| {ui.label("You right-clicked the collapse icon!");});

        return selected.to_string()
    }
}
