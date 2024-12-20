use std::iter;

#[derive(Debug, Clone)]
pub struct Array2<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Default for Array2<T> {
    fn default() -> Self {
        Self {
            data: vec![],
            width: 0,
            height: 0,
        }
    }
}

#[derive(Debug)]
pub struct UnevenRows;

impl<T> Array2<T> {
    pub fn from_rows<Row>(rows: impl IntoIterator<Item = Row>) -> Result<Self, UnevenRows>
    where
        Row: IntoIterator<Item = T>,
    {
        let mut rows = rows.into_iter();
        let Some(first_row) = rows.next() else {
            return Ok(Self {
                data: vec![],
                width: 0,
                height: 0,
            });
        };
        let mut data: Vec<_> = first_row.into_iter().collect();
        let width = data.len();
        let mut height = 1;

        for row in rows {
            data.extend(row);
            height += 1;
            if data.len() != width * height {
                return Err(UnevenRows);
            }
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }

    pub fn from_element(elt: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![elt; width * height],
            width,
            height,
        }
    }

    pub fn from_default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let data: Vec<_> = iter::repeat_with(Default::default)
            .take(width * height)
            .collect();
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from_fn(width: usize, height: usize, mut func: impl FnMut(isize, isize) -> T) -> Self {
        if width == 0 || height == 0 {
            return Default::default();
        }

        let iwidth = isize::try_from(width).expect("`Array2` was too big");
        let iheight = isize::try_from(height).expect("`Array2` was too big");
        let data = (0..iheight)
            .flat_map(|y| (0..iwidth).map(move |x| (x, y)))
            .map(|(x, y)| func(x, y))
            .collect();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn pos_to_index(&self, x: isize, y: isize) -> Option<usize> {
        let x: usize = x.try_into().ok().filter(|&x| x < self.width)?;
        let y: usize = y.try_into().ok()?;
        Some(x + y * self.width)
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.data.get(self.pos_to_index(x, y)?)
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let idx = self.pos_to_index(x, y)?;
        self.data.get_mut(idx)
    }

    pub fn raw_data(&self) -> &[T] {
        &self.data
    }

    pub fn raw_data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn into_raw_data(self) -> Vec<T> {
        self.data
    }

    fn index_iter<U>(
        iter: impl Iterator<Item = U>,
        width: usize,
    ) -> impl Iterator<Item = (isize, isize, U)> {
        let mut x = 0;
        let mut y = 0;
        let width: isize = width
            .try_into()
            .expect("`Array2` width didn't fit in an `isize`");
        iter.map(move |elt| {
            let eltx = x;
            let elty = y;
            x += 1;
            if x >= width {
                x = 0;
                y += 1;
            }
            (eltx, elty, elt)
        })
    }

    pub fn indexed_iter(&self) -> impl Iterator<Item = (isize, isize, &T)> {
        Self::index_iter(self.data.iter(), self.width)
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (isize, isize, &mut T)> {
        Self::index_iter(self.data.iter_mut(), self.width)
    }

    pub fn into_indexed_iter(self) -> impl Iterator<Item = (isize, isize, T)> {
        Self::index_iter(self.data.into_iter(), self.width)
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.chunks_mut(self.width)
    }
}

impl<T, Row> FromIterator<Row> for Array2<T>
where
    Row: IntoIterator<Item = T>,
{
    fn from_iter<Rows: IntoIterator<Item = Row>>(rows: Rows) -> Self {
        Self::from_rows(rows).expect("failed to build `Array2` from rows")
    }
}

impl<T> AsRef<Array2<T>> for Array2<T> {
    fn as_ref(&self) -> &Array2<T> {
        self
    }
}
