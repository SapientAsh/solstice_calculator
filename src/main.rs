/*
    ----This is table of values I used to generate these equations, (source: https://en.wikipedia.org/wiki/Equinox)----
    ┌──────┬─────┬─────┬─────┬─────┐
    │      │ Mar │ Jun │ Sep │ Dec │
    ├──────┼─────┼─────┼─────┼─────┤
    │ 2018 │ 20  │ 21  │ 23  │ 21  │
    │ 2019 │ 20  │ 21  │ 23  │ 22  │
    │ 2020 │ 20  │ 20  │ 22  │ 21  │
    │ 2021 │ 20  │ 21  │ 22  │ 21  │
    │ 2022 │ 20  │ 21  │ 23  │ 21  │
    │ 2023 │ 20  │ 21  │ 23  │ 22  │
    │ 2024 │ 20  │ 20  │ 22  │ 21  │
    │ 2025 │ 20  │ 21  │ 22  │ 21  │
    │ 2026 │ 20  │ 21  │ 23  │ 21  │
    │ 2027 │ 20  │ 21  │ 23  │ 22  │
    │ 2028 │ 20  │ 20  │ 22  │ 21  │
    └──────┴─────┴─────┴─────┴─────┘
*/


fn main() {    
    for year in 2018..=2028 {
        let june = match year % 4 == 0 {
            false => 21,
            true => 20,
        };

        let september = match (year / 2) % 2 == 1 {
            false => 22,
            true => 23,
        };

        let december = match year % 4 == 3 {
            false => 21,
            true => 22,
        };

        println!("{year} - March 20, June {june}, September {september}, December {december}")
    }
}
