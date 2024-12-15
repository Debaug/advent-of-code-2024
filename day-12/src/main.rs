use common::array2::Array2;
use manager::*;

mod manager;

fn main() {
    common::run_with_parser(|text| parse_input(&text), part1, part2);
}

fn parse_input(input: &str) -> Array2<u8> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_uppercase())
                .map(|c| c as u8 - b'A')
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
struct Plot<P> {
    inner: P,
    sub_region_index: usize,
}

struct Neighbors<'p, P> {
    left_plot: Option<&'p P>,
    above_plot: Option<&'p P>,
    edge_left: bool,
    edge_above: bool,
    edge_below: bool,
    edge_right: bool,
}

trait Region: Default + Merge {
    fn to_number(&self) -> u32;
}

fn process<R: Region, P: Default + Copy>(
    plants: &Array2<u8>,
    mut plot_region: impl FnMut(Neighbors<P>) -> (R, P),
) -> u32 {
    let mut region_manager: RegionManager<R> = RegionManager::new();
    let mut plots: Array2<Plot<P>> = Array2::from_default(plants.width(), plants.height());

    for (x, y, &plant) in plants.indexed_iter() {
        let left_plot = plots.get(x - 1, y);
        let above_plot = plots.get(x, y - 1);

        let left_plant = plants.get(x - 1, y).copied();
        let above_plant = plants.get(x, y - 1).copied();
        let right_plant = plants.get(x + 1, y).copied();
        let below_plant = plants.get(x, y + 1).copied();

        let edge_left = left_plant.is_none_or(|p| p != plant);
        let edge_above = above_plant.is_none_or(|p| p != plant);
        let edge_right = right_plant.is_none_or(|p| p != plant);
        let edge_below = below_plant.is_none_or(|p| p != plant);

        let neighbors = Neighbors {
            left_plot: left_plot.map(|plot| &plot.inner),
            above_plot: above_plot.map(|plot| &plot.inner),
            edge_left,
            edge_above,
            edge_right,
            edge_below,
        };
        let (plot_region, plot_data) = plot_region(neighbors);

        let left_sub_region_index = left_plot
            .filter(|_| !edge_left)
            .map(|plot| plot.sub_region_index);
        let above_sub_region_index = above_plot
            .filter(|_| !edge_above)
            .map(|plot| plot.sub_region_index);

        let sub_region_index = match (left_sub_region_index, above_sub_region_index) {
            (None, None) => region_manager.new_region_and_sub_region(),

            (Some(sub_region_index), None) | (None, Some(sub_region_index)) => sub_region_index,

            (Some(left_sub_region_index), Some(up_sub_region_index)) => {
                region_manager.merge(left_sub_region_index, up_sub_region_index)
            }
        };

        *plots.get_mut(x, y).unwrap() = Plot {
            inner: plot_data,
            sub_region_index,
        };

        region_manager
            .region_mut(sub_region_index)
            .merge(plot_region);
    }

    region_manager.regions().map(Region::to_number).sum()
}

#[derive(Debug, Clone, Copy, Default)]
struct Region1 {
    perimeter: u32,
    area: u32,
}

impl Merge for Region1 {
    fn merge(&mut self, other: Self) {
        self.perimeter += other.perimeter;
        self.area += other.area;
    }
}

impl Region for Region1 {
    fn to_number(&self) -> u32 {
        self.perimeter * self.area
    }
}

fn part1(plants: &Array2<u8>) -> u32 {
    process(plants, |neighbors| {
        let n_edges = [
            neighbors.edge_left,
            neighbors.edge_above,
            neighbors.edge_right,
            neighbors.edge_below,
        ]
        .into_iter()
        .filter(|edge| *edge)
        .count();
        let region = Region1 {
            perimeter: n_edges as u32,
            area: 1,
        };
        (region, ())
    })
}

#[derive(Debug, Clone, Copy, Default)]
struct Region2 {
    n_edges: u32,
    area: u32,
}

impl Merge for Region2 {
    fn merge(&mut self, other: Self) {
        self.n_edges += other.n_edges;
        self.area += other.area;
    }
}

impl Region for Region2 {
    fn to_number(&self) -> u32 {
        self.area * self.n_edges
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Plot2 {
    edge_left: bool,
    edge_right: bool,
    edge_above: bool,
    edge_below: bool,
}

fn part2(plants: &Array2<u8>) -> u32 {
    process(plants, |neighbors| -> (Region2, Plot2) {
        let mut n_new_edges = 0;

        let left_plot_same_plant = neighbors.left_plot.filter(|_| !neighbors.edge_left);

        if neighbors.edge_above
            && left_plot_same_plant.is_none_or(|left_plot| !left_plot.edge_above)
        {
            n_new_edges += 1;
        }
        if neighbors.edge_below
            && left_plot_same_plant.is_none_or(|left_plot| !left_plot.edge_below)
        {
            n_new_edges += 1;
        }

        let above_plot_same_plant = neighbors.above_plot.filter(|_| !neighbors.edge_above);

        if neighbors.edge_left
            && above_plot_same_plant.is_none_or(|above_plot| !above_plot.edge_left)
        {
            n_new_edges += 1;
        }
        if neighbors.edge_right
            && above_plot_same_plant.is_none_or(|above_plot| !above_plot.edge_right)
        {
            n_new_edges += 1;
        }

        (
            Region2 {
                n_edges: n_new_edges,
                area: 1,
            },
            Plot2 {
                edge_left: neighbors.edge_left,
                edge_above: neighbors.edge_above,
                edge_right: neighbors.edge_right,
                edge_below: neighbors.edge_below,
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const SMALL_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(SMALL_INPUT)), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(SMALL_INPUT)), 1206);
    }
}
