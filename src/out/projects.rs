use crate::projects::Projects;

pub struct ProjectsOut;
impl ProjectsOut {
    pub fn list(projects: &Projects) {
        println!("Available projects:");
        projects.get().iter().for_each(|p| println!(" - {}", p));
    }
}
