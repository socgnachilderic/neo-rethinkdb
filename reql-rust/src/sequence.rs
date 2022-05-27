/* use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait ReqlDocumentOps: Clone + Serialize + Eq + PartialEq + Ord + PartialOrd {

}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Document<T: ReqlDocumentOps>(T);


#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Sequence<T: ReqlDocumentOps> {
    data: Vec<Document<T>>,
    counter: usize,
}

impl<T: ReqlDocumentOps> Sequence<T> {
    pub fn new() -> Self {
        Self { data: Vec::new(), counter: 0 }
    }

    pub fn add(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

impl<T: ReqlDocumentOps> Iterator for Sequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.get(self.counter) {
            Some(value) => {
                self.counter += 1;
                Some(value.clone())
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sequence;

    #[test]
    fn test_sequence() {
        let arr = [1, 2, 3];
        let mut list: Sequence<u8> = Sequence::new();

        for num in arr {
            list.add(num);
        }

        for (index, value) in list.enumerate() {
            // if let Some(value) = list.get(index) {
                
            // }
            assert_eq!(arr[index], value);
        }
    }

} */