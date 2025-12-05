use super::*;
use test_case::test_case;

const INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[test]
fn test_task1() -> Result<()> {
    let input = parse(INPUT)?;
    assert_eq!(task1(&input), 3);
    Ok(())
}

#[test]
fn test_task2() -> Result<()> {
    let input = parse(INPUT)?;
    assert_eq!(task2(input), 14);
    Ok(())
}

// === Ranges around the start (3) ===
#[test_case("3-10\n1-2\n\n3", 10 ; "around_start_before_gap")]
#[test_case("3-10\n2-3\n\n3", 9 ; "around_start_adjacent_overlap")]
#[test_case("3-10\n1-3\n\n3", 10 ; "around_start_overlap")]
#[test_case("3-10\n3-5\n\n3", 8 ; "around_start_contained")]
// === Ranges around the end (10) ===
#[test_case("3-10\n8-10\n\n3", 8 ; "around_end_contained")]
#[test_case("3-10\n10-11\n\n3", 9 ; "around_end_adjacent_overlap")]
#[test_case("3-10\n10-12\n\n3", 10 ; "around_end_overlap")]
#[test_case("3-10\n11-12\n\n3", 10 ; "around_end_after_gap")]
// === Fully contained ranges ===
#[test_case("3-10\n5-7\n\n3", 8 ; "contained_multi_point")]
#[test_case("3-10\n5-5\n\n3", 8 ; "contained_single_point")]
#[test_case("3-10\n3-10\n\n3", 8 ; "contained_exact_duplicate")]
// === Overlapping ranges ===
#[test_case("3-10\n2-4\n\n3", 9 ; "overlap_starts_before")]
#[test_case("3-10\n9-11\n\n3", 9 ; "overlap_ends_after")]
#[test_case("3-10\n1-12\n\n3", 12 ; "overlap_encompasses")]
fn test_single_extra_range(input: &str, expected: usize) -> Result<()> {
    let input = parse(input)?;
    assert_eq!(task2(input), expected);
    Ok(())
}

// === Multiple ranges (3+) ===
#[test_case("1-2\n5-6\n9-10\n\n1", 6 ; "three_non_overlapping_gaps")]
#[test_case("1-3\n4-6\n7-9\n\n1", 9 ; "three_adjacent")]
#[test_case("1-5\n3-8\n6-10\n\n1", 10 ; "three_overlapping")]
#[test_case("1-10\n3-5\n7-12\n\n1", 12 ; "mix_contained_extending")]
#[test_case("1-20\n5-7\n10-12\n15-17\n\n1", 20 ; "multiple_small_contained")]
#[test_case("5-10\n\n1", 6 ; "single_range")]
fn test_multiple_ranges(input: &str, expected: usize) -> Result<()> {
    let input = parse(input)?;
    assert_eq!(task2(input), expected);
    Ok(())
}

// === Complex overlapping scenarios ===
#[test_case("1-5\n4-8\n7-11\n10-14\n\n1", 14 ; "cascading_overlaps")]
#[test_case("1-100\n10-20\n30-40\n50-60\n70-80\n\n1", 100 ; "large_with_contained")]
#[test_case("1-3\n2-4\n3-5\n4-6\n5-7\n\n1", 7 ; "many_small_connecting")]
#[test_case("5-10\n5-10\n5-10\n\n1", 6 ; "duplicate_ranges")]
#[test_case("1-3\n4-6\n7-9\n\n1", 9 ; "truly_adjacent")]
fn test_complex_overlaps(input: &str, expected: usize) -> Result<()> {
    let input = parse(input)?;
    assert_eq!(task2(input), expected);
    Ok(())
}
// ASCII Art Reference for test_single_extra_range (Fixed range: 3-10)
//
//      1 |2 |3 |4 |5 |6 |7 |8 |9 |10|11|12
// 3-10:      [======================]
// 1-2: [====]                                         → 10 (before, gap)
// 2-3:    [====]                                      → 9  (adjacent/overlap at 3)
// 1-3: [=======]                                      → 10 (overlap at 3)
// 3-5:       [=======]                                → 8  (starts at 3, contained)
// 8-10:                     [=======]                 → 8  (ends at 10, contained)
// 10-11:                           [====]             → 9  (adjacent/overlap at 10)
// 10-12:                           [=======]          → 10 (overlap at 10)
// 11-12:                              [====]          → 10 (after, gap)
// 5-7:            [=======]                           → 8  (multi-point contained)
// 5-5:            [=]                                 → 8  (single point contained)
// 3-10:      [======================]                 → 8  (exact duplicate)
// 2-4:    [=======]                                   → 9  (starts before, ends inside)
// 9-11:                        [=======]              → 9  (starts inside, ends after)
// 1-12: [==================================]          → 12 (completely encompasses)

// ASCII Art Reference for test_multiple_ranges
//
//      1 |2 |3 |4 |5 |6 |7 |8 |9 |10|11|12|13|14|15|16|17|18|19|20
// 1-2, 5-6, 9-10: [==] [==]    [==]                  → 6  (non-overlapping)
// 1-3, 4-6, 7-9:  [===][===][===]                    → 9  (adjacent)
// 1-5, 3-8, 6-10: [=====][======][=====]             → 10 (overlapping)
// 1-10, 3-5, 7-12:[==========][===][======]          → 12 (contained + extending)
// 1-20 + contained ranges                            → 20 (all contained)
// 5-10:           [======]                           → 6  (single range)

// ASCII Art Reference for test_complex_overlaps
//
//      1 |2 |3 |4 |5 |6 |7 |8 |9 |10|11|12|13|14
// 1-5, 4-8, 7-11, 10-14: [====][====][====][====]    → 14 (cascading)
// 1-100 + 10-20, 30-40, 50-60, 70-80                 → 100 (large + contained)
// 1-3, 2-4, 3-5, 4-6, 5-7: [==][==][==][==][==]      → 7  (connecting)
// 5-10, 5-10, 5-10: [======] (3x)                    → 6  (duplicates)
// 1-3, 4-6, 7-9: [===][===][===]                     → 9  (adjacent)

#[test]
fn test_main() -> Result<()> {
    main()
}
