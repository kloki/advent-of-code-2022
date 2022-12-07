pub enum CMD {
    Back,
    Dir(String),
    Ls(String),
}

pub fn parse_command(input: &str) -> CMD {
    if input.starts_with("ls") | input.starts_with("$ ls") {
        CMD::Ls(input.split_once('\n').unwrap().1.to_string())
    } else if input == "cd .." {
        CMD::Back
    } else if input == "$ cd /" {
        CMD::Dir("/".to_string())
    } else {
        CMD::Dir(input.split_once(' ').unwrap().1.to_string())
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct File {
    name: String,
    size: u32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Folder {
    name: String,
    files: Vec<File>,
    subs: Vec<Folder>,
}

impl Folder {
    pub fn empty(name: String) -> Self {
        Folder {
            name,
            files: vec![],
            subs: vec![],
        }
    }
    pub fn size(&self) -> u32 {
        let mut size = self.files.iter().map(|f| f.size).sum::<u32>();
        size += self.subs.iter().map(|s| s.size()).sum::<u32>();
        size
    }
    pub fn collect_sizes(&self) -> Vec<u32> {
        self.subs
            .iter()
            .map(|s| s.collect_sizes())
            .fold(vec![self.size()], |mut acc, mut x| {
                acc.append(&mut x);
                acc
            })
    }
}

pub fn find_files(input: String) -> Vec<File> {
    input
        .split('\n')
        .map(|x| x.split_once(' ').unwrap())
        .filter(|s| s.0 != "dir")
        .map(|s| File {
            name: s.1.to_string(),
            size: s.0.parse().unwrap(),
        })
        .collect()
}
pub fn build_file_system(name: String, commands: &mut Vec<CMD>) -> Folder {
    let mut folder = Folder::empty(name);
    loop {
        match commands.pop() {
            Some(CMD::Ls(s)) => folder.files.append(&mut find_files(s)),
            Some(CMD::Dir(s)) => folder.subs.push(build_file_system(s, commands)),
            _ => return folder,
        }
    }
}
