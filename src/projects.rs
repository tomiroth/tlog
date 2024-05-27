use crate::dir::Dir;

pub struct Projects<'a>{
    dir: &'a Dir,
    inner: Vec<String>,
}


impl<'a> Projects<'a>{
    pub fn get (&'a self) -> &'a Vec<String>{
        &self.inner
    }
    pub fn new (dir: &'a Dir) -> Self{
        let projects_string = dir.read_project_file();
        let split = projects_string.split(',').map(|s| s.to_string()).collect();
        Self { inner: split, dir}
    }

    pub fn add(mut self, name: &str){
        self.inner.push(name.to_owned());
        let data = self.inner.iter()
            .filter(|p|{
                !p.is_empty()
            }).
            map(|p| p.as_str())
            .collect::<Vec<_>>().join(",");
        let _ = Dir::write(&self.dir.projects_file, &data);
    }
}
