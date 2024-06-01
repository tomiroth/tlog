use crate::dir::Dir;

pub struct Projects<'a> {
    dir: &'a Dir,
    inner: Vec<String>,
}

impl<'a> Projects<'a> {
    pub fn get(&'a self) -> &'a Vec<String> {
        &self.inner
    }
    pub fn new(dir: &'a Dir) -> Self {
        let projects_string = dir.read_project_file();
        let split = projects_string
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        Self { inner: split, dir }
    }

    pub fn add(mut self, name: &str) {
        let project_exists = self.exists(name);

        if project_exists {
            println!("Project already exists");
        } else {
            self.inner.push(name.to_owned());
            self.save();
        }
    }

    fn save(&self) {
        let mut data = self
            .inner
            .iter()
            .filter(|p| !p.is_empty())
            .map(|p| p.as_str())
            .collect::<Vec<_>>();
        data.sort();
        let data = data.join(",");
        let _ = Dir::write(&self.dir.projects_file, &data);
    }

    pub fn exists(&self, name: &str) -> bool {
        self.inner.iter().position(|p| p == name).is_some()
    }

    pub fn delete(mut self, name: &str) {
        let pos = self.inner.iter().position(|p| p == name);
        if let Some(pos) = pos {
            self.inner.remove(pos);
            self.save();
        }
    }
}
