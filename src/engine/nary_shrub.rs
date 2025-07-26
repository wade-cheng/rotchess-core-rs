use std::str::FromStr;

pub enum NaryShrub<T> {
    Branch(Vec<NaryShrub<T>>),
    Leaf(T),
}

impl<T> FromStr for NaryShrub<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level: usize = 0;
        for line in s.lines() {
            if let Some('-') = line.chars().next() {
                if level != 0 {
                    return Err("Start with a single hyphen.".into());
                }
                continue;
            }
            let new_level = line
                .find(|c| !char::is_whitespace(c))
                .ok_or("Line must have non-whitespace.")?;
            if new_level == 0 {
                return Err("".into());
            }
            todo!()
        }
        Ok(todo!())
    }
}

//         let text = "
// -
//   -
//     1
//     -
//       8
//       9
//   -
//     3
//     4
// ";

impl<T: Clone> NaryShrub<T> {
    /// Return a vec with a left-to-right traversal of this shrub.
    pub fn as_vec(&self) -> Vec<T> {
        let mut ans = vec![];
        self.extend_into_vec(&mut ans);
        ans
    }

    /// Extend a vec with a left-to-right traversal of this shrub.
    pub fn extend_into_vec(&self, dest: &mut Vec<T>) {
        match self {
            NaryShrub::Branch(nary_shrubs) => {
                for shrub in nary_shrubs {
                    shrub.extend_into_vec(dest);
                }
            }
            NaryShrub::Leaf(val) => dest.push(val.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn simple() {
        let shrub = NaryShrub::Branch(vec![
            NaryShrub::Branch(vec![
                NaryShrub::Leaf(1),
                NaryShrub::Branch(vec![NaryShrub::Leaf(8), NaryShrub::Leaf(9)]),
            ]),
            NaryShrub::Branch(vec![NaryShrub::Leaf(3), NaryShrub::Leaf(4)]),
        ]);

        let text = "
-
  -
    1
    -
      8
      9
  -
    3
    4
";

        assert_eq!(
            shrub.as_vec(),
            NaryShrub::from_str(text).expect("hardcoded.").as_vec()
        )
    }
}
