// https://adventofcode.com/2025/day/5

fn main() {
    let sample = parse_input("sample.txt");
    let input  = parse_input("input.txt");

    println!("Sample part 1: {}", part1(&sample));
    println!("Sample part 2: {}", part2(&sample));
    println!("Part 1:        {}", part1(&input));
    println!("Part 2:        {}", part2(&input));
}

struct Input {
    ranges: Vec<(u64,u64)>,
    ingredient_ids: Vec<u64>
}

fn part1(input: &Input) -> usize {
    
    input.ingredient_ids
         .iter()
         .filter(|&&id| {
            input.ranges
                 .iter()
                 .any(|&(from, to)| from <= id && id <= to)
         })
         .count()
}

fn part2(input: &Input) -> u64 {

    let mut intervals: Vec<Interval> = vec![];

    for range in input.ranges.iter() {
        let interval = range.0 ..= range.1;

        // use our custom IntervalMerger iterator to merge this interval into the others
        // while keeping them all sorted and non-overlapping
        intervals = interval_merger(intervals.into_iter(), interval)
                        .collect();
    }

    intervals.into_iter()
             .map(|interval| interval.end() - interval.start() + 1)
             .sum::<u64>()
}


/* IntervalMerger Iterator */

// copied from my aoc-2022 day 15 - https://github.com/jasonincanada/aoc-2022/blob/main/days/day_15/src/main.rs#L120

type Interval = std::ops::RangeInclusive<u64>;

// our iterator maintains some mutable state to remember between next() calls
struct IntervalMerger<I: Iterator<Item=Interval>> {
    // the underlying iterator of Intervals. the intervals must be sorted by .start
    iter: I,

    // the new interval to add/merge into the outgoing stream of them
    new: Interval,

    // whenever we pull the next interval and it's non-overlapping to the right of
    // the interval we've been constructing, we suddenly have two on our hands: the
    // newly constructed one, and the next one that should come right after it.
    // but we can only return one Interval per call to next(), so here we queue
    // up the one we pulled too soon and it'll go out in the next call to next()
    queued: Option<Interval>,

    // true once we've returned the new interval
    returned: bool
}

impl<I> Iterator for IntervalMerger<I>
where
    I: Iterator<Item=Interval>
{
    type Item = Interval;

    fn next(&mut self) -> Option<Interval> {

        // if we queued up an interval in the last call, return it now
        if self.queued.is_some() {
            return self.queued.take()
        }

        // we've already returned the new interval so there's nothing left
        // to do but pass through the rest of them
        if self.returned { return self.iter.next() }

        // pull the next interval from the underlying iterator
        let next = self.iter.next();

        // none left, but we haven't returned the new one yet, so do it now
        if next.is_none() {
            self.returned = true;
            return Some(self.new.clone());
        }

        let next = next.unwrap();

        // pass through all the intervals non-overlapping to the left
        if next.end() < self.new.start() { return Some(next) }

        if self.new.end() < next.start() {
            self.queued = Some(next);
            self.returned = true;
            return Some(self.new.clone());
        }

        // the fun part, where we really get to benefit from the Iterator pattern.
        // we can keep calling next() on the underlying iterator even though we're
        // only planning to emit one Interval from this "outer" call of next()

        // XXX..XXXX...   underlying
        // ........XXXX   new
        let start = self.new.start().min(next.start());
        let end   = self.new.end()  .max(next.end());
        let mut new: Interval = Interval::new(*start, *end);

        loop {
            let next = self.iter.next();

            if next.is_none() {
                self.returned = true;
                return Some(new)
            }

            let next = next.unwrap();

            if new.end() < next.start() {
                self.queued = Some(next);
                self.returned = true;
                return Some(new);
            }

            // we don't need to consider the start here because the intervals were sorted,
            // meaning later intervals have larger starts than the one we're constructing
            let end = new.end().max(next.end());
            new = Interval::new(*start, *end);
        }
    }
}

// wrap an existing iterator of Intervals (already sorted by .start) to construct
// an iterator that merges a new Interval at the right spot in the underlying one
fn interval_merger<I>(iter: I, new: Interval) -> IntervalMerger<I>
where
    I: Iterator<Item=Interval>
{
    IntervalMerger {
        iter,
        new,
        queued: None,
        returned: false
    }
}


/* Parsing */

// ChatGPT 5.1
fn parse_input(filename: &str) -> Input {
    // Assume file always exists and is valid
    let text = std::fs::read_to_string(filename).unwrap();

    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut parsing_ranges = true;

    for line in text.lines() {
        let trimmed = line.trim();

        // Blank line transitions to ingredient ID section
        if trimmed.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            // Parse "a-b"
            let (a, b) = trimmed.split_once('-').unwrap();
            ranges.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            // Parse ingredient ID
            ids.push(trimmed.parse().unwrap());
        }
    }

    Input {
        ranges,
        ingredient_ids: ids,
    }
}


/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let input = parse_input("sample.txt");
        assert_eq!(3, part1(&input))
    }

    #[test]
    fn test_sample_part2() {
        let input = parse_input("sample.txt");
        assert_eq!(14, part2(&input))
    }

    #[test]
    fn test_part1() {
        let input = parse_input("input.txt");
        assert_eq!(840, part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = parse_input("input.txt");
        assert_eq!(359913027576322, part2(&input))
    }

}
