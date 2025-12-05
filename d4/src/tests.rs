use super::*;

const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[test]
fn test_task1() -> Result<()> {
    let input = parse(INPUT)?;
    let neighbor_cache = precompute_neighbors(input.len(), input[0].len());
    assert_eq!(task1(&input, &neighbor_cache).count(), 13);
    Ok(())
}

#[test]
fn test_task2() -> Result<()> {
    let input = parse(INPUT)?;
    let neighbor_cache = precompute_neighbors(input.len(), input[0].len());
    assert_eq!(task2(input, &neighbor_cache), 43);
    Ok(())
}

#[test]
fn test_main() -> Result<()> {
    main()
}
