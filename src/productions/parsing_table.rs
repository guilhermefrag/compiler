#[derive(Clone, Debug)]
pub struct ParseTable {
    pub non_terminal: i32,
    pub terminal: i32,
    pub production: i32,
}

pub type ParsingTable = Vec<ParseTable>;

pub fn find(parsing_table: ParsingTable, non_terminal: i32, terminal: i32) -> Option<ParseTable> {
    for parse_table in parsing_table {
        if parse_table.non_terminal == non_terminal && parse_table.terminal == terminal {
            return Some(parse_table);
        }
    }

    None
}

pub fn get_parsing_table() -> ParsingTable {
    let mut parsing_table: ParsingTable = Vec::new();
    
    parsing_table.push(ParseTable {
        non_terminal: 49,
        terminal: 2,
        production: 1,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 2,
        production: 3,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 3,
        production: 3,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 9,
        production: 2,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 13,
        production: 3,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 14,
        production: 3,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 18,
        production: 3,
    });
    parsing_table.push(ParseTable {
        non_terminal: 50,
        terminal: 24,
        production: 3,
    });
    parsing_table.push(ParseTable {
        non_terminal: 51,
        terminal: 2,
        production: 13,
    });
    parsing_table.push(ParseTable {
        non_terminal: 51,
        terminal: 3,
        production: 13,
    });
    parsing_table.push(ParseTable {
        non_terminal: 51,
        terminal: 13,
        production: 13,
    });
    parsing_table.push(ParseTable {
        non_terminal: 51,
        terminal: 14,
        production: 14,
    });
    parsing_table.push(ParseTable {
        non_terminal: 51,
        terminal: 18,
        production: 13,
    });
    parsing_table.push(ParseTable {
        non_terminal: 51,
        terminal: 24,
        production: 13,
    });
    parsing_table.push(ParseTable {
        non_terminal: 52,
        terminal: 14,
        production: 31,
    });
    parsing_table.push(ParseTable {
        non_terminal: 53,
        terminal: 39,
        production: 4,
    });
    parsing_table.push(ParseTable {
        non_terminal: 53,
        terminal: 41,
        production: 5,
    });
    parsing_table.push(ParseTable {
        non_terminal: 54,
        terminal: 3,
        production: 8,
    });
    parsing_table.push(ParseTable {
        non_terminal: 54,
        terminal: 13,
        production: 6,
    });
    parsing_table.push(ParseTable {
        non_terminal: 54,
        terminal: 18,
        production: 7,
    });
    parsing_table.push(ParseTable {
        non_terminal: 54,
        terminal: 24,
        production: 9,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 2,
        production: 11,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 3,
        production: 11,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 9,
        production: 10,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 13,
        production: 11,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 14,
        production: 11,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 18,
        production: 11,
    });
    parsing_table.push(ParseTable {
        non_terminal: 55,
        terminal: 24,
        production: 11,
    });
    parsing_table.push(ParseTable {
        non_terminal: 57,
        terminal: 9,
        production: 12,
    });
    parsing_table.push(ParseTable {
        non_terminal: 58,
        terminal: 2,
        production: 16,
    });
    parsing_table.push(ParseTable {
        non_terminal: 58,
        terminal: 3,
        production: 19,
    });
    parsing_table.push(ParseTable {
        non_terminal: 58,
        terminal: 13,
        production: 15,
    });
    parsing_table.push(ParseTable {
        non_terminal: 58,
        terminal: 18,
        production: 18,
    });
    parsing_table.push(ParseTable {
        non_terminal: 58,
        terminal: 24,
        production: 17,
    });
    parsing_table.push(ParseTable {
        non_terminal: 59,
        terminal: 36,
        production: 26,
    });
    parsing_table.push(ParseTable {
        non_terminal: 59,
        terminal: 44,
        production: 27,
    });
    parsing_table.push(ParseTable {
        non_terminal: 60,
        terminal: 5,
        production: 20,
    });
    parsing_table.push(ParseTable {
        non_terminal: 60,
        terminal: 6,
        production: 21,
    });
    parsing_table.push(ParseTable {
        non_terminal: 60,
        terminal: 8,
        production: 23,
    });
        parsing_table.push(ParseTable {
        non_terminal: 60,
        terminal: 9,
        production: 22,
    });
    parsing_table.push(ParseTable {
        non_terminal: 60,
        terminal: 10,
        production: 24,
    });
    parsing_table.push(ParseTable {
        non_terminal: 60,
        terminal: 43,
        production: 25,
    });
    parsing_table.push(ParseTable {
        non_terminal: 61,
        terminal: 3,
        production: 28,
    });
    parsing_table.push(ParseTable {
        non_terminal: 61,
        terminal: 13,
        production: 28,
    });
    parsing_table.push(ParseTable {
        non_terminal: 61,
        terminal: 18,
        production: 28,
    });
    parsing_table.push(ParseTable {
        non_terminal: 61,
        terminal: 24,
        production: 28,
    });
    parsing_table.push(ParseTable {
        non_terminal: 62,
        terminal: 38,
        production: 29,
    });
    parsing_table.push(ParseTable {
        non_terminal: 62,
        terminal: 43,
        production: 30,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 1,
        production: 40,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 8,
        production: 36,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 9,
        production: 34,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 10,
        production: 35,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 15,
        production: 39,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 17,
        production: 41,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 21,
        production: 42,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 22,
        production: 44,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 23,
        production: 43,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 25,
        production: 38,
    });
    parsing_table.push(ParseTable {
        non_terminal: 63,
        terminal: 38,
        production: 37,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 1,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 8,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 9,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 10,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 15,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 17,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 19,
        production: 32,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 21,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 22,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 23,
        production: 33,
    });
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 25,
        production: 33,
    });
    
    parsing_table.push(ParseTable {
        non_terminal: 64,
        terminal: 36,
        production: 32,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 5,
        production: 73,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 6,
        production: 73,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 8,
        production: 73,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 9,
        production: 73,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 10,
        production: 73,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 25,
        production: 74,
    });
    parsing_table.push(ParseTable {
        non_terminal: 65,
        terminal: 44,
        production: 73,
    });
    parsing_table.push(ParseTable {
        non_terminal: 66,
        terminal: 38,
        production: 45,
    });
    parsing_table.push(ParseTable {
        non_terminal: 66,
        terminal: 43,
        production: 45,
    });
    parsing_table.push(ParseTable {
        non_terminal: 66,
        terminal: 44,
        production: 46,
    });
    parsing_table.push(ParseTable {
        non_terminal: 67,
        terminal: 5,
        production: 49,
    });
    parsing_table.push(ParseTable {
        non_terminal: 67,
        terminal: 6,
        production: 51,
    });
    parsing_table.push(ParseTable {
        non_terminal: 67,
        terminal: 8,
        production: 52,
    });
    parsing_table.push(ParseTable {
        non_terminal: 67,
        terminal: 9,
        production: 53,
    });
    parsing_table.push(ParseTable {
        non_terminal: 67,
        terminal: 10,
        production: 50,
    });
    parsing_table.push(ParseTable {
        non_terminal: 68,
        terminal: 41,
        production: 48,
    });
    parsing_table.push(ParseTable {
        non_terminal: 68,
        terminal: 43,
        production: 47,
    });
    parsing_table.push(ParseTable {
        non_terminal: 69,
        terminal: 27,
        production: 59,
    });
    parsing_table.push(ParseTable {
        non_terminal: 69,
        terminal: 28,
        production: 58,
    });
    parsing_table.push(ParseTable {
        non_terminal: 69,
        terminal: 29,
        production: 56,
    });
    parsing_table.push(ParseTable {
        non_terminal: 69,
        terminal: 31,
        production: 61,
    });
    parsing_table.push(ParseTable {
        non_terminal: 69,
        terminal: 33,
        production: 60,
    });
    parsing_table.push(ParseTable {
        non_terminal: 69,
        terminal: 46,
        production: 57,
    });
    parsing_table.push(ParseTable {
        non_terminal: 70,
        terminal: 20,
        production: 54,
    });
    parsing_table.push(ParseTable {
        non_terminal: 70,
        terminal: 38,
        production: 55,
    });
    parsing_table.push(ParseTable {
        non_terminal: 71,
        terminal: 5,
        production: 62,
    });
    parsing_table.push(ParseTable {
        non_terminal: 71,
        terminal: 6,
        production: 63,
    });
    parsing_table.push(ParseTable {
        non_terminal: 71,
        terminal: 8,
        production: 65,
    });
    parsing_table.push(ParseTable {
        non_terminal: 71,
        terminal: 9,
        production: 66,
    });
    parsing_table.push(ParseTable {
        non_terminal: 71,
        terminal: 10,
        production: 64,
    });
    parsing_table.push(ParseTable {
        non_terminal: 72,
        terminal: 34,
        production: 67,
    });
    parsing_table.push(ParseTable {
        non_terminal: 72,
        terminal: 47,
        production: 68,
    });
    parsing_table.push(ParseTable {
        non_terminal: 73,
        terminal: 32,
        production: 70,
    });
    parsing_table.push(ParseTable {
        non_terminal: 73,
        terminal: 38,
        production: 69,
    });
    parsing_table.push(ParseTable {
        non_terminal: 74,
        terminal: 32,
        production: 71,
    });
    parsing_table.push(ParseTable {
        non_terminal: 74,
        terminal: 38,
        production: 71,
    });
    parsing_table.push(ParseTable {
        non_terminal: 74,
        terminal: 41,
        production: 72,
    });
    parsing_table.push(ParseTable {
        non_terminal: 77,
        terminal: 5,
        production: 78,
    });
    parsing_table.push(ParseTable {
        non_terminal: 77,
        terminal: 6,
        production: 78,
    });
    parsing_table.push(ParseTable {
        non_terminal: 77,
        terminal: 8,
        production: 78,
    });
    parsing_table.push(ParseTable {
        non_terminal: 77,
        terminal: 9,
        production: 78,
    });
    parsing_table.push(ParseTable {
        non_terminal: 77,
        terminal: 10,
        production: 78,
    });
    parsing_table.push(ParseTable {
        non_terminal: 77,
        terminal: 44,
        production: 78,
    });
    parsing_table.push(ParseTable {
        non_terminal: 78,
        terminal: 35,
        production: 77,
    });
    parsing_table.push(ParseTable {
        non_terminal: 78,
        terminal: 38,
        production: 77,
    });
    parsing_table.push(ParseTable {
        non_terminal: 78,
        terminal: 43,
        production: 77,
    });
    parsing_table.push(ParseTable {
        non_terminal: 79,
        terminal: 5,
        production: 82,
    });
    parsing_table.push(ParseTable {
        non_terminal: 79,
        terminal: 6,
        production: 83,
    });
    parsing_table.push(ParseTable {
        non_terminal: 79,
        terminal: 8,
        production: 86,
    });
    parsing_table.push(ParseTable {
        non_terminal: 79,
        terminal: 9,
        production: 84,
    });
    parsing_table.push(ParseTable {
        non_terminal: 79,
        terminal: 10,
        production: 85,
    });
    parsing_table.push(ParseTable {
        non_terminal: 79,
        terminal: 44,
        production: 87,
    });
    parsing_table.push(ParseTable {
        non_terminal: 80,
        terminal: 35,
        production: 79,
    });
    parsing_table.push(ParseTable {
        non_terminal: 80,
        terminal: 38,
        production: 79,
    });
    parsing_table.push(ParseTable {
        non_terminal: 80,
        terminal: 40,
        production: 81,
    });
    parsing_table.push(ParseTable {
        non_terminal: 80,
        terminal: 42,
        production: 80,
    });
    parsing_table.push(ParseTable {
        non_terminal: 80,
        terminal: 43,
        production: 79,
    });

    parsing_table.push(ParseTable {
        non_terminal: 80,
        terminal: 48,
        production: 79,
    });

    parsing_table
}
