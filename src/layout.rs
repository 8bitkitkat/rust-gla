// pub fn enable_layout(index: u32, layout: &Layout) {
//     // gl::vertex_attrib_pointer(index, size, ty, normalized, stride, offset)
//     for
// }

pub struct Layout {
    stride: usize,
    members: Vec<LayoutMember>,
}

impl Layout {
    pub fn new(stride: usize, members: Vec<LayoutMember>) -> Self {
        Self { stride, members }
    }

    pub fn build<V, F: FnOnce(&mut LayoutMemberAccumulator<V>, &V)>(vertex: &V, f: F) -> Self {
        let mut accumulator = LayoutMemberAccumulator::new(vertex);
        (f)(&mut accumulator, vertex);
        Self::new(std::mem::size_of::<V>(), accumulator.members)
    }

    pub fn enable(&self) {
        for (i, mem) in self.members.iter().enumerate() {
            gl::vertex_attrib_pointer(
                i as u32,
                mem.count as i32,
                mem.ty,
                false,
                self.stride as i32,
                mem.offset as isize,
            );
            gl::enable_vertex_attrib_array(i as u32);
        }
    }

    // pub fn disable(&self) {
    //     for i in self.members.len() - 1 {
    //         gl::dis(i as u32);
    //     }
    // }
}

pub struct LayoutMemberAccumulator<'a, V> {
    members: Vec<LayoutMember>,
    vertex: &'a V,
}

impl<'a, V> LayoutMemberAccumulator<'a, V> {
    fn new(vertex: &'a V) -> Self {
        Self {
            members: Vec::new(),
            vertex,
        }
    }

    pub fn add<M>(&mut self, member: &M, ty: gl::Type, count: u32) {
        self.members
            .push(LayoutMember::new(self.vertex, member, ty, count))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct LayoutMember {
    offset: usize,

    ty: gl::Type, // eg; the field `pos: [f32; 3]` = ty: Type::Float, count: 3
    count: u32,
}

impl LayoutMember {
    pub fn new<V, M>(vertex: &V, member: &M, ty: gl::Type, count: u32) -> Self {
        let v_size = std::mem::size_of::<V>();
        let m_size = std::mem::size_of::<M>();
        let v_addr = vertex as *const _ as usize;
        let m_addr = member as *const _ as usize;

        // check that member is actually a member of vertex
        assert!(m_addr >= v_addr);
        assert!((m_addr + m_size) <= (v_addr + v_size));

        let format_size = ty.size() * count as usize;
        assert_eq!(
            m_size,
            format_size,
            "The type and count information does not match the size of the given member: ty: {:?}, count: {} size: {}, member size: {}",
            ty,
            count,
            format_size,
            m_size,
        );

        let offset = m_addr - v_addr;

        Self { offset, ty, count }
    }

    // pub fn size(&self) -> usize {
    //     self.ty.size() * self.count as usize
    // }
}
