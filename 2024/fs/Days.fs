module Days

let dayRunners day =
    match day with
    | 1 -> (Day01.part1, Day01.part2)
    | _ -> System.NotImplementedException "" |> raise

let dayTests day =
    match day with
    | 1 -> (Day01.part1Tests, Day01.part2Tests)
    | _ -> System.NotImplementedException "" |> raise

let MAX_DAYS = 1
