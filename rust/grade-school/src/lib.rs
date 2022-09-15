use std::collections::HashMap;

pub struct School {
    students: HashMap<u32, Vec<String>>,
}

impl Default for School {
    fn default() -> Self {
        School::new()
    }
}

impl School {
    pub fn new() -> School {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student_name: &str) {
        self.students
            .entry(grade)
            .or_default()
            .push(student_name.to_string())
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.students.keys().copied().collect::<Vec<u32>>();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self.students.get(&grade).unwrap_or(&vec![]).to_vec();
        students.sort();
        students
    }
}
