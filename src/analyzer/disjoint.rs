use std::collections::HashSet;

/// https://en.wikipedia.org/wiki/Disjoint-set_data_structure#Applications
pub struct DisjointSet<T>
where
    T: PartialEq,
{
    entities: Vec<Entity<T>>,
}

impl<T> DisjointSet<T>
where
    T: PartialEq,
{
    pub fn new() -> DisjointSet<T> {
        DisjointSet {
            entities: Vec::new(),
        }
    }
    pub fn data(&self, ptr: EntityPtr) -> &T {
        self.entities
            .get(ptr.ptr)
            .map(|e| &e.data)
            .expect("the entity behind the pointer is absent")
    }
    pub fn make_set(&mut self, data: T) -> EntityPtr {
        self.entities
            .iter()
            .enumerate()
            .find(|(_, e)| e.data.eq(&data))
            .map(|(ptr, _)| EntityPtr { ptr })
            .unwrap_or(EntityPtr {
                ptr: self.insert(data),
            })
    }

    pub fn find(&mut self, ptr: EntityPtr) -> EntityPtr {
        let mut curr = ptr.ptr;
        let mut parent = self.parent(curr);
        while curr != parent {
            curr = self.compress(curr, self.parent(parent));
            parent = self.parent(curr);
        }
        EntityPtr { ptr: curr }
    }

    pub fn union(&mut self, lhs: EntityPtr, rhs: EntityPtr) {
        let l_ptr = self.find(lhs).ptr;
        let r_ptr = self.find(rhs).ptr;
        if l_ptr != r_ptr {
            if self.rank(l_ptr) > self.rank(r_ptr) {
                self.set_parent(r_ptr, l_ptr);
            } else {
                self.set_parent(l_ptr, r_ptr);
            }
        }
    }

    fn get_mut(&mut self, idx: usize) -> &mut Entity<T> {
        self.entities
            .get_mut(idx)
            .expect("the data behind the pointer is absent")
    }
    fn rank(&self, idx: usize) -> usize {
        self.entities
            .get(idx)
            .map(|e| e.rank)
            .expect("the data behind the pointer is absent")
    }
    fn parent(&self, idx: usize) -> usize {
        self.entities
            .get(idx)
            .map(|e| e.parent)
            .expect("the data behind the pointer is absent")
    }

    fn compress(&mut self, idx: usize, parent: usize) -> usize {
        let curr = self.get_mut(idx);
        curr.parent = parent;
        curr.parent
    }
    fn increase_rank(&mut self, idx: usize) {
        let mut entity = self.get_mut(idx);
        entity.rank = entity.rank + 1;
    }
    fn set_parent(&mut self, idx: usize, new_parent: usize) {
        let mut entity = self.get_mut(idx);
        entity.parent = new_parent;
        if self.rank(idx) == self.rank(new_parent) {
            self.increase_rank(new_parent)
        }
    }
    fn insert(&mut self, data: T) -> usize {
        let current_len = self.entities.len();
        self.entities.push(Entity::new(data, 0, current_len));
        current_len
    }
}

#[derive(Clone, Copy)]
pub struct EntityPtr {
    pub ptr: usize,
}

#[derive(Debug)]
struct Entity<T>
where
    T: PartialEq,
{
    data: T,
    rank: usize,
    parent: usize,
}

impl<T> Entity<T>
where
    T: PartialEq,
{
    fn new(data: T, rank: usize, parent: usize) -> Self {
        Self { data, rank, parent }
    }
}

#[cfg(test)]
mod tests {
    use super::{DisjointSet, Entity};

    #[test]
    fn find_work() {
        let mut set = DisjointSet {
            entities: vec![
                Entity {
                    data: 0,
                    rank: 0,
                    parent: 0,
                },
                Entity {
                    data: 0,
                    rank: 0,
                    parent: 0,
                },
                Entity {
                    data: 0,
                    rank: 0,
                    parent: 1,
                },
                Entity {
                    data: 0,
                    rank: 0,
                    parent: 2,
                },
                Entity {
                    data: 0,
                    rank: 0,
                    parent: 3,
                },
                Entity {
                    data: 0,
                    rank: 0,
                    parent: 4,
                },
            ],
        };

        let res = set.find(super::EntityPtr { ptr: 5 });
        assert_eq!(res.ptr, 0);
        let res: Vec<_> = set.entities.iter().map(|e| e.parent).collect();
        assert_eq!(res, [0, 0, 1, 1, 3, 3]);

        let res = set.find(super::EntityPtr { ptr: 5 });
        assert_eq!(res.ptr, 0);
        let res: Vec<_> = set.entities.iter().map(|e| e.parent).collect();
        assert_eq!(res, [0, 0, 1, 1, 3, 1]);
    }

    #[test]
    fn union_work() {
        let mut set: DisjointSet<usize> = DisjointSet::new();
        let p1 = set.make_set(1);
        let p2 = set.make_set(2);
        let p3 = set.make_set(3);
        let p4 = set.make_set(4);
        let p5 = set.make_set(5);

        for e in set.entities.iter() {
            assert_eq!(e.rank, 0);
            assert_eq!(e.data, e.parent + 1)
        }
        set.union(p1, p2);

        let res: Vec<_> = set.entities.iter().map(|e| e.parent).collect();
        assert_eq!(res, [1, 1, 2, 3, 4]);

        set.union(p3, p4);
        set.union(p4, p5);
        set.union(p2, p5);

        let res: Vec<_> = set.entities.iter().map(|e| e.parent).collect();
        assert_eq!(res, [1, 3, 3, 3, 3]);

        let res: Vec<_> = set.entities.iter().map(|e| e.rank).collect();
        assert_eq!(res, [0, 1, 0, 2, 0]);
    }
}
