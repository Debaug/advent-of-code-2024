pub trait Merge {
    fn merge(&mut self, other: Self);
}

#[derive(Debug, Clone)]
pub struct RegionManager<T> {
    regions: Vec<Option<RegionEntry<T>>>,
    freed_regions: Vec<usize>,
    sub_regions: Vec<SubRegions>,
}

#[derive(Debug, Clone, Copy)]
pub struct SubRegions {
    region_index: usize,
}

#[derive(Debug, Clone)]
struct RegionEntry<T> {
    region: T,
    sub_regions: Vec<usize>,
}

impl<T> RegionManager<T> {
    pub fn new() -> Self {
        Self {
            regions: vec![],
            freed_regions: vec![],
            sub_regions: vec![],
        }
    }

    pub fn new_region_and_sub_region(&mut self) -> usize
    where
        T: Default,
    {
        let new_sub_region_index = self.sub_regions.len();
        let entry = Some(RegionEntry {
            region: T::default(),
            sub_regions: vec![new_sub_region_index],
        });
        let region_index = match self.freed_regions.pop() {
            Some(index) => {
                self.regions[index] = entry;
                index
            }
            None => {
                self.regions.push(entry);
                self.regions.len() - 1
            }
        };
        self.sub_regions.push(SubRegions { region_index });
        self.sub_regions.len() - 1
    }

    pub fn merge(&mut self, to_sub_region_index: usize, from_sub_region_index: usize) -> usize
    where
        T: Merge,
    {
        let from_region_index = self.sub_regions[from_sub_region_index].region_index;
        let to_region_index = self.sub_regions[to_sub_region_index].region_index;

        if from_region_index == to_region_index {
            return to_sub_region_index;
        }

        self.freed_regions.push(from_region_index);
        let from_region = self.regions[from_region_index].take().unwrap();

        let to_region_index = self.sub_regions[to_sub_region_index].region_index;
        let to_region = self.regions[to_region_index].as_mut().unwrap();

        to_region
            .sub_regions
            .extend_from_slice(&from_region.sub_regions);
        to_region.region.merge(from_region.region);

        for sub_region in from_region.sub_regions {
            self.sub_regions[sub_region].region_index = to_region_index;
        }

        to_sub_region_index
    }

    pub fn region_mut(&mut self, sub_region_index: usize) -> &mut T {
        &mut self.regions[self.sub_regions[sub_region_index].region_index]
            .as_mut()
            .unwrap()
            .region
    }

    pub fn regions(&self) -> impl Iterator<Item = &T> {
        self.regions
            .iter()
            .filter_map(Option::as_ref)
            .map(|entry| &entry.region)
    }
}
