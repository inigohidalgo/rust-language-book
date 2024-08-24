fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(number) => {
                let number_incr = number + 1;
                let number_str = number_incr.to_string();
                println!("Int cell incremented: {}", number_str)
            },
            SpreadsheetCell::Text(text) => {
                println!("Cell contains string: {}", text)
            }
            _ => (),
        }

    }

}
