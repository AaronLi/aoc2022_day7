#[derive(Debug)]
pub enum DirectoryMember {
    File(String, usize),
    Folder(String)
}

#[derive(Debug)]
pub enum CdParam {
    In(String),
    Out,
    Root
}

#[derive(Debug)]
pub enum Command {
    Cd(CdParam),
    Ls(Vec<DirectoryMember>)
}

pub fn parse(s: &str) -> Vec<Command> {
    let lines = s.split("\r\n").collect::<Vec<&str>>();
    let mut commands = Vec::new();
    let mut current_line = 0;

    while let Some(l) = lines.get(current_line) {
        match &l[0..4] {
            "$ ls" => {
                let mut members = Vec::new();
                let mut ls_pointer = current_line + 1;
                while let Some(s) = lines.get(ls_pointer) {
                    if s.starts_with('$') {
                        break;
                    }
                    let params = s.split(' ').collect::<Vec<&str>>();
                    match params[0].starts_with(|c: char|c.is_ascii_digit()) {
                        true => {
                            // file
                            members.push(
                                DirectoryMember::File(params[1].to_string(), params[0].parse::<usize>().unwrap())
                            );
                        }
                        false => {
                            //directory
                            members.push(
                                DirectoryMember::Folder(params[1].to_string())
                            );
                        }
                    }
                    ls_pointer += 1;
                }
                current_line = ls_pointer;
                commands.push(Command::Ls(members));
            },
            "$ cd" => {
                let target = lines[current_line].split(' ').last().unwrap();
                commands.push(Command::Cd(match target {
                    ".." => CdParam::Out,
                    "/" => CdParam::Root,
                    _ => CdParam::In(target.to_string())
                }));
                current_line += 1;
            },
            _ => panic!("Unexpected line: {}", lines[current_line])
        }
    }

    commands
}
