use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct OTree {
    root: OTreeNodes,
}
#[derive(Clone, Debug)]
pub struct OTreeNodes {
    childrens: Option<[Option<Box<OTreeNodes>>; 8]>,
    value: Vec<u8>,
    is_leaf: bool,
    ref_count: u32,
}
#[derive(Clone, Debug)]
struct BriefNode {
    count: u32,
    // weight: u8,
    node: Vec<u8>,
    brief_node: Vec<u8>,
}

impl OTree {
    pub fn new() -> OTree {
        return OTree {
            root: OTreeNodes {
                childrens: None,
                value: vec![],
                is_leaf: false,
                ref_count: 0,
            },
        };
    }

    pub fn insert(&mut self, vindex: &[u8; 8]) {
        let mut cur_node = &mut self.root;
        // 不确定是否放这里，因为或许插入失败
        cur_node.ref_count += 1;

        for i in 0..8 {
            let iusize = vindex[i] as usize;

            if cur_node.childrens.is_none() {
                cur_node.childrens = Some([None, None, None, None, None, None, None, None]);
            }
            if cur_node.childrens.as_ref().unwrap()[iusize].is_some() {
                cur_node.childrens.as_mut().unwrap()[iusize]
                    .as_mut()
                    .unwrap()
                    .ref_count += 1;
            } else {
                cur_node.childrens.as_mut().unwrap()[iusize] = Some(Box::new(OTreeNodes {
                    childrens: None,
                    value: vindex[0..i + 1].to_vec(),
                    is_leaf: if i == 7 { true } else { false },
                    ref_count: 1,
                }));
            }
            cur_node = cur_node.childrens.as_mut().unwrap()[iusize]
                .as_mut()
                .unwrap();
        }
    }
    pub fn brief(self) {
        let mut stack: Vec<&OTreeNodes> = vec![];
        let mut leaf_list: Vec<BriefNode> = vec![];

        stack.push(&self.root);
        // 递归遍历八叉树，获取所有叶子节点
        while !stack.is_empty() {
            let cur_node = stack.pop().unwrap();
            if cur_node.is_leaf {
                leaf_list.push(BriefNode {
                    node: cur_node.value.clone(),
                    brief_node: vec![],
                    count: cur_node.ref_count,
                });
                continue;
            }
            for node in cur_node.childrens.as_ref().unwrap().iter() {
                match node {
                    Some(x) => {
                        stack.push(x);
                    }
                    None => {}
                }
            }
        }
        let o_tree = brief_tree(leaf_list, 2, 7);
    }
}

// 递归简化 leaf_list，直到 leaf_list.len <= max_node_count
// todo 优化合并节点的权重
// todo 优化合并节点时，及时退出，现在一定会执行完一轮
fn brief_tree(
    leaf_list: Vec<BriefNode>,
    max_node_count: usize,
    tree_deep: usize,
) -> Vec<BriefNode> {
    let mut node_list: HashMap<Vec<u8>, (Vec<u8>, u32, Vec<u8>)> = HashMap::new();
    let mut brief_leaf_list: Vec<BriefNode> = vec![];
    let mut len = leaf_list.len();

    if (len <= max_node_count) {
        return leaf_list;
    }

    for BriefNode {
        mut node,
        brief_node,
        count,
    } in leaf_list
    {
        let mut key: Vec<u8> = node.drain(..node.len() - 1).collect();
        if let Some(x) = node_list.get_mut(&key) {
            for _ in 0..count {
                x.0.append(&mut node);
            }
            x.1 = x.1 + count;
            len = len - 1;
        } else {
            let mut _v: Vec<u8> = vec![];
            for _ in 0..count {
                _v.append(&mut node);
            }
            node_list.insert(key, (_v, count, brief_node));
        }
    }
    // println!("node_list:{:?}", node_list);
    for (key, mut value) in node_list.into_iter() {
        let mut max_v: f32 = 0.0;
        let mut avg_v: u8 = 0;
        for _v in &value.0 {
            max_v += *_v as f32;
        }
        avg_v = (max_v / value.1 as f32).round() as u8;
        value.2.push(avg_v);
        brief_leaf_list.push(BriefNode {
            node: key,
            brief_node: value.2,
            count: value.1,
        });
    }
    // println!("brief_leaf_list:{:?}", brief_leaf_list);
    return brief_tree(brief_leaf_list, max_node_count, tree_deep);
}
