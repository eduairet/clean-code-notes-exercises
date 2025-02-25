use clean_code_notes_exercises::exercises::exercise_06::*;

#[cfg(test)]
mod exercise_06_test {
    use super::*;

    #[test]
    fn test_single_responsibility_principle() {
        let member_empty = Member::new("", "");
        assert_eq!(member_empty.login("test@test.com", "password"), false);
        let mut member = Member::new("test@test.com", "password");
        assert_eq!(
            member.signup("test@test.com", "password"),
            "Member test@test.com signed up"
        );
        assert_eq!(
            member.assign_role("admin"),
            "Member test@test.com assigned role admin"
        );
        assert_eq!(member.login("test@test.com", "password"), true);
    }

    #[test]
    fn test_open_closed_principle() {
        let web_printer = WebPrinter;
        assert_eq!(
            web_printer.print("Some data"),
            Ok("Web Printer Printing: Some data".to_string())
        );
        assert_eq!(web_printer.print(""), Err("Data is empty"));

        let pdf_printer = PDFPrinter;
        assert_eq!(
            pdf_printer.print("Some data"),
            Ok("PDF Printer Printing: Some data".to_string())
        );
        assert_eq!(pdf_printer.print(""), Err("Data is empty"));

        let page_printer = PagePrinter;
        assert_eq!(
            page_printer.print("Some data"),
            Ok("Page Printer Printing: Some data".to_string())
        );
        assert_eq!(page_printer.print(""), Err("Data is empty"));
    }

    #[test]
    fn test_liskov_substitution_principle() {
        let eagle = Eagle::new("Royal Eagle");
        assert_eq!(eagle.name(), "Royal Eagle");
        assert_eq!(eagle.fly(), ());
        assert_eq!(eagle.dive(), ());

        let penguin = Penguin::new("Emperor Penguin");
        assert_eq!(penguin.name(), "Emperor Penguin");
    }

    #[test]
    fn test_interface_segregation_principle() {
        let thriatlete = Triathlete;
        assert_eq!(thriatlete.swim(), ());
        assert_eq!(thriatlete.cycle(), ());
        assert_eq!(thriatlete.run(), ());

        let runner_only = RunnerOnly;
        assert_eq!(runner_only.run(), ());

        let soccer_player = SoccerPlayer;
        assert_eq!(soccer_player.play_as_team(), ());
    }

    #[test]
    fn test_dependency_inversion_principle() {
        let sql_database = SQLDatabase::new("http://localhost:8080");
        assert_eq!(sql_database.uri, "http://localhost:8080");
        assert_eq!(sql_database.connect(), ());
        assert_eq!(sql_database.store_data("Some data"), ());

        let in_memory_database = InMemoryDatabase;
        assert_eq!(in_memory_database.store_data("Some data"), ());
    }
}
