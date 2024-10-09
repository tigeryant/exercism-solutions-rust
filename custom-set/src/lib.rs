#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    elements: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: std::clone::Clone + std::cmp::PartialEq + std::cmp::Ord + std::fmt::Debug,
{
    pub fn new(input: &[T]) -> Self {
        let mut elements = input.to_vec();
        elements.sort();
        elements.dedup();
        CustomSet {
            elements
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.elements.push(element);
        self.elements.sort();
        self.elements.dedup();
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements
            .iter()
            .all(|element| other.elements.contains(element))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        !self
            .elements
            .iter()
            .any(|element| other.elements.contains(element))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let intersection = self
            .elements
            .clone()
            .into_iter()
            .filter(|element| other.elements.contains(element))
            .collect();

        CustomSet {
            elements: intersection
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut difference: Vec<T> = self
        .elements
        .clone()
        .into_iter()
        .filter(|element| !other.elements.contains(element))
        .collect();

        let mut only_other: Vec<T> = other
        .elements
        .clone()
        .into_iter()
        .filter(|element| !other.elements.contains(element))
        .collect();

        difference.append(&mut only_other);
        difference.sort();

    CustomSet {
        elements: difference
    }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut union: Vec<T> = self.elements.to_vec();
        union.extend(other.elements.iter().cloned());
        union.sort();
        union.dedup();
        Self {
            elements: union
        }
    }
}
