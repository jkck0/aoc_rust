pub struct Present(u32, u32, u32);

impl Present {
    fn wrapping_paper(&self) -> u32 {
        let faces = [self.0 * self.1, self.0 * self.2, self.1 * self.2];

        faces.iter().map(|f| f * 2).sum::<u32>() + faces.iter().min().unwrap()
    }

    fn ribbon(&self) -> u32 {
        let mut perimeters = vec![
            2 * (self.0 + self.1),
            2 * (self.0 + self.2),
            2 * (self.1 + self.2),
        ];
        perimeters.sort_unstable();
        
        let bow = self.0 * self.1 * self.2;
        perimeters[0] + bow
    }
}

pub fn parse(data: &str) -> Vec<Present> {
    data.lines()
        .map(|l| {
            let mut dimensions = l.split("x").map(|x| x.parse::<u32>().unwrap());

            Present(
                dimensions.next().unwrap(),
                dimensions.next().unwrap(),
                dimensions.next().unwrap(),
            )
        })
        .collect()
}

pub fn part1(input: &[Present]) -> u32 {
    input.iter().map(|p| p.wrapping_paper()).sum()
}

pub fn part2(input: &[Present]) -> u32 {
    input.iter().map(|p| p.ribbon()).sum()
}
