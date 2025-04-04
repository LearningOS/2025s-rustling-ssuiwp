// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

enum GradeType {
    Numerically,
    Alphabetical,
}
pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub grade_type: GradeType,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name,
            &self.student_age,
            &self.grade()
        )
    }
    fn grade(&self) -> String {
        match &self.grade_type {
            GradeType::Alphabetical => match &self.grade {
                ..2.5 => "A+".into(),
                2.5..5.5 => "F".into(),
                _ => "F".into(),
            },
            GradeType::Numerically => self.grade.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
            grade_type: GradeType::Numerically,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
            grade_type: GradeType::Alphabetical,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
