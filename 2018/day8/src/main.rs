fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .trim()
        .split(' ')
        .map(|val| val.parse())
        .collect::<Result<Vec<usize>, _>>()?;
    let root = Node::from_arr(&input);
    dbg!(root.sum_all_metadata());
    dbg!(root.value());
    Ok(())
}

#[derive(Debug, Default)]
struct Node {
    metadata: Vec<usize>,
    children: Vec<Node>,
    length: usize,
}

impl Node {
    fn from_arr(arr: &[usize]) -> Self {
        let (children_count, meta_count) = (arr[0], arr[1]);
        let mut node = Node {
            length: 2,
            ..Node::default()
        };

        for _ in 0..children_count {
            let child = Node::from_arr(&arr[node.length..]);
            node.length += child.length;
            node.children.push(child);
        }

        for _ in 0..meta_count {
            node.metadata.push(arr[node.length]);
            node.length += 1;
        }
        node
    }

    fn sum_all_metadata(&self) -> usize {
        let mut sum = self.metadata.iter().cloned().sum();
        for child in &self.children {
            sum += child.sum_all_metadata();
        }
        sum
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            return self.metadata.iter().cloned().sum();
        }

        let mut sum = 0;
        for i in self.metadata.iter() {
            if let Some(child) = self.children.get(i - 1) {
                sum += child.value();
            }
        }
        sum
    }
}
